use std::sync::Arc;
use tokio::select;
use tokio::signal::unix;
use tokio::signal::unix::SignalKind;
use tracing::info;

use crate::{
    cli::{arguments, logging},
    config::MessageRelayConfig,
};

pub async fn run(args: arguments::Run) -> eyre::Result<()> {
    let config = MessageRelayConfig::from_path(args.config)?;

    logging::init(config.logging.level)?;

    tokio::spawn(async move {
        info!("I am message relay!");
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    });

    let mut sigterm =
        unix::signal(SignalKind::terminate()).expect("Failed to create SIGTERM signal handler");
    let mut sigint =
        unix::signal(SignalKind::interrupt()).expect("Failed to create SIGINT signal handler");

    select! {
        // _ = node.cancelled() => {
        //     tracing::info!("Node run failed");
        // }
        _ = sigterm.recv() => {
            tracing::info!("Received SIGTERM signal");
        }
        _ = sigint.recv() => {
            tracing::info!("Received SIGINT signal");
        }
    }

    // node.shutdown().await;

    Ok(())
}
