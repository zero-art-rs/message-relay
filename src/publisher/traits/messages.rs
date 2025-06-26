use async_trait::async_trait;

use crate::{publisher::Publisher, types::Message};

#[async_trait]
pub trait MessagePublisher: Publisher {
    async fn publish_message(&self, message: Message) -> Result<(), Self::Error> {
        let subject = format!("chat.{}", message.chat_id);
        self.publish(subject, message.data).await
    }
}
