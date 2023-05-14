use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const COINGECKO_API_URL: &str = "https://api.coingecko.com";

#[derive(Debug, Serialize, Deserialize)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub platforms: HashMap<String, Option<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinMarket {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
    pub  price_change_24h: f64,
}

#[derive(Clone)]
pub struct CoinGeckoClient {
    client: reqwest::Client,
}

impl CoinGeckoClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Self { client }
    }

    pub fn convert_coin_vec_to_map(coins: Vec<Coin>) -> HashMap<String, Coin> {
        coins.into_iter().map(|coin| (coin.id.clone(), coin)).collect()
    }

    pub async fn get_coin_list(&self) -> Result<Vec<Coin>, Error> {
        let url = format!("{}/api/v3/coins/list?include_platform=true", COINGECKO_API_URL);
        let response = self.client.get(&url).send().await?;
        let coins: Vec<Coin> = response.json().await?;
        Ok(coins)
    }

    pub async fn get_coin_markets(&self, page: u32, per_page: u32) -> Result<Vec<CoinMarket>, Error> {
        let url = format!(
            "{}/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page={}&page={}&sparkline=false&locale=en",
            COINGECKO_API_URL, per_page, page
        );
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"));
        let response = self.client.get(&url).headers(headers).send().await?;
        let coin_markets: Vec<CoinMarket> = response.json().await?;
        Ok(coin_markets)
    }

    pub async fn get_all_coin_markets(&self, per_page: u32) -> Result<Vec<CoinMarket>, Error> {
        let mut all_coin_markets = Vec::new();
        let mut page = 1;

        loop {
            let coin_markets = self.get_coin_markets(page, per_page).await?;
            if coin_markets.is_empty() {
                break;
            }
            all_coin_markets.extend(coin_markets);
            page += 1;
        }

        Ok(all_coin_markets)
    }
}