use clap::{Args, Subcommand};
use log::info;

use crate::{cli::Ctx, project::Project};

#[derive(Debug, Args)]
pub struct ModuleArgs {
    #[command(subcommand)]
    command: ModuleCommands,
}

impl ModuleArgs {
    pub async fn exec(&self, ctx: &Ctx) -> anyhow::Result<()> {
        match &self.command {
            ModuleCommands::List => {
                let mut project = Project::from_ctx(&ctx).await?;

                project.init().await?;

                for module in &project.modules {
                    info!("{} v{}", module.root.display(), module.config.version);
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub enum ModuleCommands {
    #[command(alias = "ls")]
    List,
}
