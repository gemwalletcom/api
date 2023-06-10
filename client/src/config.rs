extern crate rocket;
use rocket::serde::json::Json;
use rocket::{State};
use rocket::tokio::sync::Mutex;
use crate::config_client::{Client as ConfigClient, ConfigResponse};
use crate::plausible_client::{Client as PlausibleClient};

#[get("/config")]
pub async fn get_config(
    ip: std::net::IpAddr,
    plausible_client: &State<Mutex<PlausibleClient>>,
    config_client: &State<Mutex<ConfigClient>>,
) -> Json<ConfigResponse> {
    let config: ConfigResponse = config_client.lock().await.get_config().await.unwrap();
    return Json(config);
}