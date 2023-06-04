use redis_client::RedisClient;

use crate::model::{FiatAssets, MAPPING_PREFIX, ASSETS_KEY};

pub struct Updater {
    store: RedisClient,
}

impl Updater {
    pub async fn new(
        redis_url: &str,
    ) -> Self {
        let store = RedisClient::new(redis_url).await.unwrap();

        Self {
            store,
        }
    }

    pub async fn update_assets(& mut self) {
        let prefix = format!("{}:", MAPPING_PREFIX);
        let keys: Vec<String> = self.store.get_keys(&prefix).await.unwrap_or_default();
        let asset_ids: Vec<String> = keys.iter()
            .filter_map(|key| key.strip_prefix(&prefix))
            .map(|x| x.to_string())
            .collect();

        let assets = FiatAssets{version: 1, asset_ids};
        let _ = self.store.set_value(ASSETS_KEY, &assets).await;  

        println!("update_assets {:?}", assets.asset_ids.len());
    }
}