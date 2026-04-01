use anyhow::anyhow;
use async_trait::async_trait;
use lumen_common::{
    api::external::ResourceType,
    db::{
        PaginationParams,
        error::{StoreError, StoreResult},
        schema::{self, auth_user_provider},
    },
};
use lumen_uuid_kinds::{GenericUuid, UserProviderUuid, UserUuid};

use crate::{
    dbs::{MockStore, PgStore},
    models::{AuthUserProviderKind, UserProviderModel},
};

#[derive(Debug, Default)]
pub struct UserProviderFilter {
    pub ids: Option<Vec<UserProviderUuid>>,
    pub user_ids: Option<Vec<UserUuid>>,
    pub provider_kinds: Option<Vec<AuthUserProviderKind>>,
    pub deleted: bool,
}

impl UserProviderFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_ids(mut self, ids: Vec<UserProviderUuid>) -> Self {
        self.ids = Some(ids);
        self
    }

    pub fn with_user_ids(mut self, user_ids: Vec<UserUuid>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn with_provider_kinds(
        mut self,
        provider_kinds: Vec<AuthUserProviderKind>,
    ) -> Self {
        self.provider_kinds = Some(provider_kinds);
        self
    }

    pub fn with_deleted(mut self, deleted: bool) -> Self {
        self.deleted = deleted;
        self
    }
}

#[async_trait]
pub trait UserProviderStore {
    /// Gets the user provider with the specified ID, returning None if it doesn't exist.
    async fn get(
        &self,
        id: UserProviderUuid,
    ) -> StoreResult<Option<UserProviderModel>> {
        self.list(
            UserProviderFilter::new().with_ids(vec![id]),
            PaginationParams::default().with_limit(1),
        )
        .await
        .map(|providers| providers.first().cloned())
    }

    /// Gets the user provider with the specified ID, returning an error if it doesn't exist.
    async fn must_get(
        &self,
        id: UserProviderUuid,
    ) -> StoreResult<UserProviderModel> {
        self.get(id).await?.ok_or(StoreError::NoResults {
            resource_type: ResourceType::UserProvider,
        })
    }

    /// Lists user providers matching the specified filter criteria.
    async fn list(
        &self,
        filter: UserProviderFilter,
        pagination: PaginationParams,
    ) -> StoreResult<Vec<UserProviderModel>>;
}

#[async_trait]
impl UserProviderStore for MockStore {
    async fn list(
        &self,
        filter: UserProviderFilter,
        pagination: PaginationParams,
    ) -> StoreResult<Vec<UserProviderModel>> {
        let mut results =
            self.user_providers.values().cloned().collect::<Vec<_>>();

        if let Some(ids) = filter.ids {
            results.retain(|provider| ids.contains(&provider.id.into()));
        }

        if let Some(user_ids) = filter.user_ids {
            results
                .retain(|provider| user_ids.contains(&provider.user_id.into()));
        }

        if let Some(provider_kinds) = filter.provider_kinds {
            results.retain(|provider| {
                provider_kinds.contains(&provider.provider_kind)
            });
        }

        if !filter.deleted {
            results.retain(|provider| provider.time_deleted.is_none());
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
impl UserProviderStore for PgStore {
    async fn list(
        &self,
        filter: UserProviderFilter,
        pagination: PaginationParams,
    ) -> StoreResult<Vec<UserProviderModel>> {
        use diesel::{ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;

        let mut query = schema::auth_user_provider::table.into_boxed();

        let UserProviderFilter { ids, user_ids, provider_kinds, deleted } =
            filter;

        if let Some(ids) = ids {
            query = query.filter(
                auth_user_provider::id
                    .eq_any(ids.into_iter().map(|id| id.into_untyped_uuid())),
            );
        }

        if let Some(user_ids) = user_ids {
            query =
                query.filter(auth_user_provider::user_id.eq_any(
                    user_ids.into_iter().map(|id| id.into_untyped_uuid()),
                ));
        }

        if let Some(provider_kinds) = provider_kinds {
            query = query.filter(
                auth_user_provider::provider_kind.eq_any(provider_kinds),
            );
        }

        if !deleted {
            query = query.filter(auth_user_provider::time_deleted.is_null());
        }

        let mut conn = self.conn().await?;

        query = query
            .offset(pagination.offset.unwrap_or(0) as i64)
            .limit(pagination.limit.unwrap_or(50) as i64);

        let results: Vec<UserProviderModel> =
            query.get_results(&mut conn).await.map_err(|e| {
                StoreError::Internal(anyhow!(
                    "Failed to query user providers: {e}"
                ))
            })?;

        Ok(results)
    }
}
