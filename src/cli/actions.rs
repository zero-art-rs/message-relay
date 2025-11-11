use crate::api;
use mongodb::{Client, options::ClientOptions};
use tokio::select;
use tokio::signal::unix;
use tokio::signal::unix::SignalKind;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{error, info, warn};

use crate::{
    cli::{arguments, logging},
    config::MessageRelayConfig,
    dbwatcher::DatabaseWatcher,
    publisher::NatsPublisher,
};

pub async fn run(args: arguments::Run) -> eyre::Result<()> {
    let config = MessageRelayConfig::from_path(args.config)?;

    logging::init(config.logging.level)?;

    let nats_connection = async_nats::connect(config.nats.url).await?;
    let nats_publisher = NatsPublisher::from(nats_connection);

    let uri = config.storage.database_url.clone();
    let database_name = config.storage.database_name.clone();

    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;

    let database = client
        .default_database()
        .unwrap_or(client.database(&database_name));

    let messages_outbox_collection =
        database.collection(&config.storage.messages_outbox_collection_name);

    let db_watcher = DatabaseWatcher::new(
        nats_publisher,
        messages_outbox_collection,
        config.nats.messages_namespace.clone(),
        config.nats.subject.clone(),
    );

    let cancel_token = CancellationToken::new();
    let task_tracker = TaskTracker::new();

    let cancel_token_clone = cancel_token.clone();
    task_tracker.spawn(async move {
        if let Err(e) = db_watcher.run(cancel_token_clone.clone()).await {
            error!("Database watcher failed: {}", e);
            cancel_token_clone.cancel();
        }
    });

    task_tracker.spawn(api::run_server(
        config.api.address.to_string(),
        cancel_token.clone(),
    ));

    let mut sigterm =
        unix::signal(SignalKind::terminate()).expect("Failed to create SIGTERM signal handler");
    let mut sigint =
        unix::signal(SignalKind::interrupt()).expect("Failed to create SIGINT signal handler");

    select! {
        _ = cancel_token.cancelled() => {}
        _ = sigterm.recv() => {
            tracing::info!("Received SIGTERM signal");
        }
        _ = sigint.recv() => {
            tracing::info!("Received SIGINT signal");
        }
    }

    info!("Shutting down...");
    cancel_token.cancel();

    let mut sigterm =
        unix::signal(SignalKind::terminate()).expect("Failed to create SIGTERM signal handler");
    let mut sigint =
        unix::signal(SignalKind::interrupt()).expect("Failed to create SIGINT signal handler");

    select! {
        res = tokio::time::timeout(std::time::Duration::from_secs(10), task_tracker.wait()) => {
            if let Err(e) = res {
                warn!("Shutdown timeout reached: {}", e);
            } else {
                info!("Shutdown completed gracefully");
            }
        }
        _ = sigterm.recv() => {
            tracing::info!("Force shutdown");
        }
        _ = sigint.recv() => {
            tracing::info!("Force shutdown");
        }
    }

    Ok(())
}
