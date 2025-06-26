use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageConfig {
    /// MongoDB connection string
    pub database_url: String,

    /// MongoDB database name
    pub database_name: String,

    /// Outbox table name
    pub outbox_table_name: String,
}
