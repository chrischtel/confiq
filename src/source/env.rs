use super::ConfigSource;
use crate::error::{ConfigError, Result};
use async_trait::async_trait;
use serde_json::Value;
use std::env;

pub struct EnvSource {
    prefix: String,
}

impl EnvSource {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }
}

#[async_trait]
impl ConfigSource for EnvSource {
    async fn load(&self) -> Result<Value> {
        let mut map = serde_json::Map::new();
        for (key, value) in env::vars() {
            if key.starts_with(&self.prefix) {
                // Strip out the prefix and lowercase the key.
                let config_key = key[self.prefix.len()..].to_lowercase();
                map.insert(config_key, Value::String(value));
            }
        }
        Ok(Value::Object(map))
    }
}
