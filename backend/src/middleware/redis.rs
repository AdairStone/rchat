
use anyhow::Result;
use deadpool_redis::{redis::AsyncCommands, Pool, Runtime};
use lazy_static::lazy_static;

use crate::app_config::SETTINGS;

lazy_static! {
    pub static ref REDIS_MANAGER: RedisManager = {
        let cfg = deadpool_redis::Config::from_url(&SETTINGS.redis_url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).expect("Failed to create Redis pool");
        RedisManager::new(pool)
    };
}

pub struct RedisManager {
    pool: Pool,
}

impl RedisManager {
    pub fn new(pool: Pool) -> Self {
        RedisManager { pool }
    }

    pub async fn set<T: redis::ToRedisArgs + Send + Sync>(&self, key: &str, value: T) -> Result<()> {
        let mut conn = self.pool.get().await?;
        conn.set(key, value).await?;
        Ok(())
    }

    pub async fn get<T: redis::FromRedisValue>(&self, key: &str) -> Result<Option<T>> {
        let mut conn = self.pool.get().await?;
        let result = conn.get(key).await?;
        Ok(result)
    }

    pub async fn del(&self, key: &str) -> Result<()> {
        let mut conn = self.pool.get().await?;
        conn.del(key).await?;
        Ok(())
    }

    pub async fn expire(&self, key: &str, seconds: usize) -> Result<bool> {
        let mut conn = self.pool.get().await?;
        let result = conn.expire(key, seconds as i64).await?;
        Ok(result)
    }

    pub async fn incr(&self, key: &str, increment: isize) -> Result<i64> {
        let mut conn = self.pool.get().await?;
        let result = conn.incr(key, increment).await?;
        Ok(result)
    }

}
