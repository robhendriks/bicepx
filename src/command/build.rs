use anyhow::{Context, Result};
use clap::Args;
use log::debug;

use crate::{az, cli::Cli};

#[derive(Debug, Args)]
pub struct BuildArgs {}

pub async fn exec(_cli: &Cli, _args: &BuildArgs) -> Result<()> {
    let bicep_cli = az::bicep::Cli::new().with_context(|| "Unable to locate Bicep CLI binary")?;
    let bicep_cli_version = bicep_cli.version().await?;

    debug!("{:?}", bicep_cli_version);

    Ok(())
}
