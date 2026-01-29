use anyhow::Result;
use clap::Args;

use crate::cli::Cli;

#[derive(Debug, Args)]
pub struct BuildArgs {}

pub async fn exec(_cli: &Cli, _args: &BuildArgs) -> Result<()> {
    Ok(())
}
