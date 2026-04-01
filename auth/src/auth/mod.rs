use lumen_common::config::ParamResolutionError;
use thiserror::Error;

pub mod jwt;

#[derive(Debug, Error)]
pub enum SigningKeyError {
    #[error("Failed to immediately verify generated signature")]
    GeneratedInvalidSignature,
    #[error("Failed to parse public key: {0}")]
    InvalidPublicKey(#[from] ed25519_dalek::pkcs8::spki::Error),
    #[error("Key does not support the requested function")]
    KeyDoesNotSupportFunction,
    #[error("Failed to resolve parameter")]
    Param(#[from] ParamResolutionError),
    #[error("Invalid signature: {0}")]
    Signature(#[from] ed25519_dalek::SignatureError),
}
