use crate::common::{gather_files, MoxenError};
use anyhow::Result;
use flate2::{write::GzEncoder, Compression};
use std::path::PathBuf;

use super::manifest::PackageManifest;

pub async fn package_content(
    manifest: &PackageManifest,
    pkg_dir: &PathBuf,
    target_dir: &PathBuf,
) -> Result<()> {
    let name = manifest.normalise_name();
    let package_dir = target_dir.join("package").join(&name);
    let cmp_dir = target_dir.join("package").join(&format!("{name}.mox"));
    if !check_for_toc(&pkg_dir) {
        eprintln!("A TOC file at the project root is needed for an Addon.");
        anyhow::bail!(MoxenError::MissingTocFile);
    }
    let files = gather_files(&pkg_dir)?;
    create_tarball(&pkg_dir, files, package_dir, cmp_dir)?;

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

fn create_tarball(
    prefix: &PathBuf,
    files: Vec<PathBuf>,
    pkg_dir: PathBuf,
    cmp_dir: PathBuf,
) -> Result<()> {
    for file in files.into_iter() {
        let stripped = file.strip_prefix(prefix)?;
        if let Some(parent) = stripped.parent() {
            std::fs::create_dir_all(pkg_dir.join(parent))?;
        }

        let dst = pkg_dir.join(stripped);
        std::fs::copy(&file, dst)?;
    }

    let output = std::fs::File::create(&cmp_dir)?;
    let enc = GzEncoder::new(output, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", pkg_dir)?;

    Ok(())
}
