use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Jwk {
    pub kty: String,
    pub kid: String,
    #[serde(rename = "use")]
    pub use_: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JwksResponse {
    /// The list of JSON Web Keys (JWKs) that can be used to verify the
    /// signatures of JSON Web Tokens (JWTs) issued by the authentication
    /// service.
    pub keys: Vec<Jwk>,
}

// SYSTEM HEALTH

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum PingStatus {
    Ok,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Ping {
    /// Whether the storage API is reachable. Will always be Ok if the endpoint
    /// returns anything at all.
    pub status: PingStatus,
}
