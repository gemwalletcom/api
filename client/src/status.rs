extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::Serialize;   
use std::time::{SystemTime, UNIX_EPOCH};

#[get("/")]
pub fn get_status() -> Json<Status> {
   Json(Status { 
        time: get_epoch_ms()
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
}

