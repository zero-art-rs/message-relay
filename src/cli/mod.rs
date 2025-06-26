mod actions;
mod arguments;
mod logging;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub enum Cli {
    Run(arguments::Run),
}

impl Cli {
    pub async fn exec(self) -> eyre::Result<()> {
        match self {
            Self::Run(args) => actions::run(args).await,
        }
    }
}

pub async fn run() -> eyre::Result<()> {
    Cli::parse().exec().await
}
