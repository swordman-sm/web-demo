use log::info;
use redis::AsyncCommands;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::constant::enums::Result;
use crate::constant::enums::CacheKey;

pub struct RedisService {
    pub client: redis::Client,
}

impl RedisService {
    //创建redis连接
    pub fn new(url: &str) -> Self {
        let client = redis::Client::open(url).unwrap();
        info!("redis connect successfully!");
        Self { client }
    }

    pub async fn set_json_value<T>(&self, key: &CacheKey, v: &T) -> Result<String> where T: Serialize {
        let mut conn = self.client.get_async_connection().await?;
        let json = serde_json::to_string(v)?;
        let r: String = redis::cmd("SET").arg(key.get_key())
            .arg(json.as_str()).query_async(&mut conn).await?;
        Ok(r)
    }

    pub async fn get_json_value<T>(&self, k: &CacheKey) -> Result<T> where T: DeserializeOwned {
        let mut conn = self.client.get_async_connection().await?;
        let result: String = redis::cmd("GET").arg(k.get_key()).query_async(&mut conn).await?;
        let data: T = serde_json::from_str(result.as_str());
        Ok(data)
    }

    pub async fn set_string_value(&self, k: &CacheKey, v: &str) -> Result<String> {
        let mut conn = self.client.get_async_connection().await?;
        let result: String = redis::cmd("SET").arg(k.get_key()).arg(v.to_owned())
            .query_async(&mut conn).await?;
        Ok(result)
    }
    ///取出来的值有可能不存在所以使用Option类型接
    pub async fn get_string_value(&self, k: &CacheKey) -> Result<Option<String>> {
        let mut conn = self.client.get_async_connection().await?;
        let result: Option<String> = redis::cmd("GET").arg(k.get_key())
            .query_async(&mut conn).await?;
        Ok(result)
    }

    pub async fn expire(&self, k: &CacheKey, lifetime: i32) -> Result<i32> {
        let mut conn = self.client.get_async_connection().await?;
        let result: i32 = redis::cmd("EXPIRE").arg(k.get_key())
            .arg(lifetime).query_async(&mut conn).await?;
        Ok(result)
    }

    pub async fn del(&self, k: &CacheKey) -> Result<i32> {
        let mut conn = self.client.get_async_connection().await?;
        let result: i32 = redis::cmd("DEL").arg(k.get_key())
            .query_async(&mut conn).await?;
        Ok(result)
    }
}