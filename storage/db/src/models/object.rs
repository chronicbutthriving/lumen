use chrono::{DateTime, Utc};
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use diesel_enums::pg_enum;
use lumen_common::db::{DbTypedUuid, schema};
use lumen_storage_types_versions::v1::object;
use lumen_uuid_kinds::{ObjectKind, ObjectUuid};
use serde::{Deserialize, Serialize};

#[pg_enum]
#[db(
    sql_type = schema::sql_types::StorageProviderKind,
    skip_test
)]
#[derive(Serialize, Deserialize)]
pub enum StorageProviderKind {
    Local,
}

#[derive(
    Debug, Clone, Queryable, Insertable, Selectable, Serialize, Deserialize,
)]
#[diesel(table_name = schema::storage_object)]
pub struct ObjectModel {
    pub id: DbTypedUuid<ObjectKind>,
    pub time_created: DateTime<Utc>,
    pub time_modified: DateTime<Utc>,
    pub time_deleted: Option<DateTime<Utc>>,
    pub provider_kind: StorageProviderKind,
    pub provider_path: String,
    pub mime_type: String,
}

impl ObjectModel {
    pub fn new(
        provider_kind: StorageProviderKind,
        provider_path: String,
        mime_type: String,
    ) -> Self {
        Self::new_with_id(
            ObjectUuid::new_v4(),
            provider_kind,
            provider_path,
            mime_type,
        )
    }

    pub fn new_with_id(
        id: ObjectUuid,
        provider_kind: StorageProviderKind,
        provider_path: String,
        mime_type: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: DbTypedUuid::from(id),
            time_created: now,
            time_modified: now,
            time_deleted: None,
            provider_kind,
            provider_path,
            mime_type,
        }
    }

    pub fn mark_deleted(&mut self) {
        self.time_deleted = Some(Utc::now());
    }
}

impl From<ObjectModel> for object::StorageObject {
    fn from(model: ObjectModel) -> Self {
        Self {
            id: model.id.into(),
            time_created: model.time_created,
            time_modified: model.time_modified,
            time_deleted: model.time_deleted,
            provider_kind: match model.provider_kind {
                StorageProviderKind::Local => {
                    object::StorageProviderKind::Local
                }
            },
            provider_path: model.provider_path,
            mime_type: model.mime_type,
        }
    }
}
