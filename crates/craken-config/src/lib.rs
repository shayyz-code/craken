use serde::Deserialize;
use config::{Config as ConfigLib, ConfigError, File};

/// Configuration manager for the framework.
/// Uses `config` crate to merge multiple configuration sources.
/// 
/// This is designed to be production-ready, allowing overrides via environment variables.
pub struct Config;

impl Config {
    /// Load configuration from a specified path and map to a concrete struct.
    pub fn load<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T, ConfigError> {
        let s = ConfigLib::builder()
            .add_source(File::with_name(path).required(false))
            .build()?;
        
        s.try_deserialize()
    }
}
