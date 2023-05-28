use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPrice {
    pub asset_id: String,
    pub price: f64,
    pub price_change_percentage_24h: f64,
    pub last_updated: u64,
    pub market_cap: f64,
    pub market_cap_rank: u64,
    pub total_volume: f64,
}

impl AssetPrice {
    pub fn new(
        asset_id: String, 
        price: f64, 
        price_change_percentage_24h: f64,
        market_cap: f64,
        market_cap_rank: u64,
        total_volume: f64,
    ) -> Self {
        let last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        AssetPrice {
            asset_id,
            price,
            price_change_percentage_24h,
            last_updated,
            market_cap,
            market_cap_rank,
            total_volume,
        }
    }
}