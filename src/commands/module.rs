use std::path::PathBuf;

use clap::{Args, Subcommand};
use semver::Version;
use serde::Serialize;

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

                let json_view_models: Vec<ModuleJson> = project
                    .modules
                    .iter()
                    .map(|m| ModuleJson {
                        path: &m.root,
                        version: &m.config.version,
                    })
                    .collect();

                println!("{}", serde_json::to_string_pretty(&json_view_models)?)
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

#[derive(Serialize)]
struct ModuleJson<'a> {
    #[serde(borrow)]
    path: &'a PathBuf,

    #[serde(borrow)]
    version: &'a Version,
}
