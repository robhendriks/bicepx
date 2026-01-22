use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use log::{error, info};

use crate::{
    config::{Config, ModuleConfig, ProjectConfig},
    project::Project,
};

mod azure;
mod config;
mod project;

#[derive(Debug, Parser)]
#[command(name = "bicepx")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(
        short,
        long,
        global = true,
        default_value = ".",
        env = "BICEPX_WORKING_DIR"
    )]
    working_dir: PathBuf,

    #[arg(
        short,
        long,
        global = true,
        default_value = "bicepx.json",
        env = "BICEPX_CONFIG_FILE"
    )]
    config_file: PathBuf,
}

impl Cli {
    fn get_config_path(&self) -> PathBuf {
        if self.config_file.is_absolute() {
            self.config_file.clone()
        } else {
            self.working_dir.join(&self.config_file)
        }
    }
}

#[derive(Debug, Args)]
struct InitArgs {
    #[arg(short, long)]
    name: Option<String>,

    #[arg(long)]
    module_pattern: Option<PathBuf>,

    #[arg(short, long, default_value_t = false)]
    overwrite: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init(InitArgs),
    Format,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder().format_timestamp(None).init();

    let cli = Cli::parse();

    let prj_config_path = cli.get_config_path();

    match &cli.command {
        Commands::Init(args) => {
            info!("Initializing project config...");

            let prj_config = ProjectConfig::new();

            match prj_config.save_to(&prj_config_path, args.overwrite).await {
                Ok(_) => {
                    info!("Created '{}'", prj_config_path.display());
                }
                Err(err) => {
                    error!("{}", err)
                }
            }

            if let Some(init_modules) = &args.module_pattern {
                info!("Initializing module config...");

                let mut project = Project::new(&cli.working_dir);
                let _ = project
                    .discover_modules(init_modules)
                    .with_context(|| "Failed to discover modules")?;

                for module in project.modules {
                    let mod_config = ModuleConfig::new();
                    let mod_config_path = module.root_path.join("module.json");

                    match mod_config.save_to(&mod_config_path, args.overwrite).await {
                        Ok(_) => {
                            info!("Created '{}'", mod_config_path.display());
                        }
                        Err(err) => {
                            error!("{}", err)
                        }
                    }
                }
            }
        }
        Commands::Format => {
            azure::AzCli::init().await?;
            azure::BicepCli::init().await?;

            let _prj = ProjectConfig::load_from(prj_config_path).await?;

            info!("format");
        }
    }

    Ok(())
}
