use std::error::Error;

use redis::{aio::Connection, AsyncCommands, RedisResult};
use serde::{de::DeserializeOwned, Serialize};

pub struct RedisClient {
    conn: Connection,
}

impl RedisClient {
    pub async fn new(database_url: &str) -> RedisResult<Self> {
        let client = redis::Client::open(database_url)?;
        let conn = client.get_async_connection().await?;
        
        Ok(Self {
            conn,
        })
    }

    pub async fn set_value<T>(&mut self, key: &str, value: &T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(value)?;
        self.conn.set(key, serialized).await?;
        Ok(())
    }

    pub async fn get_value<T>(&mut self, key: &str) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let result: Option<String> = self.conn.get(key).await?;
        match result {
            Some(serialized) => {
                let value: T = serde_json::from_str(&serialized)?;
                Ok(value)
            },
            None => {
                Err("serilization".into())
            },
        }
    }
}


