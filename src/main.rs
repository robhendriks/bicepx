use anyhow::Result;
use clap::Parser;

mod azure;
mod cli;
mod commands;
mod config;
mod project;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder().format_timestamp(None).init();

    let cli = cli::Cli::parse();

    cli.execute().await
}
