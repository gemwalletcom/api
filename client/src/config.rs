extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;
use rocket::tokio::sync::Mutex;
use crate::config_client::{Client as ConfigClient, ConfigResponse};

#[get("/config")]
pub async fn get_config(
    config_client: &State<Mutex<ConfigClient>>,
) -> Json<ConfigResponse> {
    let config = config_client.lock().await.get_config().await.unwrap();
    Json(config)
}