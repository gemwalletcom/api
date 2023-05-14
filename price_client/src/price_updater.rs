use crate::coingecko::CoinGeckoClient;
use crate::price_client:: {PriceClient, AssetPrice};
use std::str::FromStr;
use ethaddr::Address;

pub struct PriceUpdater {
    coin_gecko_client: CoinGeckoClient,
    price_client: PriceClient,
}

impl PriceUpdater {
    pub fn new(price_client: PriceClient, coin_gecko_client: CoinGeckoClient) -> Self {
        PriceUpdater {
            coin_gecko_client,
            price_client,
        }
    }

    pub async fn update_prices(&mut self)  {
        let coin_list = self.coin_gecko_client.get_coin_list().await.unwrap();
        let coins_map = CoinGeckoClient::convert_coin_vec_to_map(coin_list);
        let coin_markets = self.coin_gecko_client.get_coin_markets(1, 100).await.unwrap();
        //println!("Coin Markets (Page 1, 100 per page): {:?}", coin_markets);

        let mut prices: Vec<AssetPrice> = Vec::new();

        for market in coin_markets {
            let coin_map = coins_map.get(market.id.as_str()).unwrap();
            let chain = get_chain(market.id.as_str());
            
            match chain {
                Some(value) => {
                    let asset_id = get_asset_id(value, "".to_string());
                    let price = AssetPrice::new(asset_id, market.current_price, market.price_change_24h);
                    prices.push(price)
                }
                None=> {
                    for (platform, token_id) in coin_map.platforms.clone().into_iter() {
                        let platform = get_chain(platform.as_str());
                        match platform {
                            Some(value) => {
                                let asset_id = get_asset_id(value, token_id.unwrap());

                                let price = AssetPrice::new(asset_id, market.current_price, market.price_change_24h);
                                prices.push(price)
                            }
                            None=> {}
                        }
                    }
                }
            }
        }

    let count = self.price_client.set_assets_prices(prices).await;

    println!("set_assets_prices: {:?} assets", count);

    }
}

#[derive(Copy, Clone)]
enum Chain {
    Bitcoin,
    Ethereum,
    Binance,
}

impl Chain {
    fn id(&self) -> &str {
        match self {
            Chain::Binance => "binance",
            Chain::Bitcoin => "bitcoin",
            Chain::Ethereum => "ethereum",
        }
    }
}

fn get_asset_id(chain: Chain, token_id: String) -> String {
    if token_id.is_empty() {
        return format!("{}", chain.id())
    }
    return format!("{}_{}", chain.id(), format_token_id(chain, token_id))
}

fn format_token_id(chain: Chain, token_id: String) -> String {
    match chain {
        Chain::Ethereum => {
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
        _ => {
            None
        }
    }
}