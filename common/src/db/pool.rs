use std::time::Duration;

use deadpool::managed::{
    Object, PoolConfig, PoolError as DpPoolError, Timeouts,
};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Pool as DieselPool;
use diesel_async::pooled_connection::{
    AsyncDieselConnectionManager, PoolError,
};
use serde::{Deserialize, Serialize};

type ConnMgr = AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>;
pub type DbPoolError = DpPoolError<PoolError>;
pub type PoolConnResult = Result<
    Object<AsyncDieselConnectionManager<AsyncPgConnection>>,
    DbPoolError,
>;

pub struct DbPool {
    pool: DieselPool<AsyncPgConnection>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_conns: usize,
    pub connect_timeout: Option<Duration>,
    pub wait_timeout: Option<Duration>,
    pub recycle_timeout: Option<Duration>,
}

impl DbPool {
    pub async fn new(config: DatabaseConfig) -> anyhow::Result<Self> {
        let pool_config = ConnMgr::new(config.url);
        let pool = DieselPool::builder(pool_config)
            .config(PoolConfig {
                max_size: config.max_conns,
                timeouts: Timeouts {
                    create: config.connect_timeout,
                    wait: config.wait_timeout,
                    recycle: config.recycle_timeout,
                },
                ..Default::default()
            })
            .max_size(config.max_conns)
            .build()?;

        Ok(Self { pool })
    }

    pub async fn conn(&self) -> PoolConnResult {
        self.pool.get().await
    }
}
