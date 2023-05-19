use price_client::price_client::PriceClient;
use price_client::coingecko:: CoinGeckoClient;
use price_client::price_updater:: PriceUpdater;
use std::env;

use std::thread;
use std::time::Duration;

#[tokio::main]
pub async fn main() {
    let database_url = env::var("REDIS_URL").expect("REDIS_URL not set");
    let cg_api_key = env::var("COINGECKO_API_KEY").unwrap_or_default();
    let price_client = PriceClient::new(database_url.as_str()).await.unwrap();
    let coingecko_client = CoinGeckoClient::new(cg_api_key);
    let mut price_updater = PriceUpdater::new(price_client, coingecko_client);

    loop {
        price_updater.update_prices().await;
    
        thread::sleep(Duration::from_secs(300));
    }
}