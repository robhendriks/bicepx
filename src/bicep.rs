use anyhow::Result;
use log::{error, info};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use tokio::sync::Semaphore;
use walkdir::WalkDir;

use crate::az::AzCli;

#[derive(Debug)]
pub struct BicepProject {
    pub root_path: PathBuf,
    pub module_paths: Vec<PathBuf>,
    pub modules: Vec<BicepModule>,
}

impl BicepProject {
    pub fn new(path: impl AsRef<Path>) -> Self {
        BicepProject {
            root_path: path.as_ref().to_path_buf(),
            module_paths: Vec::new(),
            modules: Vec::new(),
        }
    }

    pub fn discover_modules(&mut self, entrypoint: impl AsRef<Path>) -> Result<usize> {
        self.module_paths.clear();

        for entry in WalkDir::new(&self.root_path).follow_links(false) {
            let entry = entry?;
            let path = entry.path();

            if path.ends_with(&entrypoint) {
                self.module_paths.push(path.to_path_buf())
            }
        }

        self.module_paths.sort();

        Ok(self.module_paths.len())
    }

    pub async fn compile_modules(&mut self) -> Result<()> {
        self.modules.clear();

        // Number of module to compile
        let mod_count = self.module_paths.len();

        // Control max parallel compile tasks
        let semaphore = Arc::new(Semaphore::new(8));

        // Count number of completed compile tasks
        let completed = Arc::new(AtomicUsize::new(0));

        let compile_tasks: Vec<_> = self
            .module_paths
            .iter()
            .map(|mod_path| {
                let mod_path = mod_path.clone();
                let semaphore = semaphore.clone();
                let completed = completed.clone();

                tokio::spawn(async move {
                    // Aquire semaphore
                    let permit = semaphore.acquire().await?;

                    // Increment number of completed compile tasks
                    let count = completed.fetch_add(1, Ordering::Relaxed) + 1;

                    info!(
                        "[{}/{}] Compiling Bicep module {}",
                        count,
                        mod_count,
                        mod_path.display()
                    );

                    // Compile Bicep file into JSON
                    let response = AzCli::compile_module(&mod_path).await;

                    // Release semaphore
                    drop(permit);

                    response
                })
            })
            .collect();

        for compile_task in compile_tasks {
            let compile_result = compile_task.await?;

            match compile_result {
                Ok((path, source)) => {
                    let mut module = BicepModule {
                        meta: None,
                        path,
                        source,
                    };

                    match module.meta_from_source() {
                        Ok(_) => {}
                        Err(e) => {
                            error!("{:#}", e)
                        }
                    }

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

#[allow(dead_code)]
#[derive(Debug)]
pub struct BicepModule {
    pub path: PathBuf,
    pub source: Vec<u8>,
    pub meta: Option<BicepModuleMeta>,
}

impl BicepModule {
    pub fn meta_from_source(&mut self) -> Result<()> {
        let json: Value = serde_json::from_slice::<Value>(&self.source)?;

        let meta = serde_json::from_value::<BicepModuleMeta>(json["metadata"].clone())?;
        self.meta = Some(meta);

        info!("{:#?}", self.meta);

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct BicepModuleMeta {
    name: String,
    description: String,
    owner: String,
    version: String,
}
