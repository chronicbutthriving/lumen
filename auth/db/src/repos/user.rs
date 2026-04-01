use anyhow::anyhow;
use async_trait::async_trait;
use lumen_common::{
    api::external::ResourceType,
    db::{
        PaginationParams,
        error::{StoreError, StoreResult},
        schema::{self, auth_user},
    },
};
use lumen_uuid_kinds::{GenericUuid, UserUuid};

use crate::{
    dbs::{MockStore, PgStore},
    models::UserModel,
};

#[derive(Debug, Default)]
pub struct UserFilter {
    pub ids: Option<Vec<UserUuid>>,
    pub emails: Option<Vec<String>>,
    pub deleted: bool,
}

impl UserFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_ids(mut self, ids: Vec<UserUuid>) -> Self {
        self.ids = Some(ids);
        self
    }

    pub fn with_emails(mut self, emails: Vec<String>) -> Self {
        self.emails = Some(emails);
        self
    }

    pub fn with_deleted(mut self, deleted: bool) -> Self {
        self.deleted = deleted;
        self
    }
}

#[async_trait]
pub trait UserStore {
    /// Gets the user with the specified ID, returning None if it doesn't exist.
    async fn get(&self, id: UserUuid) -> StoreResult<Option<UserModel>> {
        self.list(
            UserFilter::new().with_ids(vec![id]),
            PaginationParams::default().with_limit(1),
        )
        .await
        .map(|users| users.first().cloned())
    }

    /// Gets the user with the specified ID, returning an error if it doesn't exist.
    async fn must_get(&self, id: UserUuid) -> StoreResult<UserModel> {
        self.get(id)
            .await?
            .ok_or(StoreError::NoResults { resource_type: ResourceType::User })
    }

    /// Lists users matching the specified filter criteria.
    async fn list(
        &self,
        filter: UserFilter,
        pagination: PaginationParams,
    ) -> StoreResult<Vec<UserModel>>;
}

#[async_trait]
impl UserStore for MockStore {
    async fn list(
        &self,
        filter: UserFilter,
        pagination: PaginationParams,
    ) -> StoreResult<Vec<UserModel>> {
        let mut results = self.users.values().cloned().collect::<Vec<_>>();

        if let Some(ids) = filter.ids {
            results.retain(|user| ids.contains(&user.id.into()));
        }

        if let Some(emails) = filter.emails {
            results.retain(|user| emails.contains(&user.email));
        }

        if !filter.deleted {
            results.retain(|user| user.time_deleted.is_none());
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
impl UserStore for PgStore {
    async fn list(
        &self,
        filter: UserFilter,
        pagination: PaginationParams,
    ) -> StoreResult<Vec<UserModel>> {
        use diesel::{ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;

        let mut query = schema::auth_user::table.into_boxed();

        let UserFilter { ids, emails, deleted } = filter;

        if let Some(ids) = ids {
            query = query.filter(
                auth_user::id
                    .eq_any(ids.into_iter().map(|id| id.into_untyped_uuid())),
            );
        }

        if let Some(emails) = emails {
            query = query.filter(auth_user::email.eq_any(emails));
        }

        if !deleted {
            query = query.filter(auth_user::time_deleted.is_null());
        }

        let mut conn = self.conn().await?;

        query = query
            .offset(pagination.offset.unwrap_or(0) as i64)
            .limit(pagination.limit.unwrap_or(50) as i64);

        let results: Vec<UserModel> =
            query.get_results(&mut conn).await.map_err(|e| {
                StoreError::Internal(anyhow!("Failed to query users: {e}"))
            })?;

        Ok(results)
    }
}
