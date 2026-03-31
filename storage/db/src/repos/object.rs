use anyhow::anyhow;
use async_trait::async_trait;
use lumen_common::{api::external::ResourceType, db::{PaginationParams, schema::{self, storage_object}}};
use lumen_uuid_kinds::{GenericUuid, ObjectUuid};

use crate::{
    dbs::{MockStore, PgStore},
    error::{StoreError, StoreResult},
    models::{ObjectModel, StorageProviderKind},
};

#[derive(Debug, Default)]
pub struct ObjectFilter {
    pub ids: Option<Vec<ObjectUuid>>,
    pub provider_kinds: Option<Vec<StorageProviderKind>>,
    pub deleted: bool,
}

impl ObjectFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_ids(mut self, ids: Vec<ObjectUuid>) -> Self {
        self.ids = Some(ids);
        self
    }

    pub fn with_provider_kinds(mut self, provider_kinds: Vec<StorageProviderKind>) -> Self {
        self.provider_kinds = Some(provider_kinds);
        self
    }

    pub fn with_deleted(mut self, deleted: bool) -> Self {
        self.deleted = deleted;
        self
    }
}

#[async_trait]
pub trait ObjectStore {
    /// Gets the object with the specified ID, returning None if it doesn't
    /// exist.
    async fn get(&self, id: ObjectUuid) -> StoreResult<Option<ObjectModel>> {
        self.list(
            ObjectFilter::new().with_ids(vec![id]),
            PaginationParams::default().with_limit(1),
        )
        .await
        .map(|obj| obj.first().cloned())
    }

    /// Gets the object with the specified ID, returning an error if it doesn't
    /// exist.
    async fn must_get(&self, id: ObjectUuid) -> StoreResult<ObjectModel> {
        self.get(id).await?.ok_or(StoreError::NoResults {
            resource_type: ResourceType::Object,
        })
    }

    /// Lists objects matching the specified filter criteria.
    async fn list(
        &self,
        filter: ObjectFilter,
        pagination: PaginationParams,
    ) -> StoreResult<Vec<ObjectModel>>;
}

#[async_trait]
impl ObjectStore for MockStore {
    async fn list(
        &self,
        filter: ObjectFilter,
        pagination: PaginationParams,
    ) -> StoreResult<Vec<ObjectModel>> {
        let mut results = self.objects.values().cloned().collect::<Vec<_>>();

        if let Some(ids) = filter.ids {
            results.retain(|obj| ids.contains(&obj.id.into()));
        }

        if let Some(provider_kinds) = filter.provider_kinds {
            results.retain(|obj| provider_kinds.contains(&obj.provider_kind));
        }

        if !filter.deleted {
            results.retain(|obj| obj.time_deleted.is_none());
        }

        if let Some(offset) = pagination.offset {
            results.drain(0..offset as usize);
        }

        if let Some(limit) = pagination.limit {
            results.truncate(limit as usize);
        }

        Ok(results)
    }
}

#[async_trait]
impl ObjectStore for PgStore {
    async fn list(
        &self,
        filter: ObjectFilter,
        pagination: PaginationParams,
    ) -> StoreResult<Vec<ObjectModel>> {
        use diesel::{ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;

        let mut query = schema::storage_object::table.into_boxed();

        let ObjectFilter {
            ids,
            provider_kinds,
            deleted,
        } = filter;

        if let Some(ids) = ids {
            query = query.filter(storage_object::id.eq_any(ids.into_iter().map(|id| id.into_untyped_uuid())));
        }

        if let Some(provider_kinds) = provider_kinds {
            query = query.filter(storage_object::provider_kind.eq_any(provider_kinds));
        }

        if !deleted {
            query = query.filter(storage_object::time_deleted.is_null());
        }

        let mut conn = self.conn().await?;

        query = query
            .offset(pagination.offset.unwrap_or(0) as i64)
            .limit(pagination.limit.unwrap_or(50) as i64);

        let results: Vec<ObjectModel> = query
            .get_results(&mut conn)
            .await
            .map_err(|e| StoreError::Internal(anyhow!("Failed to query objects: {e}")))?;

        Ok(results)
    }
}
