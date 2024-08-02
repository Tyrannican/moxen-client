use anyhow::Result;
use std::path::PathBuf;

use crate::{
    common::{untarball, validate_package_checksum},
    moxen::{api, manifest::NormalizedManifest},
};

pub async fn download_dependency(src_dir: PathBuf, dep: String) -> Result<()> {
    let libs_dir = src_dir.join(format!("libs/{dep}"));
    if libs_dir.exists() {
        return Ok(());
    }

    let (manifest, package) = match api::fetch_mox(&dep).await {
        Ok((manifest, package)) => (manifest, package),
        Err(err) => {
            eprintln!("Error: {err}");
            anyhow::bail!(err);
        }
    };
    let manifest = toml::from_str::<NormalizedManifest>(&manifest)?;

    match validate_package_checksum(&package, &manifest.cksum) {
        Ok(()) => {}
        Err(error) => {
            eprintln!("{error}");
            anyhow::bail!(error);
        }
    }

    if !libs_dir.exists() {
        std::fs::create_dir_all(&libs_dir)?;
    }
    untarball(&libs_dir, package)?;

    println!("Adding {dep} to {}", libs_dir.display());
    Ok(())
}
