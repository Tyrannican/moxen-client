pub mod response;
use response::DownloadPackageResponse;

use anyhow::Result;
use reqwest::Client;
use reqwest::StatusCode;
use response::UserRecoveryResponse;
use response::UserRegisterResponse;
use std::collections::HashMap;

use crate::common::MoxenError;

pub const API_URL: &str = "https://localhost:9443";

fn generate_request_client() -> Result<Client> {
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .danger_accept_invalid_certs(true)
        .build()?;

    Ok(client)
}

pub async fn fetch_mox(name: &str) -> Result<(String, Vec<u8>)> {
    let url = format!("{API_URL}/api/v1/mox/{name}");
    let client = generate_request_client()?;

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
            return Err(MoxenError::ApiError(error_message).into());
        }
    }
}

pub async fn publish_mox_package(
    body: HashMap<String, String>,
    api_key: &str,
    username: &str,
) -> Result<()> {
    let client = generate_request_client()?;
    let url = format!("{API_URL}/api/v1/mox/new");
    let response = client
        .post(url)
        .json(&body)
        .header("x-api-key", api_key)
        .header("x-authorize-user", username)
        .send()
        .await?;

    let status = response.status();
    match status {
        StatusCode::CREATED => Ok(()),
        StatusCode::CONFLICT => Err(MoxenError::ProjectAlreadyExists.into()),
        StatusCode::UNAUTHORIZED => Err(MoxenError::ApiError("invalid api key".to_string()).into()),
        _ => {
            let text = response.text().await?;
            return Err(MoxenError::ApiError(text).into());
        }
    }
}

pub async fn generate_challenge(name: &str, pub_key: &str) -> Result<String> {
    let client = generate_request_client()?;
    let url = format!("{API_URL}/api/v1/auth/challenge");
    let mut body = HashMap::new();
    body.insert("name", name);
    body.insert("key", pub_key);

    let response = client.post(url).json(&body).send().await?;
    let status = response.status();
    let text = response.text().await?;
    match status {
        StatusCode::OK => Ok(text),
        _ => Err(MoxenError::ApiError(text).into()),
    }
}

pub async fn signup(original: String, challenge: String) -> Result<(String, Vec<String>)> {
    let client = generate_request_client()?;
    let url = format!("{API_URL}/api/v1/auth/register");
    let mut body = HashMap::new();
    body.insert("original", original);
    body.insert("challenge", challenge);

    let response = client.post(url).json(&body).send().await?;
    let status = response.status();
    let data = response.json::<UserRegisterResponse>().await?;

    match status {
        StatusCode::CREATED => Ok((data.api_key, data.recovery_codes)),
        _ => {
            let error = data.error.unwrap();
            return Err(MoxenError::ApiError(error).into());
        }
    }
}

pub async fn recover(challenge: String, signed: String, code: String) -> Result<String> {
    let client = generate_request_client()?;
    let url = format!("{API_URL}/api/v1/auth/recovery");
    let mut body = HashMap::new();
    body.insert("challenge", challenge);
    body.insert("signed", signed);
    body.insert("code", code);

    let response = client.post(url).json(&body).send().await?;
    let status = response.status();
    let data = response.json::<UserRecoveryResponse>().await?;

    match status {
        StatusCode::OK => Ok(data.api_key),
        StatusCode::UNAUTHORIZED => {
            let error_msg = data.error.unwrap();
            Err(MoxenError::AuthError(error_msg).into())
        }
        _ => {
            let error = data.error.unwrap();
            Err(MoxenError::ApiError(error).into())
        }
    }
}
