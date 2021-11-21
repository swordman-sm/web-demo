use log::info;
// use redis::AsyncCommands;
use crate::constant::enums::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::constant::enums::CacheKey;

///缓存服务
pub struct CacheService {
    pub client: redis::Client,
}

impl CacheService {
    pub fn new(url: &str) -> Self {
        let client = redis::Client::open(url).unwrap();
        info!("connect redis success!");
        Self { client }
    }

    pub async fn put_json<T>(&self, k: &CacheKey, v: &T) -> Result<String>
        where
            T: Serialize,
    {
        let mut conn = self.client.get_async_connection().await?;
        let data = serde_json::to_string(v)?;
        let r: String = redis::cmd("SET")
            .arg(k.get_key())
            .arg(data.as_str())
            .query_async(&mut conn)
            .await?;
        Ok(r)
    }

    pub async fn get_json<T>(&self, k: &CacheKey) -> Result<T>
        where
            T: DeserializeOwned,
    {
        let mut conn = self.client.get_async_connection().await?;
        let r: String = redis::cmd("GET")
            .arg(k.get_key())
            .query_async(&mut conn)
            .await?;
        let data: T = serde_json::from_str(r.as_str())?;
        Ok(data)
    }

    pub async fn put_string(&self, k: &CacheKey, v: &str) -> Result<String> {
        let mut conn = self.client.get_async_connection().await?;
        let r: String = redis::cmd("SET")
            .arg(k.get_key())
            .arg(v)
            .query_async(&mut conn)
            .await?;
        Ok(r)
    }

    pub async fn get_string(&self, k: &CacheKey) -> Result<Option<String>> {
        let mut conn = self.client.get_async_connection().await?;
        let r: Option<String> = redis::cmd("GET")
            .arg(k.get_key())
            .query_async(&mut conn)
            .await?;
        Ok(r)
    }

    pub async fn expire(&self, k: &CacheKey, seconds: i32) -> Result<i32> {
        let mut conn = self.client.get_async_connection().await?;
        let r: i32 = redis::cmd("EXPIRE")
            .arg(k.get_key())
            .arg(seconds)
            .query_async(&mut conn)
            .await?;
        Ok(r)
    }

    pub async fn del(&self, k: &CacheKey) -> Result<i32> {
        let mut conn = self.client.get_async_connection().await?;
        let r: i32 = redis::cmd("DEL")
            .arg(k.get_key())
            .query_async(&mut conn)
            .await?;
        Ok(r)
    }
}
