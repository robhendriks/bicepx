use anyhow::Result;
use clap::Args;

use crate::cli::Cli;

#[derive(Debug, Args)]
pub struct ListArgs {}

pub async fn exec(_cli: &Cli, _args: &ListArgs) -> Result<()> {
    Ok(())
}
