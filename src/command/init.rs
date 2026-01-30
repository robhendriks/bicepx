use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Args;
use log::{error, info, warn};

use crate::{
    cli::Cli,
    config::{self, json},
    util,
};

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(short, long, default_value_t = false)]
    force: bool,

    #[arg(short, long, default_value = "**/main.bicep")]
    module_glob: PathBuf,

    #[arg(short, long)]
    default_registry: String,
}

pub async fn exec(cli: &Cli, args: &InitArgs) -> Result<()> {
    init_root(&cli, &args).await?;
    init_modules(&cli, &args).await
}

async fn init_root(cli: &Cli, args: &InitArgs) -> Result<()> {
    let root_cfg_file = config::root::Cfg::build_path(&cli.root);
    let root_cfg = config::root::Cfg::new();

    create_or_update_json_file(&cli.root, &root_cfg_file, &root_cfg, args.force).await
}

async fn init_modules(cli: &Cli, args: &InitArgs) -> Result<()> {
    let module_glob = cli.root.join(&args.module_glob);

    for entry in glob::glob(module_glob.to_str().unwrap())? {
        if let Ok(module_path) = entry {
            let module_root = module_path.parent().unwrap();
            let module_main = module_path.iter().last().unwrap().to_str().unwrap();

            let (module_name, module_categories) =
                infer_module_name_and_categories(&module_path, &module_glob)
                    .unwrap_or_else(|| (String::from(""), vec![]));

            let module_cfg =
                config::module::Cfg::new(&module_name, &module_main, module_categories);
            let module_cfg_file = config::module::Cfg::build_path(&module_root);

            let _ =
                create_or_update_json_file(&cli.root, &module_cfg_file, &module_cfg, args.force)
                    .await;
        }
    }

    Ok(())
}

fn infer_module_name_and_categories(
    module_path: &Path,
    module_glob: &Path,
) -> Option<(String, Vec<String>)> {
    let comps = util::wildcard::extract_wildcard_components(
        module_glob.to_str().unwrap(),
        module_path.to_str().unwrap(),
    )?;

    let name: String = comps
        .last()
        .map_or_else(|| String::from(""), |s| s.to_owned());

    let categories: Vec<String> = comps.iter().rev().skip(1).map(|s| s.to_string()).collect();

    Some((name, categories))
}

async fn create_or_update_json_file<T>(
    root: &Path,
    file: &Path,
    contents: &T,
    overwrite: bool,
) -> Result<()>
where
    T: json::Save,
{
    let rel_path = file.strip_prefix(&root).unwrap();
    let file_exists = file.exists();

    if !overwrite && file_exists {
        warn!("[Skip] {}", rel_path.display());
        return Ok(());
    }

    let result = contents.save_json(&file).await;

    match result {
        Ok(_) => {
            let verb = if file_exists { "Update" } else { "Create" };
            info!("[{}] {}", verb, rel_path.display());
        }
        Err(e) => {
            error!("{:#?}", e);
        }
    }

    Ok(())
}
