use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::config::{self, json::Load};

#[derive(Debug)]
pub struct Project {
    root: PathBuf,
    config: config::root::Cfg,
    modules: Vec<Module>,
}

impl Project {
    pub async fn load(root: impl AsRef<Path>) -> Result<Project> {
        let config_file = config::root::Cfg::build_path(&root);
        let config = config::root::Cfg::load_json(&config_file).await?;

        let module_files = Module::discover_files(&root)?;
        let mut modules: Vec<Module> = Vec::new();

        for module_file in module_files {
            modules.push(Module::load(&module_file).await?);
        }

        Ok(Project {
            root: root.as_ref().to_path_buf(),
            config,
            modules,
        })
    }
}

#[derive(Debug)]
pub struct Module {
    root: PathBuf,
    config: config::module::Cfg,
}

impl Module {
    pub fn discover_files(root: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
        let root = root.as_ref();
        let pattern = root.join("**").join(config::module::FILE_NAME);

        Ok(glob::glob(pattern.to_str().unwrap())?
            .filter_map(|e| e.ok())
            .collect())
    }

    pub async fn load(file: impl AsRef<Path>) -> Result<Module> {
        let config_file = file.as_ref();
        let config = config::module::Cfg::load_json(&config_file).await?;

        Ok(Module {
            root: config_file.parent().unwrap().to_path_buf(),
            config,
        })
    }
}
