use super::ConfigSource;
use crate::error::{ConfigError, Result};
use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;

pub struct FileSource {
    path: PathBuf,
    // Optionally, you could also store a format (e.g., YAML or JSON)
}

impl FileSource {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

#[async_trait]
impl ConfigSource for FileSource {
    async fn load(&self) -> Result<Value> {
        // For an MVP, we assume YAML. You could easily extend this to JSON/TOML.
        let contents = tokio::fs::read_to_string(&self.path)
            .await
            .map_err(|e| ConfigError::IoError(e))?;
        let value: Value = serde_yaml::from_str(&contents)
            .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
        // Convert the YAML value (serde_yaml::Value) into serde_json::Value.
        let json_str = serde_json::to_string(&value)
            .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
        let json_val: Value = serde_json::from_str(&json_str)
            .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
        Ok(json_val)
    }

    fn supports_hot_reload(&self) -> bool {
        // For now, return false (we could integrate file watching later)
        false
    }
}
