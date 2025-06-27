use std::path::PathBuf;

use serde::Deserialize;

use crate::config::{logging::LoggerConfig, nats::NatsConfig, storage::StorageConfig};

mod logging;
mod nats;
mod storage;

#[derive(Debug, Deserialize)]
pub struct MessageRelayConfig {
    /// Storage configuration
    pub storage: StorageConfig,

    /// Logging configuration
    pub logging: LoggerConfig,

    /// NATS configuration
    pub nats: NatsConfig,
}

impl MessageRelayConfig {
    pub fn from_path(path: PathBuf) -> eyre::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::from(path))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}
