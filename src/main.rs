use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

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

#[derive(Debug, Subcommand)]
enum Commands {
    Init,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            println!("init");
            Ok(())
        }
    }
}
