use confiq::source::{env::EnvSource, file::FileSource};
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
    // Create an initial config.
    let initial = AppConfig {
        database_url: "postgres://localhost/db".to_string(),
        port: 8080,
    };

    // Bind the engine to a variable and then add sources.
    let mut engine = ConfigEngine::new(initial);
    engine.add_source(FileSource::new(PathBuf::from("config.yaml")));
    engine.add_source(EnvSource::new("APP_".to_string()));

    // Load and merge configuration.
    engine.load().await?;

    // Retrieve the current configuration.
    let config = engine.get_current().await;
    println!("Loaded config: {:?}", config);

    Ok(())
}
