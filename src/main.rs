use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::{debug, info};

use crate::bicep::BicepModule;

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

struct Context {
    config_path: PathBuf,
}

impl Context {
    fn from_cli(cli: &Cli) -> Context {
        Context {
            config_path: cli.get_config_path(),
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    let ctx = Context::from_cli(&cli);

    info!("Using config: {}", ctx.config_path.display());

    match &cli.command {
        Commands::Build => {
            let az_version = az::AzCli::get_version()?;
            let az_bicep_version = az::AzCli::get_bicep_version()?;

            info!("Using az cli: {}", az_version.cli);
            info!("Using az bicep: {}", az_bicep_version);

            let root = config::Root::load_from_file(&ctx.config_path)?;

            let modules = BicepModule::discover_module_paths(
                ctx.config_path.parent().unwrap(),
                root.modules.entrypoint,
            )?;

            println!("{:#?} {}", modules, modules.len())
        }
    }

    debug!("Done");

    Ok(())
}
