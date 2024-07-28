pub mod manifest;
pub mod package;
pub mod publish;

use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::common::{create_project_dir, MoxenError};
use manifest::{bootstrap_lua, bootstrap_toc, PackageManifest};
use package::package_content;

pub struct Manager {
    mox_dir: PathBuf,
    src_dir: PathBuf,
    manifest: Option<PackageManifest>,
}

impl Manager {
    pub fn new(target_dir: Option<String>) -> Self {
        let dir = if let Some(dir) = target_dir {
            PathBuf::from(dir)
        } else {
            std::env::current_dir().expect("unable to get current directory")
        };

        let mox_dir = create_project_dir().expect("cannot create project directory");
        Self {
            mox_dir,
            src_dir: dir,
            manifest: None,
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

    pub fn load(&mut self) -> Result<()> {
        self.manifest = Some(PackageManifest::load(&self.src_dir)?);
        Ok(())
    }

    pub fn info(&self) -> Result<()> {
        match &self.manifest {
            Some(manifest) => println!("{manifest}"),
            None => anyhow::bail!(MoxenError::ManifestNotLoaded),
        }

        Ok(())
    }

    pub fn package(&self) -> Result<()> {
        match &self.manifest {
            Some(manifest) => package_content(&manifest, &self.src_dir, &self.mox_dir),
            None => anyhow::bail!(MoxenError::ManifestNotLoaded),
        }
    }

    pub async fn publish(&self) -> Result<()> {
        println!("Publishing package!");
        Ok(())
    }

    // TODO: Improve name and capabilities
    pub fn convert_to_mox(&self) -> Result<()> {
        match self.src_dir.file_name() {
            Some(dir) => {
                // This should always be fine
                let name = dir.to_str().unwrap();
                let mut manifest = PackageManifest::fresh(name);
                manifest.write(&self.src_dir)?;
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

    pub fn clean(&self) -> Result<()> {
        std::fs::remove_dir_all(&self.mox_dir)?;
        Ok(())
    }
}
