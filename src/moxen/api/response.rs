use serde::Deserialize;

#[derive(Deserialize)]
pub struct DownloadPackageResponse {
    pub manifest: String,
    pub package: Vec<u8>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct UserRegisterResponse {
    pub api_key: String,
    pub recovery_codes: Vec<String>,
    pub error: Option<String>,
}
