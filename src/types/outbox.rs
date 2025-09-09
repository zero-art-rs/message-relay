use std::fmt::{write, Display};
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_with::{serde_as, base64::Base64};
use base64::prelude::*;

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
