use chrono::{DateTime, Utc};
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use lumen_common::db::{DbTypedUuid, schema};
use lumen_uuid_kinds::{
    UserKind, UserPasswordKind, UserPasswordUuid, UserUuid,
};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Queryable, Insertable, Selectable, Serialize, Deserialize,
)]
#[diesel(table_name = schema::auth_user_password)]
pub struct UserPasswordModel {
    pub id: DbTypedUuid<UserPasswordKind>,
    pub time_created: DateTime<Utc>,
    pub password_hash: String,
    pub user_id: DbTypedUuid<UserKind>,
}

impl UserPasswordModel {
    pub fn new(user_id: UserUuid, password_hash: String) -> Self {
        Self::new_with_id(UserPasswordUuid::new_v4(), user_id, password_hash)
    }

    pub fn new_with_id(
        id: UserPasswordUuid,
        user_id: UserUuid,
        password_hash: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: DbTypedUuid::from(id),
            time_created: now,
            password_hash,
            user_id: DbTypedUuid::from(user_id),
        }
    }
}
