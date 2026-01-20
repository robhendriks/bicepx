use std::path::PathBuf;

use clap::{Parser, Subcommand};
use log::debug;

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

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let ctx = Context::from_cli(&cli);

    debug!("Using config: {}", ctx.config_path.display());

    match &cli.command {
        Commands::Build => {
            let cfg = config::Root::load_from_file(&ctx.config_path);

            match cfg {
                Ok(root) => {
                    println!("{:#?}", root)
                }
                Err(e) => {
                    eprintln!("Error: {:#}", e);
                }
            }
        }
    }

    debug!("Done");
}
