use anyhow::Result;
use log::{debug, error, info};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::Semaphore;
use walkdir::WalkDir;

use crate::az::AzCli;

#[derive(Debug)]
pub struct BicepProject {
    root_path: PathBuf,
    module_paths: Vec<PathBuf>,
    modules: Vec<BicepModule>,
}

impl BicepProject {
    pub fn new(path: impl AsRef<Path>) -> Self {
        BicepProject {
            root_path: path.as_ref().to_path_buf(),
            module_paths: Vec::new(),
            modules: Vec::new(),
        }
    }

    pub fn discover_modules(&mut self, entrypoint: impl AsRef<Path>) -> Result<()> {
        self.module_paths.clear();

        for entry in WalkDir::new(&self.root_path).follow_links(false) {
            let entry = entry?;
            let path = entry.path();

            if path.file_name() == Some("main.bicep".as_ref()) {
                if path.ends_with(&entrypoint) {
                    self.module_paths.push(path.to_path_buf())
                }
            }
        }

        self.module_paths.sort();

        Ok(())
    }

    pub async fn compile_modules(&mut self) -> Result<()> {
        self.modules.clear();

        let compile_sem = Arc::new(Semaphore::new(4));
        let compile_tasks: Vec<_> = self
            .module_paths
            .iter()
            .map(|mod_path| {
                let mod_path = mod_path.clone();
                let semaphore = Arc::clone(&compile_sem);

                tokio::spawn(async move {
                    let _permit = semaphore.acquire().await.unwrap();

                    info!("Building module: {}", mod_path.display());

                    AzCli::compile_module(&mod_path).await
                })
            })
            .collect();

        for compile_task in compile_tasks {
            let compile_result = compile_task.await?;

            match compile_result {
                Ok(module) => {
                    debug!("Bicep module OK {}", module.path.display());
                    self.modules.push(module);
                }
                Err(e) => {
                    error!("{:#}", e)
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BicepModule {
    pub path: PathBuf,
    pub _source: String,
}
