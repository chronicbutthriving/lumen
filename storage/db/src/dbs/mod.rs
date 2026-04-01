//! Database access implementations for the application.

use std::collections::HashMap;

use lumen_common::db::pool::{DatabaseConfig, DbPool, PoolConnResult};
use lumen_uuid_kinds::ObjectUuid;

use crate::{models::ObjectModel, repos::ObjectStore};

pub trait StorageStore: ObjectStore + Send + Sync + 'static {}

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
    pub objects: HashMap<ObjectUuid, ObjectModel>,
}

impl MockStore {
    pub fn new() -> Self {
        Self { objects: HashMap::new() }
    }
}

impl Default for MockStore {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> StorageStore for T where T: ObjectStore + Send + Sync + 'static {}
