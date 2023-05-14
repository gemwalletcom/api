#[macro_use] 
extern crate rocket;
mod ip_address;
mod status;
mod assets_prices;
use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use std::env;
use price_client::PriceClient;
use rocket::tokio::sync::Mutex;

async fn rocket() -> Rocket<Build> {
    let database_url = env::var("REDIS_URL").expect("REDIS_URL not set");

    let price_client = PriceClient::new(database_url.as_str()).await.unwrap();

    rocket::build()
        .attach(AdHoc::on_ignite("Tokio Runtime Configuration", |rocket| async {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime");
            rocket.manage(runtime)
        }))
        .manage(Mutex::new(price_client))
        .mount("/", routes![
            status::get_status, 
            ip_address::get_ip_address,
            assets_prices::get_assets_prices,
        ])
}

#[tokio::main]
async fn main() {
    let rocket = rocket().await;
    rocket.launch().await.expect("Failed to launch Rocket");
}