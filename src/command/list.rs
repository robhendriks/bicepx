use anyhow::Result;
use clap::{Args, Subcommand};
use log::info;

use crate::{cli::Cli, model};

#[derive(Debug, Args)]
pub struct ListArgs {
    #[command(subcommand)]
    command: ListCommands,
}

#[derive(Debug, Subcommand)]
enum ListCommands {
    #[command(alias = "mod")]
    Module,
}

pub async fn exec(_cli: &Cli, _args: &ListArgs) -> Result<()> {
    let prj = model::Project::load(&_cli.root).await?;

    match &_args.command {
        ListCommands::Module => {
            info!("yooo {:?}", prj);
        }
    }

    Ok(())
}
