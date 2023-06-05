#[macro_use] 
extern crate rocket;
mod status;
mod assets_prices;
mod fiat_quotes;
mod nodes;
mod node_client;
mod config;
mod config_client;

use fiat::mercuryo::MercuryoClient;
use fiat::moonpay::MoonPayClient;
use fiat::transak::TransakClient;
use node_client::Client as NodeClient;
use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use settings::Settings;
use pricer::client::Client as PriceClient;
use fiat::client::Client as FiatClient;
use config_client::Client as ConfigClient;

use rocket::tokio::sync::Mutex;

async fn rocket(settings: Settings) -> Rocket<Build> {
    let redis_url = settings.redis.url.as_str();
    let price_client = PriceClient::new(redis_url).await.unwrap();
    let node_client = NodeClient::new(redis_url).await;
    let config_client = ConfigClient::new(redis_url).await;
    let request_client = FiatClient::request_client(settings.fiat.timeout);
    let transak = TransakClient::new(request_client.clone(), settings.transak.key.public);
    let moonpay = MoonPayClient::new( request_client.clone(),  settings.moonpay.key.public,  settings.moonpay.key.secret);
    let mercuryo = MercuryoClient::new(request_client.clone(), settings.mercuryo.key.public);
    let fiat_client = FiatClient::new(
        redis_url,
        transak,
        moonpay,
        mercuryo,
    ).await;

    rocket::build()
        .attach(AdHoc::on_ignite("Tokio Runtime Configuration", |rocket| async {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime");
            rocket.manage(runtime)
        }))
        .manage(Mutex::new(fiat_client))
        .manage(Mutex::new(price_client))
        .manage(Mutex::new(node_client))
        .manage(Mutex::new(config_client))
        .mount("/", routes![
            status::get_status,
        ])
        .mount("/v1", routes![
            assets_prices::get_asset_price,
            assets_prices::get_assets_prices,
            fiat_quotes::get_fiat_quotes,
            fiat_quotes::get_fiat_assets,
            nodes::get_nodes,
            config::get_config,
        ])
}

#[tokio::main]
async fn main() {

    let settings = Settings::new().unwrap();

    let rocket = rocket(settings).await;
    rocket.launch().await.expect("Failed to launch Rocket");
}