// src/source/remote.rs
use super::ConfigSource;
use crate::error::Result;
use async_trait::async_trait;
use serde_json::Value;

pub struct RemoteSource {
    url: String,
}

impl RemoteSource {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

#[async_trait]
impl ConfigSource for RemoteSource {
    async fn load(&self) -> Result<Value> {
        // Implementation will come later
        todo!()
    }
}
