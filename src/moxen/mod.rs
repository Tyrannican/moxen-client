pub mod api;
pub mod download;
pub mod manifest;
pub mod package;
pub mod publish;

use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::common::{create_project_dir, MoxenError};
use manifest::{bootstrap_lua, bootstrap_toc, PackageManifest};
use package::package_content;
use publish::publish_package;

pub struct Manager {
    mox_dir: PathBuf,
    src_dir: PathBuf,
    manifest: PackageManifest,
}

// TODO: Set the current directory to either the supplied dir or the current dir
// That way we can get rid of the Option for the manifest and load it directly
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

        std::env::set_current_dir(&dir).expect("error setting current directory");
        let mox_dir = create_project_dir().expect("cannot create project directory");
        let manifest = PackageManifest::load(&dir).expect("error loading package manifest");
        Self {
            mox_dir,
            src_dir: dir,
            manifest,
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
                // This should always be fine
                let name = dir.to_str().unwrap();
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
        let mut handles = vec![];
        let deps_copy = deps.clone();

        for dep in deps.into_iter() {
            let src_dir = self.src_dir.clone();
            let hdl = tokio::task::spawn(async move {
                download::download_dependency(src_dir, dep.clone()).await
            });
            handles.push(hdl);
        }

        for hdl in handles {
            let _ = hdl.await?;
        }

        for dep in deps_copy {
            self.manifest.add_dependency(dep);
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
