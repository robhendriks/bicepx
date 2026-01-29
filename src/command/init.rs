use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use log::{error, info, warn};
use semver::Version;

use crate::{
    cli::Cli,
    config::{
        json::Save,
        module::ModuleCfg,
        root::{ModulesCfg, RootCfg},
    },
};

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(short, long, default_value_t = false)]
    force: bool,

    #[arg(short, long, default_value = "**/main.bicep")]
    module_glob: PathBuf,
}

pub async fn exec(cli: &Cli, args: &InitArgs) -> Result<()> {
    init_root(&cli, &args).await?;
    init_modules(&cli, &args).await?;

    info!("Done");

    Ok(())
}

async fn init_root(cli: &Cli, args: &InitArgs) -> Result<()> {
    let root_cfg_path = RootCfg::build_path(&cli.root);
    let root_cfg_path_rel = root_cfg_path.strip_prefix(&cli.root).unwrap();

    if !args.force && root_cfg_path.exists() {
        warn!("File exists: {}", root_cfg_path_rel.display());
        return Ok(());
    }

    let root_cfg = RootCfg {
        modules: ModulesCfg {
            glob: args.module_glob.clone(),
        },
    };

    let result = root_cfg.save_json(&root_cfg_path).await;

    match result {
        Ok(_) => {
            info!("File created: {}", root_cfg_path_rel.display());
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }

    Ok(())
}

async fn init_modules(cli: &Cli, args: &InitArgs) -> Result<()> {
    let module_glob = cli.root.join(&args.module_glob);

    for entry in glob::glob(module_glob.to_str().unwrap())? {
        if let Ok(path) = entry {
            let module_root = path.parent().unwrap();

            let module_cfg_path = ModuleCfg::build_path(&module_root);
            let module_cfg_path_rel = module_cfg_path.strip_prefix(&cli.root).unwrap();

            let module_cfg_exists = module_cfg_path.exists();

            if !args.force && module_cfg_exists {
                warn!("File exists: {}", module_cfg_path_rel.display());
                continue;
            }

            let module_cfg = ModuleCfg {
                name: String::from(""),
                version: Version::new(0, 0, 0),
            };

            let result = module_cfg.save_json(&module_cfg_path).await;

            match result {
                Ok(_) => {
                    info!("File created: {}", module_cfg_path_rel.display());
                }
                Err(e) => {
                    error!("{:?}", e);
                }
            }
        }
    }

    Ok(())
}
