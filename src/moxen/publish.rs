use anyhow::Result;
use base64::prelude::*;
use sha1::{Digest, Sha1};

use std::{collections::HashMap, path::PathBuf};

use super::{
    api,
    manifest::{NormalizedManifest, PackageManifest},
};

pub async fn publish_package(
    manifest: PackageManifest,
    pkg_path: PathBuf,
    api_key: &str,
    username: &str,
) -> Result<()> {
    let (cksum, pkg) = generate_checksum(&pkg_path)?;
    let normalised = manifest.normalise(cksum);
    let req_body = create_request_body(normalised, &pkg)?;
    match api::publish_mox_package(req_body, api_key, username).await {
        Ok(()) => println!("Package published successfully!"),
        Err(e) => anyhow::bail!(e),
    }

    Ok(())
}

fn generate_checksum(file: &PathBuf) -> Result<(String, Vec<u8>)> {
    let fd = std::fs::read(&file)?;
    let mut sha = Sha1::new();
    sha.update(&fd);
    let cksum = hex::encode(sha.finalize());

    Ok((cksum, fd))
}

fn create_request_body(
    manifest: NormalizedManifest,
    pkg: &[u8],
) -> Result<HashMap<String, String>> {
    let mut body = HashMap::new();
    let manifest_as_str = toml::to_string(&manifest)?;
    let manifest = BASE64_STANDARD.encode(manifest_as_str.as_bytes());
    let pkg = BASE64_STANDARD.encode(pkg);
    body.insert("manifest".to_string(), manifest);
    body.insert("package".to_string(), pkg);

    Ok(body)
}
