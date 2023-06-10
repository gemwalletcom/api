use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub redis: Redis,
    pub fiat: Fiat,
    pub moonpay: MoonPay,
    pub transak: Transak,
    pub mercuryo: Mercuryo,
    pub coingecko: CoinGecko,
    pub pricer: Pricer,
    pub plausible: Plausible,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Fiat {
    pub timeout: u64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Redis {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KeyPublic {
    pub public: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KeySecret {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Key {
    pub secret: String,
    pub public: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct MoonPay {
    pub key: Key,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Transak {
    pub key: KeyPublic,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Mercuryo {
    pub key: KeyPublic,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct CoinGecko {
    pub key: KeySecret,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Pricer {
    pub timer: u64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Plausible {
    pub url: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("./Settings"))
            .add_source(Environment::with_prefix("").prefix_separator("").separator("_"))
            .build()?;
        s.try_deserialize()
    }
}