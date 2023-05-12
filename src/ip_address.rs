extern crate rocket;

use rocket::serde::Serialize;
use rocket::serde::json::Json;
use rocket_client_addr::ClientRealAddr;

#[get("/ip_address")]
pub fn get_ip_address(client_addr: &ClientRealAddr) -> Json<IpResponse> {
    Json(IpResponse { 
        ipv4: client_addr.get_ipv4_string().unwrap(),
        ipv6: client_addr.get_ipv6_string(),
    })   
}

#[derive(Serialize)]
pub struct IpResponse {
    ipv4: String,
    ipv6: String,
}