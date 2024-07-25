use anyhow::Result;
use std::path::{Path, PathBuf};

const DEBUG: bool = true;
const INVALID_FILETYPE: [&'static str; 6] = ["exe", "c", "cpp", "rs", "js", "cs"];

#[derive(Debug)]
pub enum MoxenError {
    ManifestNotLoaded,
    MissingTocFile,
    MissingManifestFile,
    InvalidFileExtension(String),
    GeneralError(String),
}

impl std::error::Error for MoxenError {}

impl std::fmt::Display for MoxenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ManifestNotLoaded => writeln!(f, "Moxen.toml manifest is not loaded"),
            Self::MissingTocFile => writeln!(f, "missing required toc file"),
            Self::MissingManifestFile => writeln!(f, "missing Moxen.toml file"),
            Self::InvalidFileExtension(ext) => writeln!(f, "invalid file extension found: {ext}"),
            Self::GeneralError(err) => writeln!(f, "{err}"),
        }
    }
}

pub fn create_project_dir() -> Result<PathBuf> {
    let subfolders = vec!["package"];
    if DEBUG {
        let current_dir = std::env::current_dir()?.join(".moxen");
        for sf in subfolders.into_iter() {
            let dir = current_dir.join(sf);
            if !dir.exists() {
                std::fs::create_dir_all(&dir)?;
            }
        }

        return Ok(current_dir);
    }

    if let Some(home) = dirs::home_dir() {
        let project_dir = home.join(".moxen");
        for sf in subfolders.into_iter() {
            let dir = project_dir.join(sf);
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

fn invalid_filetypes(file: &PathBuf) -> Result<()> {
    match file.extension() {
        Some(ext) => {
            let ext = ext.to_str().unwrap();
            if INVALID_FILETYPE.contains(&ext) {
                anyhow::bail!(MoxenError::InvalidFileExtension(ext.to_owned()));
            }

            return Ok(());
        }
        None => Ok(()),
    }
}

fn iterdir(dir: impl AsRef<Path>, collector: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir.as_ref())? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            iterdir(path, collector)?;
        } else {
            match invalid_filetypes(&path) {
                Ok(_) => collector.push(path),
                Err(e) => return Err(e.into()),
            }
        }
    }

    Ok(())
}
