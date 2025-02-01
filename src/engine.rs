use crate::error::{ConfigError, Result};
use crate::source::ConfigSource;
use futures::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

/// The main configuration engine that manages loading and accessing configuration.
pub struct ConfigEngine<T> {
    /// List of configuration sources in order of priority (last one wins)
    pub(crate) sources: Vec<Box<dyn ConfigSource>>,
    /// Current configuration state
    pub(crate) current_config: Arc<RwLock<T>>,
}

impl<T> ConfigEngine<T>
where
    T: DeserializeOwned + Send + Sync + 'static,
{
    /// Create a new configuration engine with an initial configuration
    pub fn new(initial_config: T) -> Self {
        Self {
            sources: Vec::new(),
            current_config: Arc::new(RwLock::new(initial_config)),
        }
    }

    /// Add a configuration source
    pub fn add_source<S>(&mut self, source: S) -> &mut Self
    where
        S: ConfigSource + 'static,
    {
        self.sources.push(Box::new(source));
        self
    }

    /// Load configuration from all sources
    pub async fn load(&mut self) -> Result<()> {
        let mut merged_value = serde_json::Value::Object(serde_json::Map::new());

        // Load and merge all sources in order
        for source in &self.sources {
            let source_value = source.load().await?;
            merge_json(&mut merged_value, &source_value);
        }

        // Convert merged JSON to the target type
        let new_config: T = serde_json::from_value(merged_value)
            .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;

        // Update current configuration
        let mut current = self.current_config.write().await;
        *current = new_config;

        Ok(())
    }

    /// Get a reference to the current configuration
    pub fn get(&self) -> Arc<RwLock<T>> {
        self.current_config.clone()
    }

    /// Get a clone of the current configuration
    pub async fn get_current(&self) -> T
    where
        T: Clone,
    {
        self.current_config.read().await.clone()
    }

    /// Watch for configuration changes.
    ///
    /// This spawns a background task which sends an updated clone of the configuration
    /// every 5 seconds.
    pub async fn watch(&self) -> impl Stream<Item = T>
    where
        T: Clone,
    {
        let (tx, rx) = mpsc::channel(16);
        let config = self.current_config.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                let current = config.read().await.clone();
                if tx.send(current).await.is_err() {
                    break;
                }
            }
        });

        futures::stream::unfold(rx, |mut rx| async move {
            rx.recv().await.map(|value| (value, rx))
        })
    }
}

/// Helper function to merge JSON values
fn merge_json(target: &mut serde_json::Value, source: &serde_json::Value) {
    use serde_json::Value;

    match (&mut *target, source) {
        (Value::Object(ref mut target_map), Value::Object(source_map)) => {
            for (key, value) in source_map {
                match target_map.get_mut(key) {
                    Some(target_value) => merge_json(target_value, value),
                    None => {
                        target_map.insert(key.clone(), value.clone());
                    }
                }
            }
        }
        (target_value, source_value) => {
            *target_value = source_value.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone, PartialEq)]
    struct TestConfig {
        name: String,
        port: u16,
    }

    #[tokio::test]
    async fn test_basic_config_loading() {
        let initial_config = TestConfig {
            name: "test".to_string(),
            port: 8080,
        };

        let engine = ConfigEngine::new(initial_config);
        let config = engine.get_current().await;

        assert_eq!(config.name, "test");
        assert_eq!(config.port, 8080);
    }

    #[tokio::test]
    async fn test_watch_config() {
        let initial_config = TestConfig {
            name: "test".to_string(),
            port: 8080,
        };

        let engine = ConfigEngine::new(initial_config);
        let mut watch_stream = engine.watch().await;

        // Pin the stream so that it is Unpin.
        let mut pinned_stream = Box::pin(watch_stream);

        if let Some(config) = pinned_stream.next().await {
            assert_eq!(config.name, "test");
            assert_eq!(config.port, 8080);
        }
    }
}
