extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::Serialize;   
use std::time::{SystemTime, UNIX_EPOCH};
use rocket_client_addr::ClientRealAddr;

#[get("/")]
pub fn get_status(
    ip: std::net::IpAddr,
    ip_socket: std::net::SocketAddr,
    client_addr: &ClientRealAddr
) -> Json<Status> {
   Json(Status { 
        time: get_epoch_ms(),
        ipv4: client_addr.get_ipv4_string().unwrap(),
        ipv4_v2: ip.to_string(),
        ipv4_v3: ip_socket.to_string(),
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
    ipv4_v2: String,
    ipv4_v3: String,
    ipv6: String,
}

