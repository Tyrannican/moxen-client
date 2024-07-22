use anyhow::Result;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum MoxenError {
    ManifestNotLoaded,
    MissingTocFile,
    MissingManifestFile,
    GeneralError(String),
}

impl std::error::Error for MoxenError {}

impl std::fmt::Display for MoxenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ManifestNotLoaded => writeln!(f, "Moxen.toml manifest is not loaded"),
            Self::MissingTocFile => writeln!(f, "missing required toc file"),
            Self::MissingManifestFile => writeln!(f, "missing Moxen.toml file"),
            Self::GeneralError(err) => writeln!(f, "{err}"),
        }
    }
}

pub fn create_project_dir() -> Result<PathBuf> {
    if let Some(home) = dirs::home_dir() {
        let project_dir = home.join(".moxen");
        for dir in [project_dir.join("package")].iter() {
            if !dir.exists() {
                std::fs::create_dir_all(&dir)?;
            }
        }

        return Ok(project_dir);
    }

    Err(MoxenError::GeneralError("unable to determine home directory".to_string()).into())
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
