use anyhow::Result;
use reqwest::StatusCode;

use crate::common::MoxenError;

pub const API_URL: &str = "http://localhost:9000";

pub async fn fetch_mox(name: &str) -> Result<Vec<u8>> {
    let url = format!("{API_URL}/api/v1/mox/{name}");
    let client = reqwest::Client::new();

    let response = client.get(url).send().await?;
    match response.status() {
        StatusCode::OK => {
            let data = response.bytes().await?;
            return Ok(data.to_vec());
        }
        StatusCode::NOT_FOUND => Err(MoxenError::ProjectNotFound.into()),
        _ => {
            let text = response.text().await?;
            return Err(MoxenError::GeneralError(text).into());
        }
    }
}
