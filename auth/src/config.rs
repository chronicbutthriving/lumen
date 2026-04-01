use std::path::PathBuf;

use base64::{Engine,prelude::BASE64_URL_SAFE_NO_PAD as URL_SAFE_NO_PAD};
use jsonwebtoken::jwk::{AlgorithmParameters, CommonParameters, EllipticCurve, Jwk, KeyAlgorithm, OctetKeyPairParameters, OctetKeyPairType, PublicKeyUse};
use lumen_common::config::StringParam;
use secrecy::ExposeSecret;
use serde::Deserialize;
use ed25519_dalek::{VerifyingKey, pkcs8::DecodePublicKey};

use crate::auth::{SigningKeyError, jwt::JwtSignerError};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database: lumen_common::db::pool::DatabaseConfig,
    pub log: dropshot::ConfigLogging,
    pub dropshot: dropshot::ConfigDropshot,
    pub jwt: JwtConfig,
    pub keys: Vec<AsymmetricKey>,
}

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    pub default_expiration: i64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            default_expiration: 3600,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AsymmetricKey {
    LocalVerifier {
        kid: String,
        public: StringParam,
    },
    LocalSigner {
        kid: String,
        private: StringParam,
    }
}

impl AsymmetricKey {
    pub fn kid(&self) -> &str {
        match self {
            Self::LocalVerifier { kid, .. } => kid,
            Self::LocalSigner { kid, .. } => kid,
        }
    }

    pub fn resolve_jwk(&self, path: Option<PathBuf>) -> Result<Jwk, JwtSignerError> {
        let key_id = self.kid();
        let public_key = self.public_key(path).map_err(JwtSignerError::InvalidKey)?;

        Ok(Jwk {
            common: CommonParameters {
                public_key_use: Some(PublicKeyUse::Signature),
                key_operations: None,
                key_algorithm: Some(KeyAlgorithm::EdDSA),
                key_id: Some(key_id.to_string()),
                x509_chain: None,
                x509_sha1_fingerprint: None,
                x509_sha256_fingerprint: None,
                x509_url: None,
            },
            algorithm: AlgorithmParameters::OctetKeyPair(OctetKeyPairParameters {
                key_type: OctetKeyPairType::OctetKeyPair,
                curve: EllipticCurve::Ed25519,
                x: URL_SAFE_NO_PAD.encode(public_key.to_bytes()),
            }),
        })
    }

    fn public_key(&self, path: Option<PathBuf>) -> Result<VerifyingKey, SigningKeyError> {
        Ok(match self {
            AsymmetricKey::LocalVerifier { public, .. } => {
                VerifyingKey::from_public_key_pem(public.resolve(path)?.expose_secret())?
            }
            _ => Err(SigningKeyError::KeyDoesNotSupportFunction)?,
        })
    }
}
