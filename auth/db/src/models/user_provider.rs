use chrono::{DateTime, Utc};
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use diesel_enums::pg_enum;
use lumen_common::db::{DbTypedUuid, schema};
use lumen_uuid_kinds::{
    UserKind, UserProviderKind, UserProviderUuid, UserUuid,
};
use serde::{Deserialize, Serialize};

#[pg_enum]
#[db(
    sql_type = schema::sql_types::AuthUserProviderKind,
    skip_test
)]
#[derive(Serialize, Deserialize)]
pub enum AuthUserProviderKind {
    Google,
    Github,
}

#[derive(
    Debug, Clone, Queryable, Insertable, Selectable, Serialize, Deserialize,
)]
#[diesel(table_name = schema::auth_user_provider)]
pub struct UserProviderModel {
    pub id: DbTypedUuid<UserProviderKind>,
    pub time_created: DateTime<Utc>,
    pub time_modified: DateTime<Utc>,
    pub time_deleted: Option<DateTime<Utc>>,
    pub provider_kind: AuthUserProviderKind,
    pub provider_id: String,
    pub user_id: DbTypedUuid<UserKind>,
}

impl UserProviderModel {
    pub fn new(
        provider_kind: AuthUserProviderKind,
        provider_id: String,
        user_id: UserUuid,
    ) -> Self {
        Self::new_with_id(
            UserProviderUuid::new_v4(),
            provider_kind,
            provider_id,
            user_id,
        )
    }

    pub fn new_with_id(
        id: UserProviderUuid,
        provider_kind: AuthUserProviderKind,
        provider_id: String,
        user_id: UserUuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: DbTypedUuid::from(id),
            time_created: now,
            time_modified: now,
            time_deleted: None,
            provider_kind,
            provider_id,
            user_id: DbTypedUuid::from(user_id),
        }
    }

    pub fn mark_deleted(&mut self) {
        self.time_deleted = Some(Utc::now());
    }
}
