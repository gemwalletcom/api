use std::error::Error;

use crate::model::{FiatQuote, FiatProviderName, FiatRequest, FiatMappings, FiatMapping, FiatAssets};
use crate::moonpay::MoonPayClient;
use crate::transak::TransakClient;
use crate::mercuryo::MercuryoClient;
use futures::future::join_all;
use redis_client::RedisClient;

const ASSETS_KEY: &str = "fiat:assets";
//const MAPPINGS_KEY: &str = "fiat:mappings";

pub struct Client {
    store: RedisClient,
    transak: TransakClient,
    moonpay: MoonPayClient,
    mercuryo: MercuryoClient,
    mappings: FiatMappings
}

impl Client {
    pub async fn new(
        redis_url: &str,
        transak: TransakClient, 
        moonpay: MoonPayClient,
        mercuryo: MercuryoClient
    ) -> Self {
        let store = RedisClient::new(redis_url).await.unwrap();
        Self {
            store,
            transak,
            moonpay,
            mercuryo,
            mappings: serde_json::from_str(include_str!("./fiat_mappings.json")).unwrap()
        }
    }

    pub async fn get_assets(&mut self) -> Result<FiatAssets, Box<dyn Error>> {
        return self.store.get_value(ASSETS_KEY).await;
    }

    pub fn get_mappings(&self) -> FiatMappings {
        return self.mappings.clone()
    }

    pub async fn get_quotes(&self, request: FiatRequest) -> Result<Vec<FiatQuote>, String> {
        let futures = vec![
            self.get_quote(request.clone(), FiatProviderName::Transak),
            self.get_quote(request.clone(), FiatProviderName::Mercuryo),
            self.get_quote(request.clone(), FiatProviderName::MoonPay),
        ];
        let results = join_all(futures).await.into_iter().flatten().collect();
        return Ok(results);
    }
    //TODO: Refactor to simplify and later use async traits
    async fn get_quote(&self, request: FiatRequest, provider: FiatProviderName) -> Result<FiatQuote, String> {
        let mappings = self.get_mappings();
        let asset_id = request.asset_id.as_str();
        
        match mappings.get_fiat_mapping(asset_id, provider.as_str()) {
            Some(mapping) => {
                match provider {
                    FiatProviderName::Mercuryo => {
                        let value = self.mercuryo.get_quote(request.clone(), mapping).await;
                        match value {
                            Ok(value) => {  return Ok(value) }
                            Err(_) => { return Err("".into()) }
                        }
                    },
                    FiatProviderName::Transak => {
                        let value = self.transak.get_quote(request.clone(), mapping).await;
                            match value {
                                Ok(value) => { return Ok(value) }
                                Err(_) => { return Err("".into()) }
                            }
                    },
                    FiatProviderName::MoonPay => {
                        let value = self.moonpay.get_quote(request.clone(), mapping).await;
                        match value {
                            Ok(value) => { return Ok(value) }
                            Err(_) => { return Err("".into()) }
                        }
                    },
                }
            },
            None => {
                return Err("".into())
            }
        }
    } 
}

impl FiatMappings {
    pub fn get_fiat_mapping(&self, asset_id: &str, provider: &str) -> Option<FiatMapping> {
        self.assets
            .get(asset_id)
            .and_then(|inner_map| inner_map.get(provider).cloned())
    }
}

