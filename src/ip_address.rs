use std::net::SocketAddr;

#[get("/ip_address")]
pub async fn get_ip_address(addr: SocketAddr) -> String {
    return format!("{}", addr.ip());
}