use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

mod cli;
mod command;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    cli.exec().await
}
