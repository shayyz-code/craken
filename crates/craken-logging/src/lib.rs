use tracing_subscriber::{fmt, EnvFilter, prelude::*};

/// Structured logging for the framework.
/// Uses `tracing` and `tracing-subscriber` for asynchronous, structured logging.
pub struct Logging;

impl Logging {
    /// Initialize the global tracing subscriber.
    /// It automatically picks up logs from `tracing` macros across the app.
    pub fn init() {
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env())
            .init();
    }
}
