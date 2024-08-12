use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::common::MoxenError;

#[derive(Debug, Serialize, Deserialize)]
pub struct MoxenConfig {
    #[serde(skip)]
    pub file_path: PathBuf,

    pub credentials: Option<MoxenCredentials>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MoxenCredentials {
    pub username: String,
    pub private_key: String,
    pub api_key: Option<String>,
}

impl MoxenConfig {
    pub fn load(moxen_dir: impl AsRef<Path>) -> Result<Self, MoxenError> {
        let cfg_file = moxen_dir.as_ref().join("config");
        if !cfg_file.exists() {
            let cfg = MoxenConfig {
                file_path: cfg_file,
                credentials: None,
            };

            cfg.write()?;
            return Ok(cfg);
        }

        let contents =
            std::fs::read_to_string(&cfg_file).map_err(|e| MoxenError::LoadError(e.to_string()))?;
        let mut cfg: MoxenConfig =
            toml::from_str(&contents).map_err(|e| MoxenError::ConfigError(e.to_string()))?;

        cfg.file_path = cfg_file;

        Ok(cfg)
    }

    pub fn write(&self) -> Result<(), MoxenError> {
        let contents =
            toml::to_string_pretty(&self).map_err(|e| MoxenError::ConfigError(e.to_string()))?;

        std::fs::write(&self.file_path, contents)
            .map_err(|e| MoxenError::GeneralError(e.to_string()))
    }
}
