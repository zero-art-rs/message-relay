use async_trait::async_trait;

use crate::{
    publisher::Publisher,
    types::{Message, MessageOutbox},
};

#[async_trait]
pub trait MessagePublisher: Publisher {
    async fn publish_message(&self, message: MessageOutbox) -> Result<(), Self::Error> {
        let subject = format!("chat.{}", message.chat_id);
        self.publish(subject, message.content).await
    }
}
