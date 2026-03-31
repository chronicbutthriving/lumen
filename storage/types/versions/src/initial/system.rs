use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

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
