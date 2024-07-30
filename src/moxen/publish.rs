use anyhow::Result;
use base64::prelude::*;
use sha1::{Digest, Sha1};

use std::{collections::HashMap, path::PathBuf};

use super::manifest::{NormalizedManifest, PackageManifest};

const PUBLISH_URL: &str = "http://localhost:9000/api/v1/mox/new";

pub async fn publish_package(manifest: PackageManifest, pkg_path: PathBuf) -> Result<()> {
    // Check if it exists first

    let (cksum, pkg) = generate_checksum(&pkg_path)?;
    let normalised = manifest.normalise(cksum);
    send_request(normalised, pkg).await?;

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
    pkg: Vec<u8>,
) -> Result<HashMap<String, String>> {
    let mut body = HashMap::new();
    let manifest_as_str = toml::to_string(&manifest)?;
    let manifest = BASE64_STANDARD.encode(manifest_as_str.as_bytes());
    let pkg = BASE64_STANDARD.encode(pkg);
    body.insert("manifest".to_string(), manifest);
    body.insert("package".to_string(), pkg);

    Ok(body)
}

async fn send_request(manifest: NormalizedManifest, pkg: Vec<u8>) -> Result<()> {
    let req_body = create_request_body(manifest, pkg)?;
    let client = reqwest::Client::new();

    let response = client.post(PUBLISH_URL).json(&req_body).send().await?;
    println!("Response: {}", response.text().await?);
    Ok(())
}
