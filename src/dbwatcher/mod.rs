use mongodb::{
    Collection,
    bson::{Binary, Bson, doc, spec::BinarySubtype},
    change_stream::event::OperationType,
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::publisher::ARTChangesPublisher;
use crate::{
    publisher::MessagePublisher,
    types::{ARTChangeOutbox, MessageOutbox},
};
use futures_util::StreamExt;

#[derive(Debug, Clone)]
pub struct DatabaseWatcher<P> {
    publisher: P,
    messages_outbox_collection: Collection<MessageOutbox>,
    art_changes_outbox_collection: Collection<ARTChangeOutbox>,

    messages_namespace: String,
    art_changes_namespace: String,
    subject: String,
}

impl<P> DatabaseWatcher<P> {
    pub fn new(
        publisher: P,
        messages_outbox_collection: Collection<MessageOutbox>,
        art_changes_outbox_collection: Collection<ARTChangeOutbox>,
        messages_namespace: String,
        art_changes_namespace: String,
        subject: String,
    ) -> Self {
        Self {
            publisher,
            messages_outbox_collection,
            art_changes_outbox_collection,
            messages_namespace,
            art_changes_namespace,
            subject,
        }
    }
}

impl<P> DatabaseWatcher<P>
where
    P: MessagePublisher + ARTChangesPublisher + Send + Sync + 'static,
{
    pub async fn run(self, cancel_token: CancellationToken) -> Result<(), eyre::Error> {
        info!("Handling history messages");

        self.handle_history_messages().await?;
        self.handle_history_art_changes().await?;

        info!("History messages handled");

        let mut messages_stream = self.messages_outbox_collection.watch().await?;
        let mut art_changes_stream = self.art_changes_outbox_collection.watch().await?;

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
                art_change_opt = art_changes_stream.next() => {
                    let Some(art_change) = art_change_opt else {
                        info!("ART changes stream closed");
                        break;
                    };

                    let art_change = match art_change {
                        Ok(art_change) => art_change,
                        Err(e) => {
                            error!("Error getting ART change from stream: {:?}", e);
                            continue;
                        }
                    };

                    info!("Got a new ART changes outbox event: {:?}", art_change);

                    if !matches!(art_change.operation_type, OperationType::Insert) {
                        continue;
                    }

                    let Some(art_change) = art_change.full_document else {
                        error!("Art change is not a full document");
                        continue;
                    };

                    self.handle_new_art_change(art_change).await?;
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
        debug!("Handling new message: {:?}", message);

        self.publisher
            .publish_message(
                message.chat_id,
                serde_json::to_value(message.clone())?,
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

    async fn handle_new_art_change(&self, art_change: ARTChangeOutbox) -> Result<(), eyre::Error> {
        debug!("Handling new ART change: {:?}", art_change);

        self.publisher
            .publish_art_change(
                art_change.chat_id,
                serde_json::to_value(art_change.clone())?,
                self.art_changes_namespace.clone(),
                self.subject.clone(),
            )
            .await?;

        let filter = doc! { "chat_id": Bson::Binary(Binary {
            subtype: BinarySubtype::Generic,
            bytes: art_change.chat_id.as_bytes().to_vec(),
        }), "sequence_number": art_change.sequence_number };

        debug!("Deleting ART change from outbox: {:?}", filter);

        self.art_changes_outbox_collection
            .delete_many(filter)
            .await?;

        debug!("Deleted message from outbox");

        Ok(())
    }

    async fn handle_history_art_changes(&self) -> Result<(), eyre::Error> {
        let mut all_art_changes = self
            .art_changes_outbox_collection
            .find(doc! {})
            .sort(doc! { "created_at": -1 })
            .await?;

        while let Some(art_change) = all_art_changes.next().await {
            let art_change = match art_change {
                Ok(art_change) => art_change,
                Err(e) => {
                    error!("Error getting art_change from stream: {:?}", e);
                    continue;
                }
            };

            self.handle_new_art_change(art_change).await?;
        }

        Ok(())
    }
}
