use crate::coingecko::{CoinGeckoClient, CoinMarket};
use crate::client:: Client;
use crate::model::AssetPrice;
use std::str::FromStr;
use ethaddr::Address;

pub struct PriceUpdater {
    coin_gecko_client: CoinGeckoClient,
    price_client: Client,
}

impl PriceUpdater {
    pub fn new(price_client: Client, coin_gecko_client: CoinGeckoClient) -> Self {
        PriceUpdater {
            coin_gecko_client,
            price_client,
        }
    }

    pub async fn update_prices(&mut self) ->  Result<usize, Box<dyn std::error::Error>>  {
        let coin_list = self.coin_gecko_client.get_coin_list().await?;
        let coins_map = CoinGeckoClient::convert_coin_vec_to_map(coin_list.clone());
        let coin_markets = self.coin_gecko_client.get_all_coin_markets(250, 10).await?;

        println!("coin_list: {}", coin_list.len());
        println!("coin_markets: {}", coin_markets.len());

        let mut prices: Vec<AssetPrice> = Vec::new();

        for market in coin_markets {
            
            let chain = get_chain(market.id.as_str());
            
            match chain {
                Some(value) => {
                    let asset_id = get_asset_id(value, "".to_string());
                    prices.push(
                        asset_price_map(asset_id, market.clone())
                    );
                    // special case.
                    if value.id() == Chain::Binance.id() {
                        prices.push(
                            asset_price_map(Chain::SmartChain.id().to_string(), market.clone())
                        );
                    }
                    if value.id() == Chain::Ethereum.id() {
                        prices.push(
                            asset_price_map(Chain::Arbitrum.id().to_string(), market.clone())
                        );
                        prices.push(
                            asset_price_map(Chain::Optimism.id().to_string(), market.clone())
                        );
                    }
                }
                None=> {
                    let coin_map = coins_map.get(market.id.as_str()).unwrap();
                    for (platform, token_id) in coin_map.platforms.clone().into_iter() {
                        let platform = get_chain(platform.as_str());
                        match platform {
                            Some(value) => {
                                let token_id = token_id.unwrap_or_default();
                                if token_id.len() > 0 {
                                    let asset_id = get_asset_id(value, token_id);
                                    prices.push(
                                        asset_price_map(asset_id, market.clone())
                                    );
                                }
                            }
                            None=> {}
                        }
                    }
                }
            }
        }

    let count = self.price_client.set_assets_prices(prices).await?;

    println!("set_assets_prices: {:?} assets", count);

    return Ok(count)
    }
}

#[derive(Copy, Clone)]
pub enum Chain {
    Bitcoin,
    Ethereum,
    Binance,
    SmartChain,
    Polygon,
    Solana,
    Arbitrum,
    Optimism,
    Thorchain,
    Cosmos,
    Osmosis,
    Ton,
    Tron,
}

impl Chain {
    pub fn id(&self) -> &str {
        match self {
            Chain::Binance => "binance",
            Chain::Bitcoin => "bitcoin",
            Chain::Ethereum => "ethereum",
            Chain::SmartChain => "smartchain",
            Chain::Polygon => "polygon",
            Chain::Solana => "solana",
            Chain::Arbitrum => "arbitrum",
            Chain::Optimism => "optimism",
            Chain::Thorchain => "thorchain",
            Chain::Cosmos => "cosmos",
            Chain::Osmosis => "osmosis",
            Chain::Ton => "ton",
            Chain::Tron => "tron",
        }
    }
}

fn asset_price_map(asset_id: String, market: CoinMarket) -> AssetPrice {
    return AssetPrice::new(
        asset_id, 
        market.current_price.unwrap_or_default(), 
        market.price_change_percentage_24h.unwrap_or_default(),
        market.market_cap.unwrap_or_default(),
        market.market_cap_rank.unwrap_or_default(),
        market.total_volume.unwrap_or_default(),
    );
}

fn get_asset_id(chain: Chain, token_id: String) -> String {
    if token_id.is_empty() {
        return format!("{}", chain.id())
    }
    return format!("{}_{}", chain.id(), format_token_id(chain, token_id))
}

fn format_token_id(chain: Chain, token_id: String) -> String {
    match chain {
        Chain::Ethereum |
        Chain::SmartChain |
        Chain::Polygon |
        Chain::Arbitrum |
        Chain::Optimism => {
            return Address::from_str(&token_id.as_str()).unwrap().to_string();
        }
        _ => {
            return token_id
        }
    }
}

fn get_chain(id: &str) -> Option<Chain> {
    match id {
        "bitcoin" => Some(Chain::Bitcoin),
        "binancecoin" => Some(Chain::Binance),
        "ethereum" => Some(Chain::Ethereum),
        "binance-smart-chain" => Some(Chain::SmartChain),
        "matic-network" |
        "polygon-pos" => Some(Chain::Polygon),
        "solana" => Some(Chain::Solana),
        "arbitrum-one" => Some(Chain::Arbitrum),
        "optimistic-ethereum" => Some(Chain::Optimism),
        "thorchain" => Some(Chain::Thorchain),
        "cosmos" => Some(Chain::Cosmos),
        "osmosis" => Some(Chain::Osmosis),
        "the-open-network" => Some(Chain::Ton),
        "tron" => Some(Chain::Tron),
        _ => {
            None
        }
    }
}