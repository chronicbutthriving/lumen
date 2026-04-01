//! Database access implementations for the application.

use std::collections::HashMap;

use lumen_common::db::pool::{DatabaseConfig, DbPool, PoolConnResult};
use lumen_uuid_kinds::{UserPasswordUuid, UserProviderUuid, UserUuid};

use crate::{
    models::{UserModel, UserPasswordModel, UserProviderModel},
    repos::{UserPasswordStore, UserProviderStore, UserStore},
};

pub trait AuthStore:
    UserStore + UserPasswordStore + UserProviderStore + Send + Sync + 'static
{
}

pub struct PgStore {
    pool: DbPool,
}

impl PgStore {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn open(cfg: DatabaseConfig) -> anyhow::Result<Self> {
        let pool = DbPool::new(cfg).await?;
        Ok(Self { pool })
    }

    pub async fn conn(&self) -> PoolConnResult {
        self.pool.conn().await
    }
}

pub struct MockStore {
    pub users: HashMap<UserUuid, UserModel>,
    pub user_passwords: HashMap<UserPasswordUuid, UserPasswordModel>,
    pub user_providers: HashMap<UserProviderUuid, UserProviderModel>,
}

impl MockStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            user_passwords: HashMap::new(),
            user_providers: HashMap::new(),
        }
    }
}

impl Default for MockStore {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AuthStore for T where
    T: UserStore
        + UserPasswordStore
        + UserProviderStore
        + Send
        + Sync
        + 'static
{
}
