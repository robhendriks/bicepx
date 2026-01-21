use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::{debug, info};

use crate::{az::AzCli, bicep::BicepProject};

mod az;
mod bicep;
mod config;

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
        default_value = "bicepx.toml",
        env = "BICEPX_CONFIG_FILE"
    )]
    config_file: PathBuf,
}

impl Cli {
    fn get_config_path(&self) -> PathBuf {
        return if self.config_file.is_absolute() {
            self.config_file.clone()
        } else {
            self.working_dir.join(&self.config_file)
        };
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Build,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    let config_path = cli.get_config_path();

    info!("Working dir: {}", cli.working_dir.display());
    info!("Config: {}", config_path.display());

    match &cli.command {
        Commands::Build => {
            let az_version = AzCli::get_version().await?;
            let az_bicep_version = AzCli::get_bicep_version().await?;

            info!("az cli version: {}", az_version.cli);
            info!("az bicep version: {}", az_bicep_version);

            let root = config::Root::load_from_file(&config_path)?;

            let mut bicep_project = BicepProject::new(cli.working_dir);

            debug!(
                "Looking for modules: **/{}",
                root.modules.entrypoint.display()
            );

            let module_count = bicep_project.discover_modules(root.modules.entrypoint)?;

            info!("Found {} Bicep module(s)", module_count);

            bicep_project.compile_modules().await?;

            // println!("{:?}", bicep_project);
        }
    }

    debug!("Done");

    Ok(())
}
