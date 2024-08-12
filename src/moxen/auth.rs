use base64::prelude::*;
use ring::{
    rand::SystemRandom,
    signature::{Ed25519KeyPair, KeyPair},
};
use rustrict::CensorStr;

use super::config::{MoxenConfig, MoxenCredentials};
use crate::common::MoxenError;

static MIN_USERNAME_LENGTH: usize = 3;

#[derive(Debug)]
pub struct MoxenKeyPair {
    prv_key: Ed25519KeyPair,
}

#[allow(dead_code)]
impl MoxenKeyPair {
    pub fn new(pkcs8_raw: &[u8]) -> Result<Self, MoxenError> {
        let keypair = Ed25519KeyPair::from_pkcs8(pkcs8_raw)
            .map_err(|e| MoxenError::GeneralError(e.to_string()))?;

        Ok(Self { prv_key: keypair })
    }

    pub fn from_private_key(key: &str) -> Result<Self, MoxenError> {
        let document = BASE64_STANDARD
            .decode(key)
            .map_err(|e| MoxenError::GeneralError(e.to_string()))?;

        let keypair = Ed25519KeyPair::from_pkcs8(&document)
            .map_err(|e| MoxenError::GeneralError(e.to_string()))?;

        Ok(Self { prv_key: keypair })
    }

    pub fn public_key_as_string(&self) -> String {
        BASE64_STANDARD.encode(self.prv_key.public_key().as_ref())
    }

    pub fn sign_message(&self, msg: &str) -> String {
        let sig = self.prv_key.sign(msg.as_bytes());
        BASE64_STANDARD.encode(sig.as_ref())
    }
}

pub fn validate_username(name: &str) -> Result<(), MoxenError> {
    if name.len() < MIN_USERNAME_LENGTH {
        return Err(MoxenError::InvalidUsername(
            "username must be at least 3 characters long".to_string(),
        ));
    }

    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(MoxenError::InvalidUsername(
            "invalid characters in username. allowed characters are letters, numbers, and _"
                .to_string(),
        ));
    }

    if name.is_inappropriate() {
        return Err(MoxenError::InvalidUsername(
            "inappropriate username, watch your profanity!".to_string(),
        ));
    }

    Ok(())
}

pub fn generate_keyfile_pair(config: &mut MoxenConfig) -> Result<MoxenKeyPair, MoxenError> {
    match config.credentials {
        Some(_) => Err(MoxenError::ConfigError(
            "credentials already present, you are already registered as someone!".to_string(),
        )),
        None => {
            let rng = SystemRandom::new();
            let document = Ed25519KeyPair::generate_pkcs8(&rng)
                .map_err(|e| MoxenError::GeneralError(e.to_string()))?;

            let keypair = MoxenKeyPair::new(document.as_ref())?;

            let private_key = BASE64_STANDARD.encode(document.as_ref());
            let credentials = MoxenCredentials {
                private_key,
                api_key: None,
            };

            config.credentials = Some(credentials);

            Ok(keypair)
        }
    }

    // This is how to sign and generate
    // let challenge = "hi there";
    // let signed = keypair.sign(challenge.as_bytes());
    // let something = UnparsedPublicKey::new(&ED25519, keypair.public_key().as_ref());
    // if something
    //     .verify(challenge.as_bytes(), signed.as_ref())
    //     .is_err()
    // {
    //     panic!("this isnt valid");
    // }
}
