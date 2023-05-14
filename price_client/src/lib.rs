use redis::{aio::Connection, AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PriceClient {
    conn: Connection,
    prefix: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPrice {
    pub asset: String,
    pub price: f64,
    pub price_change: f64,
    pub last_updated: u64,
}

impl AssetPrice {
    pub fn new(asset: String, price: f64, price_change: f64) -> Self {
        let last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get system time")
            .as_secs();

        AssetPrice {
            asset,
            price,
            price_change,
            last_updated,
        }
    }
}

impl PriceClient {
    pub async fn new(database_url: &str) -> RedisResult<Self> {
        let client = redis::Client::open(database_url)?;
        let conn = client.get_async_connection().await?;
        
        Ok(PriceClient {
            conn,
            prefix: "assets:".to_owned(),
        })
    }

    pub async fn set_assets_prices(&mut self, prices: Vec<AssetPrice>) -> RedisResult<()> {
        // let serialized = serde_json::to_string(&asset_price)?;
        // self.conn
        //     .set(format!("{}{}", self.prefix, asset), serialized)
        //     .await?;
        Ok(())
    }

    pub async fn get_assets_prices(&mut self, assets: Vec<&str>) -> RedisResult<Vec<AssetPrice>> {
        let keys: Vec<String> = assets.iter().map(|x| format!("{}{}", self.prefix, x)).collect();
        let result: Option<Vec<String>> = self
            .conn
            .mget(keys)
            .await?;

        match result {
            Some(serialized) => {
                let value = serialized.iter().map(|x| {
                    let price: Option<AssetPrice> = serde_json::from_str(&x).unwrap_or(None);
                    return price;
                }).flatten().collect();
                Ok(value)
            }
            None => Ok(vec![]),
        }
    }
}