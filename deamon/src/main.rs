use pricer::client::Client;
use pricer::coingecko:: CoinGeckoClient;
use pricer::price_updater:: PriceUpdater;

use std::thread;
use std::time::Duration;

#[tokio::main]
pub async fn main() {
    let settings = settings::Settings::new().unwrap();
    let price_client = Client::new(&settings.redis.url).await.unwrap();
    let coingecko_client = CoinGeckoClient::new(settings.coingecko.key.secret);
    let mut price_updater = PriceUpdater::new(price_client, coingecko_client);

    loop {
        price_updater.update_prices().await;
    
        thread::sleep(Duration::from_secs(settings.pricer.timer));
    }
}