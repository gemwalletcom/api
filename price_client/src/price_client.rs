use redis::{aio::Connection, AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

pub struct PriceClient {
    conn: Connection,
    prefix: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPrice {
    pub asset_id: String,
    pub price: f64,
    pub price_change_24h: f64,
    pub last_updated: u64,
}

impl AssetPrice {
    pub fn new(asset_id: String, price: f64, price_change_24h: f64) -> Self {
        let last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get system time")
            .as_secs();

        AssetPrice {
            asset_id,
            price,
            price_change_24h,
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
            prefix: "prices:".to_owned(),
        })
    }

    pub fn convert_asset_price_vec_to_map(coins: Vec<AssetPrice>) -> HashMap<String, AssetPrice> {
        coins.into_iter().map(|coin| (coin.asset_id.clone(), coin)).collect()
    }

    pub fn asset_key(&mut self, asset: String) -> String {
        return format!("{}{}", self.prefix, asset);
    }

    pub async fn set_assets_prices(&mut self, prices: Vec<AssetPrice>) -> RedisResult<usize> {
        let serialized: Vec<(String, String)> = prices
        .iter()
        .map(|x| {
            (
                self.asset_key(x.asset_id.clone()),
                serde_json::to_string(x).unwrap(),
            )
        })
        .collect();

        self.conn.mset(serialized.as_slice()).await?;

        Ok(serialized.len())
    }

    pub async fn get_assets_prices(&mut self, assets: Vec<&str>) -> RedisResult<Vec<AssetPrice>> {
        let keys: Vec<String> = assets.iter().map(|x| self.asset_key(x.to_string())).collect();
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