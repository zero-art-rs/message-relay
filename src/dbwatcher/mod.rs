use mongodb::{
    Collection,
    bson::{Binary, Bson, doc, spec::BinarySubtype},
    change_stream::event::OperationType,
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::{
    publisher::MessagePublisher,
    types::MessageOutbox,
    types::{Frame, SpFrame}
};
use futures_util::StreamExt;
use prost::Message;
use prost_types::Timestamp;

#[derive(Debug, Clone)]
pub struct DatabaseWatcher<P> {
    publisher: P,
    messages_outbox_collection: Collection<MessageOutbox>,

    messages_namespace: String,
    subject: String,
}

impl<P> DatabaseWatcher<P> {
    pub fn new(
        publisher: P,
        messages_outbox_collection: Collection<MessageOutbox>,
        messages_namespace: String,
        subject: String,
    ) -> Self {
        Self {
            publisher,
            messages_outbox_collection,
            messages_namespace,
            subject,
        }
    }
}

impl<P> DatabaseWatcher<P>
where
    P: MessagePublisher + Send + Sync + 'static,
{
    pub async fn run(self, cancel_token: CancellationToken) -> Result<(), eyre::Error> {
        info!("Handling history messages");

        self.handle_history_messages().await?;

        info!("History messages handled");

        let mut messages_stream = self.messages_outbox_collection.watch().await?;

        info!("Starting database watcher");

        loop {
            tokio::select! {
                message_opt = messages_stream.next() => {
                    let Some(message) = message_opt else {
                        info!("Messages stream closed");
                        break;
                    };

                    let message = match message {
                        Ok(message) => message,
                        Err(e) => {
                            error!("Error getting message from stream: {:?}", e);
                            continue;
                        }
                    };

                    info!("Got a new message outbox event: {:?}", message);

                    if !matches!(message.operation_type, OperationType::Insert) {
                        continue;
                    }

                    let Some(message) = message.full_document else {
                        error!("Message is not a full document");
                        continue;
                    };

                    self.handle_new_message(message).await?;
                }
                _ = cancel_token.cancelled() => {
                    info!("Token cancellation received");
                    break;
                }
            }
        }

        info!("Database watcher stopped");

        Ok(())
    }

    async fn handle_new_message(&self, message: MessageOutbox) -> Result<(), eyre::Error> {
        debug!("Handling new message: {}", message);

        let frame = Frame::decode(&*message.content)?;
        let sp_frame = SpFrame {
            seq_num: message.sequence_number as u64,
            created: Some(Timestamp {
                seconds: message.created_at.timestamp(),
                nanos: message.created_at.timestamp_subsec_nanos() as i32,
            }),
            frame: Some(frame),
        };

        self.publisher
            .publish_message(
                message.chat_id,
                sp_frame.encode_to_vec(),
                self.messages_namespace.clone(),
                self.subject.clone(),
            )
            .await?;

        let filter = doc! { "chat_id": Bson::Binary(Binary {
            subtype: BinarySubtype::Generic,
            bytes: message.chat_id.as_bytes().to_vec(),
        }), "sequence_number": message.sequence_number };

        debug!("Deleting message from outbox: {:?}", filter);

        self.messages_outbox_collection.delete_many(filter).await?;

        debug!("Deleted message from outbox");

        Ok(())
    }

    async fn handle_history_messages(&self) -> Result<(), eyre::Error> {
        let mut all_messages = self
            .messages_outbox_collection
            .find(doc! {})
            .sort(doc! { "created_at": -1 })
            .await?;

        while let Some(message) = all_messages.next().await {
            let message = match message {
                Ok(message) => message,
                Err(e) => {
                    error!("Error getting message from stream: {:?}", e);
                    continue;
                }
            };

            self.handle_new_message(message).await?;
        }

        Ok(())
    }
}
