use anyhow::Result;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::common::MoxenError;

pub const API_URL: &str = "http://localhost:9000";

#[derive(Deserialize)]
pub struct DownloadPackageResponse {
    pub manifest: String,
    pub package: Vec<u8>,
    pub error: Option<String>,
}

pub async fn fetch_mox(name: &str) -> Result<(String, Vec<u8>)> {
    let url = format!("{API_URL}/api/v1/mox/{name}");
    let client = reqwest::Client::new();

    let response = client.get(url).send().await?;
    let status = response.status();
    let response = response.json::<DownloadPackageResponse>().await?;
    match status {
        StatusCode::OK => {
            let manifest = response.manifest;
            let package = response.package;
            return Ok((manifest, package));
        }
        StatusCode::NOT_FOUND => {
            let error_message = response.error.unwrap();
            Err(MoxenError::ProjectNotFound(error_message).into())
        }
        _ => {
            let error_message = response.error.unwrap();
            return Err(MoxenError::GeneralError(error_message).into());
        }
    }
}
