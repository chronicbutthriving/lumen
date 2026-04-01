use chrono::{DateTime, Utc};
use lumen_uuid_kinds::UserUuid;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct User {
    pub id: UserUuid,
    pub time_created: DateTime<Utc>,
    pub time_modified: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_deleted: Option<DateTime<Utc>>,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum UserProviderKind {
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserProvider {
    pub id: UserUuid,
    pub time_created: DateTime<Utc>,
    pub time_modified: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_deleted: Option<DateTime<Utc>>,
    pub provider_kind: UserProviderKind,
    pub provider_id: String,
    pub user_id: UserUuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InviteUserRequest {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InviteUserResponse {
    pub invite_url: String,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct UserPathParams {
    pub user_id: UserUuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateUserPasswordRequest {
    pub password: String,
    pub password_confirm: String,
}
