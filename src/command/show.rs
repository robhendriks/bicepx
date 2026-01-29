use anyhow::Result;
use clap::Args;

use crate::cli::Cli;

#[derive(Debug, Args)]
pub struct ShowArgs {}

pub async fn exec(_cli: &Cli, _args: &ShowArgs) -> Result<()> {
    Ok(())
}
