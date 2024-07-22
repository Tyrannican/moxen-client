pub mod manifest;
pub mod package;

use anyhow::Result;
use std::path::PathBuf;

use crate::common::{create_project_dir, MoxenError};
use manifest::{load_manifest, PackageManifest};
use package::package_content;

pub struct Manager {
    home_dir: PathBuf,
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

        let home_dir = create_project_dir().expect("cannot create project directory");
        Self {
            home_dir,
            src_dir: dir,
            manifest: None,
        }
    }

    pub fn load(&mut self) -> Result<()> {
        self.manifest = Some(load_manifest(&self.src_dir)?);
        Ok(())
    }

    pub fn info(&self) -> Result<()> {
        match &self.manifest {
            Some(manifest) => println!("{manifest}"),
            None => anyhow::bail!(MoxenError::ManifestNotLoaded),
        }

        Ok(())
    }

    pub async fn package(&self) -> Result<()> {
        match &self.manifest {
            Some(manifest) => package_content(&manifest, &self.src_dir, &self.home_dir).await,
            None => anyhow::bail!(MoxenError::ManifestNotLoaded),
        }
    }
}
