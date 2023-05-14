extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use price_client::price_client::{PriceClient};
use rocket::State;
use rocket::tokio::sync::Mutex;
use serde::Deserialize;

#[post("/prices", format = "json", data = "<request>")]
pub async fn get_assets_prices(request: Json<AssetPriceRequest>, price_client: &State<Mutex<PriceClient>>,) -> Json<PricesResponse> {
    let mut response = Vec::new();
    let asset_ids = request.asset_ids.iter().map(|x| x.as_str()).collect();
    let prices_result = price_client.lock().await.get_assets_prices(asset_ids).await;

    match prices_result {
        Ok(prices) => {
            for asset_price in prices {
                let price_response = PriceResponse{
                    asset_id: asset_price.asset_id,
                    price: asset_price.price,
                    price_change_24h: asset_price.price_change_24h,
                };
                response.push(price_response);
            }
            Json(PricesResponse{currency: request.currency.clone(), prices: response})
        },
        Err(_) => {
            Json(PricesResponse{currency: request.currency.clone(), prices: response})
        },
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PricesResponse {
    pub currency: String,
    pub prices: Vec<PriceResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceResponse {
    pub asset_id: String,
    pub price: f64,
    pub price_change_24h: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPriceRequest {
    pub asset_ids: Vec<String>,
    pub currency: String,
}