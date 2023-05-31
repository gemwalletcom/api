use std::error::Error;

use crate::model::{FiatQuote, FiatProviderName, FiatRequest, FiatAssets, FiatMappingMap};
use crate::moonpay::MoonPayClient;
use crate::transak::TransakClient;
use crate::mercuryo::MercuryoClient;
use futures::future::join_all;
use redis_client::RedisClient;

const ASSETS_KEY: &str = "fiat:assets";
const MAPPING_PREFIX: &str = "fiat:mapping";

pub struct Client {
    store: RedisClient,
    transak: TransakClient,
    moonpay: MoonPayClient,
    mercuryo: MercuryoClient,
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
            mercuryo
        }
    }

    pub async fn get_assets(&mut self) -> Result<FiatAssets, Box<dyn Error + Send + Sync>> {
        return self.store.get_value(ASSETS_KEY).await;
    }

    pub async fn get_fiat_mapping(&mut self, asset_id: &str) -> Result<FiatMappingMap, Box<dyn Error + Send + Sync>> {
        let key = format!("{}:{}", MAPPING_PREFIX, asset_id);
        return self.store.get_value(key.as_str()).await
    }

    pub async fn get_quotes(& mut self, request: FiatRequest, fiat_mapping_map: FiatMappingMap) -> Result<Vec<FiatQuote>, Box<dyn Error + Send>> {
        let futures = vec![
            self.get_quote(request.clone(), FiatProviderName::Transak, fiat_mapping_map.clone()),
            self.get_quote(request.clone(), FiatProviderName::Mercuryo, fiat_mapping_map.clone()),
            self.get_quote(request.clone(), FiatProviderName::MoonPay, fiat_mapping_map.clone()),
        ];

        let results = join_all(futures).await.into_iter().flatten().collect();
        return Ok(results);
    }
    
    //TODO: Refactor to simplify and later use async traits
    async fn get_quote(&self, request: FiatRequest, provider: FiatProviderName, fiat_mapping_map: FiatMappingMap) -> Result<FiatQuote, String> {
        match fiat_mapping_map.get(provider.as_str()) {
            Some(mapping) => {
                match provider {
                    FiatProviderName::Mercuryo => {
                        let value = self.mercuryo.get_quote(request.clone(), mapping.clone()).await;
                        match value {
                            Ok(value) => {  return Ok(value) }
                            Err(_) => { return Err("".into()) }
                        }
                    },
                    FiatProviderName::Transak => {
                        let value = self.transak.get_quote(request.clone(), mapping.clone()).await;
                            match value {
                                Ok(value) => { return Ok(value) }
                                Err(_) => { return Err("".into()) }
                            }
                    },
                    FiatProviderName::MoonPay => {
                        let value = self.moonpay.get_quote(request.clone(), mapping.clone()).await;
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
