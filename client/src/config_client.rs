use std::error::Error;
use redis_client::RedisClient;
use serde::{Serialize, Deserialize};

const CONFIG_TOKENLIST_PREFIX: &str = "config:tokenlists:";

//TODO: Move client to separate folder

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigResponse {
    pub fiat_assets_version: i32,
    pub token_lists: Vec<TokenListVersion>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenListVersion {
    pub chain: String,
    pub version: i32,
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

    pub async fn get_config(&mut self) -> Result<ConfigResponse, Box<dyn Error>> {
        let token_lists: Vec<TokenListVersion> = self.store.get_values(CONFIG_TOKENLIST_PREFIX).await.unwrap();
        let response = ConfigResponse{
            //TODO fetch fiat assets version from db
            fiat_assets_version: 1,
            token_lists
        };
        return Ok(response)
    }
}