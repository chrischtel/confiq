mod engine;
mod error;
pub mod source;

pub use engine::ConfigEngine;
pub use error::{ConfigError, Result};

pub mod prelude {
    pub use crate::engine::ConfigEngine;
    pub use crate::error::{ConfigError, Result};
    pub use crate::source::ConfigSource;
}
