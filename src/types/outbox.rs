use base64::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};
use std::fmt::Display;
use uuid::Uuid;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageOutbox {
    /// The message content as binary data
    #[serde_as(as = "Base64")]
    pub content: Vec<u8>, // binary string, Vec<u8>
    /// When the message was created
    pub created_at: DateTime<Utc>,
    /// Sequential number of this message in the chat
    pub sequence_number: i64,
    /// Sequential number of epoch during which the message was sent
    pub epoch: i64,
    /// Unique identifier of the chat to send the message to.
    pub chat_id: Uuid,
}

impl Display for MessageOutbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ content: {}, ", BASE64_STANDARD.encode(&self.content))?;
        write!(f, "created_at: {}, ", self.created_at)?;
        write!(f, "sequence_number: {}, ", self.sequence_number)?;
        write!(f, "epoch: {}, ", self.epoch)?;
        write!(f, "chat_id: {} }}", self.chat_id)?;

        Ok(())
    }
}
