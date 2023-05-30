use std::error::Error;
use redis_client::RedisClient;
use serde::{Serialize, Deserialize};

const NODES_CHAIN_PREFIX: &str = "nodes:chain:";

//TODO: Move client to separate folder

#[derive(Debug, Deserialize, Serialize)]
pub struct ChainNode {
    pub chain: String,
    pub nodes: Vec<Node>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub url: String,
    pub status: String,
    pub priority: String,
}

pub struct Client {
    store: RedisClient,
}

impl Client {
    pub async fn new(
        redis_url: &str,
    ) -> Self {
        let store = RedisClient::new(redis_url).await.unwrap();
        Self {
            store,
        }
    }

    pub async fn get_nodes(&mut self) -> Result<Vec<ChainNode>, Box<dyn Error>> {
        return self.store.get_values(NODES_CHAIN_PREFIX).await
    }
}