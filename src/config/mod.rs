use serde::Deserialize;

use crate::config::{centrifugo::CentrifugoConfig, logging::LoggerConfig, storage::StorageConfig};

mod centrifugo;
mod logging;
mod storage;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// Storage configuration
    pub storage: StorageConfig,

    /// Logging configuration
    pub logging: LoggerConfig,

    /// Centrifugo configuration
    pub centrifugo: CentrifugoConfig,
}
