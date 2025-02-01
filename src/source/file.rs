use super::ConfigSource;
use crate::error::{ConfigError, Result};
use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;

/// FileFormat is an enum that indicates whether to parse the file as YAML or TOML.
///
/// By default, YAML is used. If the "toml" feature is enabled, then TOML is available.
#[derive(Clone, Copy)]
pub enum FileFormat {
    Yaml,
    #[cfg(feature = "toml")]
    Toml,
}

impl Default for FileFormat {
    fn default() -> Self {
        FileFormat::Yaml
    }
}

/// FileSource now contains a file path and a file format.
/// The user can create a FileSource with the default YAML parser or explicitly select TOML (if enabled).
pub struct FileSource {
    path: PathBuf,
    format: FileFormat,
}

impl FileSource {
    /// Create a new FileSource using the given file path.
    /// By default, the file is assumed to be in YAML format.
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            format: FileFormat::default(),
        }
    }

    /// Set the file format, e.g. to FileFormat::Toml.
    /// (This is only available if the "tool" feature is enabled.)
    pub fn with_format(mut self, format: FileFormat) -> Self {
        self.format = format;
        self
    }
}

#[async_trait]
impl ConfigSource for FileSource {
    async fn load(&self) -> Result<Value> {
        // Read file contents asynchronously using Tokio's fs.
        let contents = tokio::fs::read_to_string(&self.path)
            .await
            .map_err(|e| ConfigError::IoError(e))?;
        // Process file contents based on the chosen format.
        match self.format {
            FileFormat::Yaml => {
                // Parse using YAML.
                let value: Value = serde_yaml::from_str(&contents)
                    .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
                // Optionally, convert the YAML (serde_yaml::Value) into a serde_json::Value.
                let json_str = serde_json::to_string(&value)
                    .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
                let json_val: Value = serde_json::from_str(&json_str)
                    .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
                Ok(json_val)
            }
            #[cfg(feature = "toml")]
            FileFormat::Toml => {
                // Parse using TOML.
                let value: toml::Value = toml::from_str(&contents)
                    .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
                // Convert toml::Value into serde_json::Value.
                let json_val: Value = serde_json::to_value(value)
                    .map_err(|e| ConfigError::DeserializationError(e.to_string()))?;
                Ok(json_val)
            }
        }
    }

    fn supports_hot_reload(&self) -> bool {
        false
    }
}
