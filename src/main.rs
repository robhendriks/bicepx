use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;

mod cli;
mod command;
mod config;
mod console;
mod model;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder().format_timestamp(None).init();

    let cli = Cli::parse();

    cli.exec().await
}
