extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;
use rocket::tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::node_client::{Client as NodeClient, ChainNode};


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodesResponse {
    pub version: i32,
    pub nodes: Vec<ChainNode>
}

#[get("/nodes")]
pub async fn get_nodes(
    node_client: &State<Mutex<NodeClient>>,
) -> Json<NodesResponse> {
    let nodes = node_client.lock().await.get_nodes().await;
    match nodes {
        Ok(nodes) => Json(NodesResponse{version: 1, nodes}),
        Err(_) => Json(NodesResponse{version: 1, nodes: vec![]}),
    }
}