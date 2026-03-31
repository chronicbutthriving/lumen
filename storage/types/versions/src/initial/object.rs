use chrono::{DateTime, Utc};
use lumen_uuid_kinds::ObjectUuid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// What kind of storage provider is being used for this object.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum StorageProviderKind {
    /// The object is stored on the local filesystem of the storage service.
    Local,
}

/// A storage object represents a file or other piece of data stored in the
/// system.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StorageObject {
    /// The unique identifier for this object.
    pub id: ObjectUuid,
    /// The time at which this object was created.
    pub time_created: DateTime<Utc>,
    /// The time at which this object was last modified.
    pub time_modified: DateTime<Utc>,
    /// The time at which this object was deleted, if it has been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_deleted: Option<DateTime<Utc>>,
    /// The kind of storage provider being used for this object.
    pub provider_kind: StorageProviderKind,
    /// The path or identifier used by the storage provider to locate this
    /// object.
    pub provider_path: String,
    /// The MIME type of the object, if known.
    pub mime_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetObjectParams {
    pub id: ObjectUuid,
}
