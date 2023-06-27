use std::error::Error;
use redis_client::RedisClient;
use serde::{Serialize, Deserialize};

const CONFIG_TOKENLIST_PREFIX: &str = "config:tokenlists:";

//TODO: Move client to separate folder

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigResponse {
    pub app: App,
    pub versions: Versions,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Versions {
    pub nodes: i32,
    pub fiat_assets: i32,
    pub token_lists: i32,
    pub token_lists_chains: Vec<TokenListVersion>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppVersion {
    pub production: String,
    pub beta: String,
    pub alpha: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppAndroid {
    pub version: AppVersion
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppiOS {
    pub version: AppVersion
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
    pub ios: AppiOS,
    pub android: AppAndroid,
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
        let token_lists_chains: Vec<TokenListVersion> = self.store.get_values(CONFIG_TOKENLIST_PREFIX).await.unwrap_or_default();
        let token_lists: i32 = token_lists_chains
            .iter()
            .map(|token_list| token_list.version)
            .sum();
        let app = self.store.get_value("config:app").await.unwrap();

        let response = ConfigResponse{
            //TODO fetch fiat assets version from db
            app,
            versions: Versions { 
                nodes: 1, 
                fiat_assets: 1, 
                token_lists,
                token_lists_chains,
            }
        };
        return Ok(response)
    }
}