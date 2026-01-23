use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use log::{error, info};

use crate::{cli, config::Config, config::ModuleConfig, project::Project};

#[derive(Debug, Args)]
pub struct ModuleArgs {
    #[command(subcommand)]
    command: ModuleCommands,
}

#[derive(Debug, Subcommand)]
enum ModuleCommands {
    Init(InitArgs),
}

#[derive(Debug, Args)]
struct InitArgs {
    #[arg(long, short, default_value_t = false)]
    force: bool,
}

impl ModuleArgs {
    pub async fn execute(cli: &cli::Cli, args: &ModuleArgs) -> Result<()> {
        match &args.command {
            ModuleCommands::Init(args) => {
                info!("Initializing module config...");

                let mut project = Project::new(&cli.working_dir);
                let _ = project
                    .discover_modules("bicep/main.bicep")
                    .with_context(|| "Failed to discover modules")?;

                for module in project.modules {
                    let mod_config = ModuleConfig::new();
                    let mod_config_path = module.root_path.join("bicepx-module.json");

                    match mod_config.save_to(&mod_config_path, args.force).await {
                        Ok(_) => {
                            info!("Created '{}'", mod_config_path.display());
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
