extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use price_client::price_client::PriceClient;
use rocket::State;
use rocket::tokio::sync::Mutex;
use serde::Deserialize;

#[post("/prices", format = "json", data = "<request>")]
pub async fn get_assets_prices(request: Json<AssetPriceRequest>, price_client: &State<Mutex<PriceClient>>,) -> Json<Vec<AssetPriceResponse>> {
    let mut response = Vec::new();
    let asset_ids = request.assets.iter().map(|x| x.as_str()).collect();
    let prices_result = price_client.lock().await.get_assets_prices(asset_ids).await;

    match prices_result {
        Ok(prices) => {
            for asset_price in prices {
                let asset_price_response: AssetPriceResponse = AssetPriceResponse {
                    asset: asset_price.asset,
                    price: asset_price.price,
                };
                response.push(asset_price_response);
            }
            Json(response)
        },
        Err(_) => {
            Json(response)
        },
    }
}

#[derive(Debug, Serialize)]
pub struct AssetPriceResponse {
    pub asset: String,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPriceRequest {
    pub assets: Vec<String>,
    pub currency: String,
}