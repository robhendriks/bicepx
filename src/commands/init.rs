use anyhow::Result;
use clap::Args;
use log::{error, info};

use crate::{cli::Cli, config::Config, config::ProjectConfig};

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

pub async fn execute(cli: &Cli, args: &InitArgs) -> Result<()> {
    let prj_config_path = cli.get_config_path();
    let prj_config = ProjectConfig::new();

    match prj_config.save_to(&prj_config_path, args.force).await {
        Ok(_) => {
            info!("Created '{}'", prj_config_path.display());
        }
        Err(err) => {
            error!("{}", err)
        }
    }

    Ok(())
}
