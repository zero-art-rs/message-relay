mod cli;
mod config;
mod dbwatcher;
mod publisher;
mod types;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::run().await
}
