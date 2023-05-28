extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;
use rocket::tokio::sync::Mutex;
use crate::node_client::{Client as NodeClient, ChainNode};

#[get("/nodes")]
pub async fn get_nodes(
    node_client: &State<Mutex<NodeClient>>,
) -> Json<Vec<ChainNode>> {
    let assets = node_client.lock().await.get_nodes().await;
    match assets {
        Ok(value) => Json(value),
        Err(_) => Json(vec![]),
    }
}