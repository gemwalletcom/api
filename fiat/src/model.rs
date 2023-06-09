use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub const ASSETS_KEY: &str = "fiat:assets";
pub const MAPPING_PREFIX: &str = "fiat:mapping";

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FiatQuote {
    pub provider: FiatProvider,
    pub fiat_amount: f64,
    pub fiat_currency: String,
    pub crypto_amount: f64,
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
    Ramp,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiatAssets {
    pub version: i64,
    pub asset_ids: Vec<String>
}

impl FiatProviderName {
    pub fn as_str(&self) -> &'static str {
        match self {
            FiatProviderName::Mercuryo => "Mercuryo",
            FiatProviderName::Transak => "Transak",
            FiatProviderName::MoonPay => "MoonPay",
            FiatProviderName::Ramp => "Ramp",
        }
    }

    pub fn as_fiat_provider(&self) -> FiatProvider {
        return FiatProvider { 
            name: self.as_str().to_string(),
            image_url: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FiatProvider {
    pub name: String,
    pub image_url: String,
}

// mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiatMapping {
    pub symbol: String,
    pub network: Option<String>
}

pub type FiatMappingMap = HashMap<String, FiatMapping>;

pub trait FiatClient {
    fn name(&self) -> FiatProviderName;
}