extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::Serialize;   
use std::time::{SystemTime, UNIX_EPOCH};
use rocket_client_addr::ClientRealAddr;

#[get("/")]
pub fn get_status(client_addr: &ClientRealAddr) -> Json<Status> {
   Json(Status { 
        time: get_epoch_ms(),
        ipv4: client_addr.get_ipv4_string().unwrap(),
        ipv6: client_addr.get_ipv6_string(),
    })
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[derive(Serialize)]
pub struct Status {
    time: u128,
    ipv4: String,
    ipv6: String,
}

