use std::collections::HashMap;

use anyhow::Result;
use reqwest::Client;
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
    let client = Client::new();

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

pub async fn publish_mox_package(body: HashMap<String, String>) -> Result<()> {
    let client = Client::new();
    let url = format!("{API_URL}/api/v1/mox/new");
    let response = client.post(url).json(&body).send().await?;
    let status = response.status();
    match status {
        StatusCode::CREATED => Ok(()),
        StatusCode::CONFLICT => Err(MoxenError::ProjectAlreadyExists.into()),
        _ => {
            let text = response.text().await?;
            return Err(MoxenError::GeneralError(text).into());
        }
    }
}
