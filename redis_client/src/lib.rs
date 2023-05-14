use redis::{aio::Connection, AsyncCommands, RedisResult};
use serde::{de::DeserializeOwned, Serialize};

// Generic function to insert a value into Redis
pub async fn set_value<T>(conn: &mut Connection, key: &str, value: &T) -> RedisResult<()>
where
    T: Serialize,
{
    let serialized = serde_json::to_string(value)?;
    conn.set(key, serialized).await?;
    Ok(())
}

// Generic function to retrieve a value from Redis
pub async fn get_value<T>(conn: &mut Connection, key: &str) -> RedisResult<Option<T>>
where
    T: DeserializeOwned,
{
    let result: Option<String> = conn.get(key).await?;
    match result {
        Some(serialized) => {
            let value: T = serde_json::from_str(&serialized)?;
            Ok(Some(value))
        },
        None => Ok(None),
    }
}
