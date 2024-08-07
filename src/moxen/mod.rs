pub mod api;
pub mod config;
pub mod download;
pub mod manifest;
pub mod package;
pub mod publish;

use anyhow::{Context, Result};
use config::MoxenConfig;
use std::path::PathBuf;
use tokio::sync::mpsc::channel;

use crate::common::{create_project_dir, MoxenError};
use manifest::{bootstrap_lua, bootstrap_toc, PackageManifest};
use package::package_content;
use publish::publish_package;

// TODO: Find a use for the config
#[allow(dead_code)]
pub struct Manager {
    mox_dir: PathBuf,
    src_dir: PathBuf,
    manifest: PackageManifest,
    config: MoxenConfig,
}

impl Manager {
    pub fn new(target_dir: Option<String>) -> Self {
        let dir = if let Some(dir) = target_dir {
            PathBuf::from(dir)
                .canonicalize()
                .expect("error canonicalising path")
        } else {
            std::env::current_dir()
                .expect("unable to get current directory")
                .canonicalize()
                .expect("error canonicalising path")
        };

        // TODO: Replace these with MoxenError handling
        std::env::set_current_dir(&dir).expect("error setting current directory");
        let mox_dir = create_project_dir().expect("cannot create project directory");
        let config = MoxenConfig::load(&mox_dir).expect("unable to load moxen config");
        let manifest = PackageManifest::load(&dir).expect("error loading package manifest");
        println!("Config loaded: {config:?}");

        Self {
            mox_dir,
            src_dir: dir,
            manifest,
            config,
        }
    }

    pub fn bootstrap(&mut self, name: String) -> Result<()> {
        let project_path = self.src_dir.join(&name);
        if !project_path.exists() {
            std::fs::create_dir_all(&project_path).context("creating new project directory")?;
        } else {
            eprintln!("A project with the name `{name}` already exists!");
            anyhow::bail!(MoxenError::ProjectAlreadyExists);
        }
        self.src_dir = project_path;
        let manifest = PackageManifest::fresh(&name);
        manifest.write(&self.src_dir)?;

        bootstrap_lua(&self.src_dir)?;
        bootstrap_toc(&self.src_dir, &manifest)?;
        println!("Created new Mox package `{name}`");

        Ok(())
    }

    pub fn info(&self) {
        println!("{}", self.manifest);
    }

    pub fn package(&self) -> Result<PathBuf> {
        let ignore_list = self.generate_ignore_list();
        return package_content(&self.manifest, &self.src_dir, &self.mox_dir, ignore_list);
    }

    pub async fn publish(self) -> Result<()> {
        let pkg_path = self.package()?;
        publish_package(self.manifest, pkg_path).await
    }

    // TODO: Improve name and capabilities
    pub fn convert_to_mox(&self) -> Result<()> {
        match self.src_dir.file_name() {
            Some(dir) => {
                let name = dir.to_str().unwrap_or("Moxen Addon");
                let manifest = PackageManifest::interactive(name);
                manifest.write(&self.src_dir)?;
                println!("Bootstrapped new mox: {name}");
            }
            None => {
                eprintln!(
                    "cannot determine folder name / location at: {}",
                    self.src_dir.display()
                );
                anyhow::bail!(MoxenError::GeneralError("invalid directory".to_string()))
            }
        }

        Ok(())
    }

    // TODO: Clean up clone mess here
    pub async fn download_dependencies(&mut self, deps: Vec<String>) -> Result<()> {
        let size = deps.len();
        let (tx, mut rx) = channel(size);
        for dep in deps.into_iter() {
            let src_dir = self.src_dir.clone();
            let sender = tx.clone();
            tokio::task::spawn(async move {
                if let Ok(_) = download::download_dependency(src_dir, &dep).await {
                    if let Err(_) = sender.send(dep).await {
                        eprintln!("dep receiver dropped");
                    }
                }
            });
        }
        drop(tx);

        while let Some(dep) = rx.recv().await {
            self.manifest.add_dependency(dep.to_owned());
        }

        self.manifest.write(&self.src_dir)?;

        Ok(())
    }

    pub fn clean(&self) -> Result<()> {
        std::fs::remove_dir_all(&self.mox_dir)?;
        Ok(())
    }

    fn generate_ignore_list(&self) -> Option<Vec<PathBuf>> {
        let mut inner = vec![];

        match &self.manifest.mox.ignore {
            Some(items) => {
                let globs: Vec<String> = items
                    .into_iter()
                    .map(|item| {
                        self.src_dir
                            .join(item)
                            .to_str()
                            .expect("unable to convert ignore path")
                            .to_owned()
                    })
                    .collect();

                for pattern in globs.into_iter() {
                    let entries = glob::glob(&pattern)
                        .unwrap()
                        .into_iter()
                        .map(|c| c.unwrap())
                        .collect::<Vec<PathBuf>>();

                    inner.extend(entries);
                }

                return Some(inner);
            }
            None => None,
        }
    }
}
