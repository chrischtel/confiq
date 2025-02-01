# confiq

confiq is a modern, type-safe configuration management library for Rust. It provides a simple API to load, merge, and watch configuration data from multiple sources (YAML files and environment variables). The library uses Tokio for asynchronous I/O and supports custom configuration sources.

## Features

- **Type-Safe Configuration**
  Uses Serde to deserialize configuration into Rust types.

- **Multiple Sources**
  Combine configuration values from files (YAML format by default) and environment variables.

- **Configuration Merging**
  Merge values from multiple sources—later sources override earlier settings.

- **Asynchronous I/O**
  Utilizes Tokio for non-blocking, asynchronous file operations.

- **Watch for Changes**
  Provides a stream-based watch mechanism that periodically yields the current configuration.

## Installation

Add `confiq` to your `Cargo.toml`:

```toml
[dependencies]
confiq = "0.1.0"
```

Ensure you also have a Tokio runtime configured in your project. See [Tokio documentation](https://tokio.rs) for details.

## Dependencies and Features

Below is a summary of the main dependencies and activated features in the current MVP:

- **serde** with the `derive` feature for type-safe deserialization.
- **thiserror** for error handling.
- **serde_json** for merging and converting configuration values.
- **tokio** with features: `rt`, `sync`, `time`, `macros`, and `fs` (asynchronous file I/O).
- **async-trait** for async trait methods.
- **tokio-stream** and **futures** for streaming configuration updates.
- Optional: **serde_yaml** for parsing YAML configuration files.
- Basic sources:
  - **FileSource:** Reads configuration from a YAML file.
  - **EnvSource:** Reads configuration from environment variables.

## Usage

### Basic Example

Create a file named `config.yaml` with content similar to:

```yaml
database_url: "postgres://localhost/yourdb"
port: 8080
```

Set some environment variables to override or add to the configuration values:

```bash
export APP_DATABASE_URL="postgres://production/db"
export APP_PORT="9000"
```

Create an example executable (e.g., in `examples/basic.rs`):

```rust
use confiq::{ConfigEngine, Result};
use confiq::source::{FileSource, EnvSource};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
struct AppConfig {
    database_url: String,
    port: u16,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Initial default configuration.
    let initial = AppConfig {
        database_url: "postgres://localhost/db".to_string(),
        port: 8080,
    };

    // Create the configuration engine and add sources.
    let mut engine = ConfigEngine::new(initial);
    engine.add_source(FileSource::new(PathBuf::from("config.yaml")));
    engine.add_source(EnvSource::new("APP_".to_string()));

    // Load configuration from all sources.
    engine.load().await?;

    // Retrieve and print the current configuration.
    let config = engine.get_current().await;
    println!("Loaded configuration: {:?}", config);

    Ok(())
}
```

Build and run the example:

```bash
cargo run --example basic
```

### Using the Watch Function

The `watch` method returns a stream that periodically yields the current configuration. For example:

```rust
use futures::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let initial = AppConfig { /* ... */ };

    let engine = ConfigEngine::new(initial);
    // Add your sources...

    // Start the watch stream.
    let mut stream = Box::pin(engine.watch().await);

    // Process configuration updates.
    while let Some(config) = stream.next().await {
        println!("Configuration update: {:?}", config);
    }

    Ok(())
}
```

### Configuration Merging and Precedence

When multiple sources provide the same configuration key, the current implementation follows a simple rule: the **later source overwrites the earlier one**.

For example, suppose you add two sources:
- A YAML file that specifies `DATABASE_URL`
- Environment variables that contain `APP_DATABASE_URL`

If both sources define the same key (after prefix stripping for environment variables), the value provided by the environment variable (which is added later) will overwrite the value from the file. This merging behavior is:

- **Deterministic:** The sources are processed in the order they were added, with later sources taking precedence.
- **Predictable:** In your configuration management, you can control which source should override by adjusting the order of source registration.

If more granular control over merging strategies is needed, future versions of confiq could offer configurable merge strategies (e.g., merge arrays, error on conflicting keys, etc.).

## Tests

Basic tests are provided in the library and can be run with:

```bash
cargo test
```

Tests include:
- Basic configuration loading.
- The watch functionality that streams updates.

## Contributing

Contributions are welcome. Please feel free to open issues or submit pull requests on the [GitHub repository](https://github.com/chrischtel/confiq).

## License

confiq is licensed under the MIT OR Apache-2.0 license.

---

This README outlines the current MVP state of confiq, its features, configuration merging behavior, and how to use it. As additional features (such as remote sources, secret management, or hot reloading with file watchers) are implemented, the documentation will be updated accordingly.
