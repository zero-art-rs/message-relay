use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageOutbox {
    /// The message content as binary data
    pub content: Vec<u8>, // binary string, Vec<u8>
    /// When the message was created
    pub created_at: DateTime,
    /// Sequential number of this message in the chat
    pub sequence_number: i64,
    /// Public key of the message sender
    pub sender_public_key: String,
    /// Unique identifier of the chat to send the message to.
    pub chat_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupOperationOutbox {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub data: Vec<u8>,
    pub created_at: DateTime,
}
