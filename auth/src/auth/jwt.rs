use thiserror::Error;

use crate::auth::SigningKeyError;

#[derive(Debug, Error)]
pub enum JwtSignerError {
    #[error("Failed to encode header")]
    Header(serde_json::Error),
    #[error("Failed to generate signer: {0}")]
    InvalidKey(SigningKeyError),
    #[error("Failed to serialize claims: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Failed to generate signature: {0}")]
    Signature(SigningKeyError),
}
