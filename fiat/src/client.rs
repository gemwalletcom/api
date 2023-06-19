use std::error::Error;
use std::time::Duration;

use crate::model::{FiatQuote, FiatProviderName, FiatRequest, FiatAssets, FiatMappingMap, MAPPING_PREFIX, ASSETS_KEY};
use crate::moonpay::MoonPayClient;
use crate::transak::TransakClient;
use crate::mercuryo::MercuryoClient;
use crate::ramp::RampClient;
use futures::future::join_all;
use redis_client::RedisClient;
use reqwest::Client as RequestClient;

pub struct Client {
    store: RedisClient,
    transak: TransakClient,
    moonpay: MoonPayClient,
    mercuryo: MercuryoClient,
    ramp: RampClient,
}

impl Client {
    pub async fn new(
        redis_url: &str,
        transak: TransakClient, 
        moonpay: MoonPayClient,
        mercuryo: MercuryoClient,
        ramp: RampClient
    ) -> Self {
        let store = RedisClient::new(redis_url).await.unwrap();

        Self {
            store,
            transak,
            moonpay,
            mercuryo,
            ramp
        }
    }

    pub fn request_client(timeout_seconds: u64) -> RequestClient {
        return RequestClient::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .build().unwrap()
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
            self.get_quote(request.clone(), FiatProviderName::Ramp, fiat_mapping_map.clone()),
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
                    FiatProviderName::Ramp => {
                        let value = self.ramp.get_quote(request.clone(), mapping.clone()).await;
                        match value {
                            Ok(value) => { return Ok(value) }
                            Err(ee) => { 
                                println!("error {}", ee);
                                return Err("".into()) 
                            }
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
