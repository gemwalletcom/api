use price_client::{ PriceClient, AssetPrice};
use price_client::coingecko:: CoinGeckoClient;
use std::env;
use std::fmt::Debug;
use std::str::FromStr;
use ethaddr::Address;

#[tokio::main]
pub async fn main() {
    let database_url = env::var("REDIS_URL").expect("REDIS_URL not set");
    let mut price_client = PriceClient::new(database_url.as_str()).await.unwrap();
    
    let coin_gecko_client = CoinGeckoClient::new().await.unwrap();
    let coin_list = coin_gecko_client.get_coin_list().await.unwrap();
    
    let coins_map = CoinGeckoClient::convert_coin_vec_to_map(coin_list);

    //println!("Coin List: {:?}", coin_list);

    println!("bitcoin: {:?}", coins_map.get("bitcoin"));

    let coin_markets = coin_gecko_client.get_coin_markets(1, 100).await.unwrap();
    //println!("Coin Markets (Page 1, 100 per page): {:?}", coin_markets);

    let mut prices: Vec<AssetPrice> = Vec::new();

    for market in coin_markets {
        let coin_map = coins_map.get(market.id.as_str()).unwrap();
        let chain = get_chain(market.id.as_str());
        
        match chain {
            Some(value) => {
                let asset_id = get_asset_id(value, "".to_string());
                print!("asset_id: {}\n", get_asset_id(value, "".to_string()));    

                let price = AssetPrice::new(asset_id, market.current_price, market.price_change_24h);
                prices.push(price)
            }
            None=> {
                for (platform, token_id) in coin_map.platforms.clone().into_iter() {
                    //print!("platform: {}, {:?}\n", platform, token_id);
                    
                    let platform = get_chain(platform.as_str());
                    match platform {
                        Some(value) => {
                            let asset_id = get_asset_id(value, token_id.unwrap());

                            let price = AssetPrice::new(asset_id, market.current_price, market.price_change_24h);
                            prices.push(price)
                            //print!("asset_id: {}: name: {}, price: {}\n", asset_id, market.name, market.current_price);
                        }
                        None=> {}
                    }
                }
            }
        }
    }

    let _ = price_client.set_assets_prices(prices).await;

    

    // let all_coin_markets = coin_gecko_client.get_all_coin_markets(100).await?;
    // println!("All Coin Markets: {:?}", all_coin_markets);


    //price_client.set_asset_price("bitcoin", 50000.0).await.expect("Failed to set asset price");
    //price_client.set_asset_price("ethereum", 2100.0).await.expect("Failed to set asset price");

    //let value = price_client.get_asset_price("bitcoin").await.expect("nope");

    //println!("{:?}", value);
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