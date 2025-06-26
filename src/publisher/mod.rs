use async_nats::subject::ToSubject;
use async_trait::async_trait;
use bytes::Bytes;

mod impls;
mod traits;

#[async_trait]
pub trait Publisher {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn publish<S: ToSubject + Send, M: Into<Bytes> + Send>(
        &self,
        subject: S,
        message: M,
    ) -> Result<(), Self::Error>;
}
