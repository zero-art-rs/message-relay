use async_nats::subject::ToSubject;
use async_trait::async_trait;
use bytes::Bytes;

use crate::publisher::Publisher;

pub struct NatsPublisher {
    connection: async_nats::Client,
}

impl From<async_nats::Client> for NatsPublisher {
    fn from(connection: async_nats::Client) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl Publisher for NatsPublisher {
    type Error = async_nats::PublishError;

    async fn publish<S: ToSubject + Send, M: Into<Bytes> + Send>(
        &self,
        subject: S,
        message: M,
    ) -> Result<(), Self::Error> {
        self.connection.publish(subject, message.into()).await
    }
}
