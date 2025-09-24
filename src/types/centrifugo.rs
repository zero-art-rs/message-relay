use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CentrifugoMethod {
    #[serde(rename = "publish")]
    Publish,
    #[serde(rename = "broadcast")]
    Broadcast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub enum CentrifugoEventType {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "user_added")]
    UserAdded,
    #[serde(rename = "user_removed")]
    UserRemoved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentrifugoPayload {
    #[serde(rename = "channels")]
    pub channels: Vec<String>,
    #[serde(rename = "event_type")]
    pub event_type: CentrifugoEventType,
    #[serde(rename = "data")]
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentrifugoMessage {
    #[serde(rename = "method")]
    pub method: CentrifugoMethod,
    #[serde(rename = "payload")]
    pub payload: CentrifugoPayload,
}
