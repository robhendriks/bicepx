use clap::{Args, Subcommand};
use log::info;

use crate::cli::Cli;

#[derive(Debug, Args)]
pub struct ModuleArgs {
    #[command(subcommand)]
    command: ModuleCommands,
}

impl ModuleArgs {
    pub async fn exec(&self, _cli: &Cli) -> anyhow::Result<()> {
        match &self.command {
            ModuleCommands::List => {
                info!("MODULE LIST");
            }
        }

        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub enum ModuleCommands {
    List,
}
