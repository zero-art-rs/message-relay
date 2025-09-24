use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    publisher::Publisher,
    types::{CentrifugoEventType, CentrifugoMessage, CentrifugoMethod, CentrifugoPayload},
};

#[async_trait]
pub trait ARTChangesPublisher: Publisher {
    async fn publish_art_change(
        &self,
        chat_id: Uuid,
        data: serde_json::Value,
        namespace: String,
        subject: String,
    ) -> Result<(), Self::Error> {
        let centrifugo_message = CentrifugoMessage {
            method: CentrifugoMethod::Broadcast,
            payload: CentrifugoPayload {
                channels: vec![format!("{namespace}:{chat_id}")],
                event_type: CentrifugoEventType::ARTChange,
                data: "".to_string(),
            },
        };

        let serialized_message = serde_json::to_vec(&centrifugo_message)
            .map_err(|_| panic!("Failed to serialize centrifugo message to bytes"))?;

        let subject = format!("{subject}.{namespace}.{chat_id}");
        self.publish(subject, serialized_message).await
    }
}
