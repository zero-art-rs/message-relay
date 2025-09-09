use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageConfig {
    /// MongoDB connection string
    pub database_url: String,

    /// MongoDB database name
    pub database_name: String,

    /// Messages outbox collection name
    pub messages_outbox_collection_name: String,
}
