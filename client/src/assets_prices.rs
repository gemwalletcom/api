extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use pricer::client::{Client as PriceClient};
use pricer::model::{AssetPrice};
use rocket::State;
use rocket::tokio::sync::Mutex;
use serde::Deserialize;

#[get("/prices/<asset_id>")]
pub async fn get_asset_price(asset_id: String, price_client: &State<Mutex<PriceClient>>,) -> Json<PricesResponse> {
    //TODO: Add currency as an optional value
    let prices_result = price_client.lock().await.get_assets_prices(vec![asset_id.clone().as_str()]).await;
    let prices = price_response(prices_result.unwrap_or_default());

    Json(PricesResponse{currency: "USD".to_string(), prices})
}

#[post("/prices", format = "json", data = "<request>")]
pub async fn get_assets_prices(request: Json<AssetPriceRequest>, price_client: &State<Mutex<PriceClient>>,) -> Json<PricesResponse> {
    let asset_ids = request.asset_ids.iter().map(|x| x.as_str()).collect();
    let prices_result = price_client.lock().await.get_assets_prices(asset_ids).await;
    let prices = price_response(prices_result.unwrap_or_default());
    Json(PricesResponse{currency: request.currency.clone(), prices})
}

fn price_response(prices: Vec<AssetPrice>) -> Vec<PriceResponse> {
    let mut response = Vec::new();
    for asset_price in prices {
        let price_response = PriceResponse{
            asset_id: asset_price.asset_id,
            price: asset_price.price,
            price_change_percentage_24h: asset_price.price_change_percentage_24h,
        };
        response.push(price_response);
    }
    return response
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
    pub price_change_percentage_24h: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPriceRequest {
    pub asset_ids: Vec<String>,
    pub currency: String,
}
