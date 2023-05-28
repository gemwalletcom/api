use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FiatQuote {
    pub provider: FiatProvider,
    pub fiat_amount: f64,
    pub fiat_currency: String,
    pub crypto_amount: f64,
    pub crypto_currency: String,
    pub redirect_url: String,
}

#[derive(Debug, Clone)]
pub struct FiatRequest {
    pub asset_id: String,
    pub ip_address: String,
    pub amount: f64,
    pub currency: String,
    pub wallet_address: String
}

pub struct FiatRequestMap {
    pub crypto_currency: String,
    pub network: String,
}

pub enum FiatProviderName {
    Mercuryo,
    Transak,
    MoonPay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FiatAssets {
    pub version: i64,
    pub assets: Vec<String>
}

impl FiatProviderName {
    pub fn as_str(&self) -> &'static str {
        match self {
            FiatProviderName::Mercuryo => "Mercuryo",
            FiatProviderName::Transak => "Transak",
            FiatProviderName::MoonPay => "MoonPay",
        }
    }

    pub fn as_fiat_provider(&self) -> FiatProvider {
        match self {
            FiatProviderName::Mercuryo => FiatProvider { 
                name: self.as_str().to_string(),
                image_url: "".to_string(),
            },
            FiatProviderName::Transak => FiatProvider { 
                name: self.as_str().to_string(),
                image_url: "".to_string(),
            },
            FiatProviderName::MoonPay => FiatProvider { 
                name: self.as_str().to_string(),
                image_url: "".to_string(),
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FiatProvider {
    pub name: String,
    pub image_url: String,
}

// mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatMappings {
    pub version: i32,
    pub assets: HashMap<String, HashMap<String, FiatMapping>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiatMapping {
    pub symbol: String,
    pub network: Option<String>
}

pub trait FiatClient {
    fn name(&self) -> FiatProviderName;
}