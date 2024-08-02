use crate::common::{gather_files, tarball, MoxenError};
use anyhow::Result;
use std::path::PathBuf;

use super::manifest::PackageManifest;

pub fn package_content(
    manifest: &PackageManifest,
    src_path: &PathBuf,
    mox_path: &PathBuf,
) -> Result<PathBuf> {
    let name = manifest.normalise_name(true);
    println!("Packaging {} as {}...", src_path.display(), name);
    let package_target_path = mox_path.join("package").join(&name);
    let compressed_target_path = mox_path.join("package").join(&format!("{name}.mox"));
    if !check_for_toc(&src_path) {
        eprintln!("No TOC file present, searching subdirectories...");
        if !find_any_toc(&src_path) {
            eprintln!(
                "Cannot find a TOC file in the project: {}!",
                src_path.display()
            );
            anyhow::bail!(MoxenError::MissingTocFile);
        }
    }

    package_mox(src_path, &package_target_path, &compressed_target_path)?;
    println!("Crafted {}!", compressed_target_path.display());
    Ok(compressed_target_path)
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
    let toc = glob::glob(cwd.join("*.toc").to_str().unwrap())
        .unwrap()
        .into_iter()
        .count();

    toc != 0
}

// TODO: This could be better
fn find_any_toc(cwd: &PathBuf) -> bool {
    let tocs = glob::glob(cwd.join("**/*.toc").to_str().unwrap())
        .unwrap()
        .into_iter()
        .count();

    tocs != 0
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

    tarball(package_target_path, &compressed_target_path)?;

    Ok(())
}
