use price_client::PriceClient;
use std::env;

#[tokio::main]
pub async fn main() {
    let database_url = env::var("REDIS_URL").expect("DATABASE_URL not set");
    let mut price_client = PriceClient::new(database_url.as_str()).await.unwrap();

    price_client.set_asset_price("bitcoin", 50000.0).await.expect("Failed to set asset price");
    price_client.set_asset_price("ethereum", 2100.0).await.expect("Failed to set asset price");

    let value = price_client.get_asset_price("bitcoin").await.expect("nope");

    println!("{:?}", value);
}

