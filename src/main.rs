use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

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

// impl Cli {
//     fn get_config_path(&self) -> PathBuf {
//         return if self.config_file.is_absolute() {
//             self.config_file.clone()
//         } else {
//             self.working_dir.join(&self.config_file)
//         };
//     }
// }

#[derive(Debug, Subcommand)]
enum Commands {
    Build,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Build => {
            println!("Build")
        }
    }

    Ok(())
}
