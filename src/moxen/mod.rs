pub mod manifest;
pub mod package;

use anyhow::Result;
use std::path::PathBuf;

use crate::common::{create_project_dir, MoxenError};
use manifest::{load_manifest, PackageManifest};
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
            Some(manifest) => package_content(&manifest, &self.src_dir, &self.mox_dir).await,
            None => anyhow::bail!(MoxenError::ManifestNotLoaded),
        }
    }

    pub fn clean(&self) -> Result<()> {
        std::fs::remove_dir_all(&self.mox_dir)?;
        Ok(())
    }
}
