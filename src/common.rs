use anyhow::Result;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ScholoError {
    MissingTocFile,
    MissingManifestFile,
}

impl std::error::Error for ScholoError {}

impl std::fmt::Display for ScholoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTocFile => writeln!(f, "missing required toc file"),
            Self::MissingManifestFile => writeln!(f, "missing Scholo.toml file"),
        }
    }
}

pub fn gather_files(dir: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    iterdir(dir, &mut files)?;

    Ok(files)
}

fn iterdir(dir: impl AsRef<Path>, collector: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir.as_ref())? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            iterdir(path, collector)?;
        } else {
            collector.push(path);
        }
    }

    Ok(())
}
