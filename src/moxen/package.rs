use super::manifest::PackageManifest;
use crate::common::{gather_files, MoxenError};
use anyhow::Result;
use flate2::{write::GzEncoder, Compression};
use std::path::PathBuf;
use tempdir::TempDir;

pub async fn package_content(_manifest: &PackageManifest, dir: &PathBuf) -> Result<()> {
    if !check_for_toc(&dir) {
        eprintln!("A TOC file at the project root is needed for an Addon.");
        anyhow::bail!(MoxenError::MissingTocFile);
    }
    let files = gather_files(&dir)?;
    create_tarball(&dir, files)?;

    Ok(())
}

fn check_for_toc(cwd: &PathBuf) -> bool {
    // This can be missing in the case of an Addon Collection
    // TODO: Handle collections when supported
    let entries: Vec<_> = glob::glob(cwd.join("*.toc").to_str().unwrap())
        .unwrap()
        .into_iter()
        .map(|e| e.unwrap())
        .collect();

    !entries.is_empty()
}

fn create_tarball(prefix: &PathBuf, files: Vec<PathBuf>) -> Result<()> {
    // TODO: Remove relicance on TempDir
    let td = TempDir::new("moxen")?;
    let path = td.path();
    for file in files.into_iter() {
        let stripped = file.strip_prefix(prefix)?;
        if let Some(parent) = stripped.parent() {
            std::fs::create_dir_all(path.join(parent))?;
        }

        let dst = path.join(stripped);
        std::fs::copy(&file, dst)?;
    }

    // TODO: Give it the package name
    // TODO: Save it to a common location or an output dir
    let output = std::fs::File::create("some-addon.moxen")?;
    let enc = GzEncoder::new(output, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", td.path())?;

    Ok(())
}
