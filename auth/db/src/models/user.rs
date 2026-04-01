use chrono::{DateTime, Utc};
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use lumen_common::db::{DbTypedUuid, schema};
use lumen_uuid_kinds::{UserKind, UserUuid};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Queryable, Insertable, Selectable, Serialize, Deserialize,
)]
#[diesel(table_name = schema::auth_user)]
pub struct UserModel {
    pub id: DbTypedUuid<UserKind>,
    pub time_created: DateTime<Utc>,
    pub time_modified: DateTime<Utc>,
    pub time_deleted: Option<DateTime<Utc>>,
    pub email: String,
}

impl UserModel {
    pub fn new(email: String) -> Self {
        Self::new_with_id(UserUuid::new_v4(), email)
    }

    pub fn new_with_id(id: UserUuid, email: String) -> Self {
        let now = Utc::now();
        Self {
            id: DbTypedUuid::from(id),
            time_created: now,
            time_modified: now,
            time_deleted: None,
            email,
        }
    }

    pub fn mark_deleted(&mut self) {
        self.time_deleted = Some(Utc::now());
    }
}
