use anyhow::Result;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use sha1::{Digest, Sha1};
use std::path::{Path, PathBuf};
use tar::Archive;

const DEBUG: bool = true;
const INVALID_FILETYPE: [&'static str; 6] = ["exe", "c", "cpp", "rs", "js", "cs"];

#[derive(Debug)]
pub enum MoxenError {
    MissingTocFile,
    MissingManifestFile,
    InvalidFileExtension(String),
    ProjectAlreadyExists,
    ProjectNotFound(String),
    ChecksumFailure((String, String)),
    ConfigError(String),
    LoadError(String),
    InvalidUsername(String),
    ApiError(String),
    AuthError(String),
    GeneralError(String),
}

impl std::error::Error for MoxenError {}

impl std::fmt::Display for MoxenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTocFile => writeln!(f, "missing required toc file"),
            Self::MissingManifestFile => writeln!(f, "missing Moxen.toml file"),
            Self::InvalidFileExtension(ext) => writeln!(f, "invalid file extension found - {ext}"),
            Self::ProjectNotFound(pkg) => writeln!(f, "project not found in registry - {pkg}"),
            Self::ProjectAlreadyExists => writeln!(f, "project already exists"),
            Self::ChecksumFailure((chk1, chk2)) => {
                writeln!(f, "checksum failure: {chk1} doesn't match expected {chk2}")
            }
            Self::LoadError(err) => writeln!(f, "loading error: {err}"),
            Self::ConfigError(err) => {
                writeln!(f, "config error: {err}")
            }
            Self::InvalidUsername(reason) => writeln!(f, "invalid username: {reason}"),
            Self::ApiError(err) => {
                writeln!(f, "moxen registry api error: {err}")
            }
            Self::AuthError(err) => writeln!(f, "authentication error: {err}"),
            Self::GeneralError(err) => writeln!(f, "error occurred: {err}"),
        }
    }
}

pub fn create_project_dir() -> Result<PathBuf> {
    let subfolders = vec!["package"];

    if let Some(home) = dirs::home_dir() {
        let current_dir = std::env::current_dir()?;
        let project_dir = if DEBUG {
            current_dir.join(".moxen")
        } else {
            home.join(".moxen")
        };
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

pub fn tarball(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    let output = std::fs::File::create(&dst)?;
    let enc = GzEncoder::new(output, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", src)?;
    Ok(())
}

pub fn untarball(path: &PathBuf, data: Vec<u8>) -> Result<()> {
    let tar = GzDecoder::new(data.as_slice());
    let mut archive = Archive::new(tar);
    archive.unpack(&path)?;
    Ok(())
}

pub fn validate_package_checksum(data: &Vec<u8>, checksum: &str) -> Result<(), MoxenError> {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let check = hex::encode(hasher.finalize());
    if check == checksum {
        Ok(())
    } else {
        Err(MoxenError::ChecksumFailure((check, checksum.to_string())))
    }
}
