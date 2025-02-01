pub mod env;
pub mod file;

use crate::error::Result;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ConfigSource: Send + Sync {
    /// Load configuration from this source
    async fn load(&self) -> Result<Value>;

    /// Whether this source supports hot reloading
    fn supports_hot_reload(&self) -> bool {
        false
    }
}
