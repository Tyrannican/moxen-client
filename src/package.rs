use crate::{
    common::{gather_files, ScholoError},
    manifest::Addon,
};
use anyhow::Result;
use std::path::PathBuf;

pub async fn package(_manifest: Addon, dir: PathBuf) -> Result<()> {
    if !check_for_toc(&dir) {
        eprintln!("A TOC file at the project root is needed for an Addon.");
        anyhow::bail!(ScholoError::MissingTocFile);
    }
    let files = gather_files(&dir)?;
    let _ = create_tarball(files)?;
    // TODO: Ignore list for files from the manifest
    // 2. Package everything in a ZIP / Tarball

    Ok(())
}

fn check_for_toc(cwd: &PathBuf) -> bool {
    let entries: Vec<_> = glob::glob(cwd.join("*.toc").to_str().unwrap())
        .unwrap()
        .into_iter()
        .map(|e| e.unwrap())
        .collect();

    !entries.is_empty()
}

fn create_tarball(files: Vec<PathBuf>) -> Result<()> {
    Ok(())
}
