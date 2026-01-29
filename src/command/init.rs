use std::path::PathBuf;

use anyhow::anyhow;
use clap::Args;

use crate::{
    cli::Cli,
    config::{
        json::Save,
        root::{ModuleCfg, RootCfg},
    },
};

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(short, long, default_value_t = false)]
    force: bool,

    #[arg(short, long, default_value = "**/*.bicep")]
    module_glob: PathBuf,
}

pub async fn exec(cli: &Cli, args: &InitArgs) -> anyhow::Result<()> {
    let root_cfg_path = RootCfg::build_path(&cli.root);

    if !args.force && root_cfg_path.exists() {
        return Err(anyhow!("File already exists: {}", root_cfg_path.display()));
    }

    let root_cfg = RootCfg {
        modules: ModuleCfg {
            glob: args.module_glob.clone(),
        },
    };

    root_cfg.save_json(&root_cfg_path).await
}
