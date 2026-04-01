use anyhow::anyhow;
use async_trait::async_trait;
use lumen_common::{
    api::external::ResourceType,
    db::{
        PaginationParams,
        error::{StoreError, StoreResult},
        schema::{self, auth_user_password},
    },
};
use lumen_uuid_kinds::{GenericUuid, UserPasswordUuid, UserUuid};

use crate::{
    dbs::{MockStore, PgStore},
    models::UserPasswordModel,
};

#[derive(Debug, Default)]
pub struct UserPasswordFilter {
    pub ids: Option<Vec<UserPasswordUuid>>,
    pub user_ids: Option<Vec<UserUuid>>,
}

impl UserPasswordFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_ids(mut self, ids: Vec<UserPasswordUuid>) -> Self {
        self.ids = Some(ids);
        self
    }

    pub fn with_user_ids(mut self, user_ids: Vec<UserUuid>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }
}

#[async_trait]
pub trait UserPasswordStore {
    /// Gets the user password with the specified ID, returning None if it doesn't exist.
    async fn get(
        &self,
        id: UserPasswordUuid,
    ) -> StoreResult<Option<UserPasswordModel>> {
        self.list(
            UserPasswordFilter::new().with_ids(vec![id]),
            PaginationParams::default().with_limit(1),
        )
        .await
        .map(|passwords| passwords.first().cloned())
    }

    /// Gets the user password with the specified ID, returning an error if it doesn't exist.
    async fn must_get(
        &self,
        id: UserPasswordUuid,
    ) -> StoreResult<UserPasswordModel> {
        self.get(id).await?.ok_or(StoreError::NoResults {
            resource_type: ResourceType::UserPassword,
        })
    }

    /// Lists user passwords matching the specified filter criteria.
    async fn list(
        &self,
        filter: UserPasswordFilter,
        pagination: PaginationParams,
    ) -> StoreResult<Vec<UserPasswordModel>>;
}

#[async_trait]
impl UserPasswordStore for MockStore {
    async fn list(
        &self,
        filter: UserPasswordFilter,
        pagination: PaginationParams,
    ) -> StoreResult<Vec<UserPasswordModel>> {
        let mut results =
            self.user_passwords.values().cloned().collect::<Vec<_>>();

        if let Some(ids) = filter.ids {
            results.retain(|password| ids.contains(&password.id.into()));
        }

        if let Some(user_ids) = filter.user_ids {
            results
                .retain(|password| user_ids.contains(&password.user_id.into()));
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
impl UserPasswordStore for PgStore {
    async fn list(
        &self,
        filter: UserPasswordFilter,
        pagination: PaginationParams,
    ) -> StoreResult<Vec<UserPasswordModel>> {
        use diesel::{ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;

        let mut query = schema::auth_user_password::table.into_boxed();

        let UserPasswordFilter { ids, user_ids } = filter;

        if let Some(ids) = ids {
            query = query.filter(
                auth_user_password::id
                    .eq_any(ids.into_iter().map(|id| id.into_untyped_uuid())),
            );
        }

        if let Some(user_ids) = user_ids {
            query =
                query.filter(auth_user_password::user_id.eq_any(
                    user_ids.into_iter().map(|id| id.into_untyped_uuid()),
                ));
        }

        let mut conn = self.conn().await?;

        query = query
            .offset(pagination.offset.unwrap_or(0) as i64)
            .limit(pagination.limit.unwrap_or(50) as i64);

        let results: Vec<UserPasswordModel> =
            query.get_results(&mut conn).await.map_err(|e| {
                StoreError::Internal(anyhow!(
                    "Failed to query user passwords: {e}"
                ))
            })?;

        Ok(results)
    }
}
