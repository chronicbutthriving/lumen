//! Data structures and related facilities for representing resources in the API
//!
//! This includes all representations over the wire for both the external and
//! internal APIs. The contents here are all HTTP-agnostic.

pub mod error;

use chrono::{DateTime, Utc};
use error::*;
use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use uuid::Uuid;

/// Result of a create operation for the specified type
pub type CreateResult<T> = Result<T, Error>;
/// Result of a delete operation for the specified type
pub type DeleteResult<T> = Result<T, Error>;
/// Result of a list operation that returns a vector
pub type ListResult<T> = Result<Vec<T>, Error>;
/// Result of a get operation for the specified type
pub type GetResult<T> = Result<T, Error>;
/// Result of an update operation for the specified type
pub type UpdateResult<T> = Result<T, Error>;
/// Result of an optional get operation for the specified type
pub type OptionalGetResult<T> = Result<Option<T>, Error>;

#[derive(
    Clone,
    Copy,
    Debug,
    DeserializeFromStr,
    Display,
    Eq,
    FromStr,
    Ord,
    PartialEq,
    PartialOrd,
    SerializeDisplay,
)]
#[display(style = "kebab-case")]
pub enum ResourceType {
    User,
    UserProvider,
    UserPassword,
    Object,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct IdentityMetadata {
    pub id: Uuid,
    pub time_created: DateTime<Utc>,
    pub time_updated: DateTime<Utc>,
}
