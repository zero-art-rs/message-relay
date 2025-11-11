mod cli;
mod config;
mod dbwatcher;
mod publisher;
mod types;
mod api;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::run().await
}
