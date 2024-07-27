use crate::common::{gather_files, MoxenError};
use anyhow::Result;
use flate2::{write::GzEncoder, Compression};
use std::path::PathBuf;

use super::manifest::PackageManifest;

// TODO: Better logging of the packaging step
pub fn package_content(
    manifest: &PackageManifest,
    src_path: &PathBuf,
    mox_path: &PathBuf,
) -> Result<()> {
    let name = manifest.normalise_name();
    println!("Packaging {} as {}...", src_path.display(), name);
    let package_target_path = mox_path.join("package").join(&name);
    let compressed_target_path = mox_path.join("package").join(&format!("{name}.mox"));

    if let Some(collection) = &manifest.collection {
        for item in collection.members.iter() {
            let item_path = src_path.join(item);
            if !check_for_toc(&item_path) {
                eprintln!(
                    "A TOC file in the root of {} is needed.",
                    item_path.display()
                );
                anyhow::bail!(MoxenError::MissingTocFile);
            }
        }
    } else {
        if !check_for_toc(&src_path) {
            eprintln!(
                "A TOC file at the project root {} is needed for an Addon.",
                src_path.display()
            );
            anyhow::bail!(MoxenError::MissingTocFile);
        }
    }

    package_mox(src_path, &package_target_path, &compressed_target_path)?;
    println!("Crafted {}!", compressed_target_path.display());
    Ok(())
}

fn package_mox(
    src_path: &PathBuf,
    dst_path: &PathBuf,
    compressed_dst_path: &PathBuf,
) -> Result<()> {
    let files = gather_files(&src_path)?;
    create_tarball(&src_path, files, dst_path, compressed_dst_path)?;
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

fn create_tarball(
    prefix: &PathBuf,
    files: Vec<PathBuf>,
    package_target_path: &PathBuf,
    compressed_target_path: &PathBuf,
) -> Result<()> {
    for file in files.into_iter() {
        let stripped = file.strip_prefix(prefix)?;
        if let Some(parent) = stripped.parent() {
            std::fs::create_dir_all(package_target_path.join(parent))?;
        }

        let dst = package_target_path.join(stripped);
        std::fs::copy(&file, dst)?;
    }

    let output = std::fs::File::create(&compressed_target_path)?;
    let enc = GzEncoder::new(output, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", package_target_path)?;

    Ok(())
}
