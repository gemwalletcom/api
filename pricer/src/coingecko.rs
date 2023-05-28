use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const COINGECKO_API_URL: &str = "https://api.coingecko.com";
const COINGECKO_API_PRO_URL: &str = "https://pro-api.coingecko.com";
const USER_AGENT_VALUE: HeaderValue = HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36");

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub platforms: HashMap<String, Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinMarket {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub current_price: Option<f64>,
    pub price_change_percentage_24h: Option<f64>,
    pub market_cap: Option<f64>,
    pub market_cap_rank: Option<u64>,
    pub total_volume: Option<f64>,
}

#[derive(Clone)]
pub struct CoinGeckoClient {
    client: reqwest::Client,
    url: String,
    api_key: String,
}

impl CoinGeckoClient {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::new();
        let url = Self::url_for_api_key(api_key.clone());
        Self { client, url, api_key }
    }

    fn url_for_api_key(api_key: String) -> String {
        if api_key.len() > 0 {
            return COINGECKO_API_PRO_URL.to_string()
        }
        return COINGECKO_API_URL.to_string()
    }

    pub fn convert_coin_vec_to_map(coins: Vec<Coin>) -> HashMap<String, Coin> {
        coins.into_iter().map(|coin| (coin.id.clone(), coin)).collect()
    }
    
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, USER_AGENT_VALUE);
        return headers;
    }

    pub async fn get_coin_list(&self) -> Result<Vec<Coin>, Error> {
        let url = format!("{}/api/v3/coins/list?include_platform=true&x_cg_pro_api_key={}", self.url, self.api_key);
        let response = self.client
            .get(&url)
            .headers(self.headers())
            .send()
            .await?;
        let coins: Vec<Coin> = response.json().await?;
        Ok(coins)
    }

    pub async fn get_coin_markets(&self, page: u32, per_page: u32) -> Result<Vec<CoinMarket>, Error> {
        let url = format!(
            "{}/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page={}&page={}&sparkline=false&locale=en&x_cg_pro_api_key={}",
            self.url, per_page, page, self.api_key
        );
        let response = self.client
            .get(&url)
            .headers(self.headers())
            .send().await?;

        let coin_markets: Vec<CoinMarket> = response.json().await?;
        Ok(coin_markets)
    }

    pub async fn get_all_coin_markets(&self, per_page: u32, limit_page: u32) -> Result<Vec<CoinMarket>, Error> {
        let mut all_coin_markets = Vec::new();
        let mut page = 1;

        loop {
            let coin_markets = self.get_coin_markets(page, per_page).await?;
            if coin_markets.is_empty() || page == limit_page  {
                break;
            }
            all_coin_markets.extend(coin_markets);
            page += 1;
        }

        Ok(all_coin_markets)
    }
}