use confiq::source::{env::EnvSource, file::FileFormat, file::FileSource};
use confiq::{ConfigEngine, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
struct AppConfig {
    database_url: String,
    port: u16,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Define a default configuration.
    let initial = AppConfig {
        database_url: "postgres://localhost/db".to_string(),
        port: 8080,
    };

    // Create the configuration engine with the default configuration.
    let mut engine = ConfigEngine::new(initial);

    // Add a FileSource that will load configuration from a TOML file.
    // The `.with_format(FileFormat::Toml)` method is only available with the "toml" feature enabled.
    let file_source = FileSource::new(PathBuf::from("config.toml")).with_format(FileFormat::Toml);
    engine.add_source(file_source);

    // Optionally, add an environment source to override configuration from the file.
    // The environment variables should be prefixed appropriately (e.g., APP_).
    engine.add_source(EnvSource::new("APP_".to_string()));

    // Load and merge configuration from all sources.
    engine.load().await?;

    // Retrieve and print the merged configuration.
    let config = engine.get_current().await;
    println!("Loaded configuration (TOML): {:?}", config);

    Ok(())
}
