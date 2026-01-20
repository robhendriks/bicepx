use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::{debug, error, info};
use tokio::sync::Semaphore;

use crate::{az::AzCli, bicep::BicepModule};

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

    info!("Using working dir: {}", cli.working_dir.display());
    info!("Using config: {}", config_path.display());

    match &cli.command {
        Commands::Build => {
            let az_version = AzCli::get_version().await?;
            let az_bicep_version = AzCli::get_bicep_version().await?;

            info!("Using az cli: {}", az_version.cli);
            info!("Using az bicep: {}", az_bicep_version);

            let root = config::Root::load_from_file(&config_path)?;

            let mod_paths =
                BicepModule::discover_module_paths(cli.working_dir, root.modules.entrypoint)?;

            let semaphore = Arc::new(Semaphore::new(4));

            let tasks: Vec<_> = mod_paths
                .iter()
                .map(|mod_path| {
                    let mod_path = mod_path.clone();
                    let semaphore = Arc::clone(&semaphore);

                    tokio::spawn(async move {
                        let _permit = semaphore.acquire().await.unwrap();

                        info!("Building module: {}", mod_path.display());

                        match AzCli::exec_bicep_build(&mod_path).await {
                            Ok(_) => {
                                println!("COMPILE_OK {}", mod_path.display())
                            }
                            Err(err) => error!("{:#}", err),
                        }
                    })
                })
                .collect();

            for task in tasks {
                task.await?;
            }
        }
    }

    debug!("Done");

    Ok(())
}
