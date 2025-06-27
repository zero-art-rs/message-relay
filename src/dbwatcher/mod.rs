use mongodb::{Collection, bson::doc, change_stream::event::OperationType};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::{
    publisher::MessagePublisher,
    types::{GroupOperationOutbox, MessageOutbox},
};
use futures_util::StreamExt;

#[derive(Debug, Clone)]
pub struct DatabaseWatcher<P> {
    publisher: P,
    messages_outbox_collection: Collection<MessageOutbox>,
    group_operations_outbox_collection: Collection<GroupOperationOutbox>,
}

impl<P> DatabaseWatcher<P> {
    pub fn new(
        publisher: P,
        messages_outbox_collection: Collection<MessageOutbox>,
        group_operations_outbox_collection: Collection<GroupOperationOutbox>,
    ) -> Self {
        Self {
            publisher,
            messages_outbox_collection,
            group_operations_outbox_collection,
        }
    }
}

impl<P> DatabaseWatcher<P>
where
    P: MessagePublisher + Send + Sync + 'static,
{
    pub async fn run(self, cancel_token: CancellationToken) -> Result<(), eyre::Error> {
        let mut messages_stream = self.messages_outbox_collection.watch().await?;
        let mut group_operations_stream = self.group_operations_outbox_collection.watch().await?;

        info!("Starting database watcher");

        loop {
            tokio::select! {
                message_opt = messages_stream.next() => {
                    if let Some(Ok(message)) = message_opt {
                        info!("Got a new message outbox event: {:?}", message);

                        if !matches!(message.operation_type, OperationType::Insert) {
                            continue;
                        }

                        info!("Got a new message outbox insert event: {:?}", message);

                        let Some(message) = message.full_document else {
                            continue;
                        };

                        info!("Got a new message: {:?}", message);

                        self.handle_new_message(message).await?;
                    }
                }
                _group_operation = group_operations_stream.next() => {
                    warn!("Got a group operation, but group operations are not supported yet");
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
        self.publisher.publish_message(message.clone()).await?;

        let filter = doc! { "_id": message.chat_id.to_string() };

        self.messages_outbox_collection.delete_one(filter).await?;

        info!("Deleted message from outbox");

        Ok(())
    }
}
