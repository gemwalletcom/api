use redis::{aio::Connection, AsyncCommands, RedisResult};
use std::collections::HashMap;

use crate::model::AssetPrice;

pub struct Client {
    conn: Connection,
    prefix: String,
}

impl Client {
    pub async fn new(database_url: &str) -> RedisResult<Self> {
        let client = redis::Client::open(database_url)?;
        let conn = client.get_async_connection().await?;
        
        Ok(Self {
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
        let result: Vec<Option<String>> = self
            .conn
            .mget(keys)
            .await?;

        let values: Vec<String> = result.into_iter().flatten().collect();
        let prices: Vec<AssetPrice> = values.iter().map(|x| {
            serde_json::from_str(&x).unwrap_or(None)
        }).flatten().collect();

        Ok(prices)
    }
}