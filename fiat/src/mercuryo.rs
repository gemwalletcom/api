use reqwest::Client;
use serde::{Deserialize};
use url::Url;
use crate::model::{FiatQuote, FiatProviderName, FiatRequest, FiatMapping, FiatClient};

const MERCURYO_API_BASE_URL: &str = "https://api.mercuryo.io";
const MERCURYO_REDIRECT_URL: &str = "https://exchange.mercuryo.io";
#[derive(Debug, Deserialize)]
pub struct MercyryoResponse<T> {
    pub data: T,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MercyryoQuote {
    pub amount: String,
    pub currency: String,
    pub fiat_amount: String,
}

pub struct MercuryoClient {
    client: Client,
    widget_id: String,
}

impl MercuryoClient {
    pub fn new(client: Client, widget_id: String) -> Self {
        MercuryoClient {
            client,
            widget_id,
        }
    }

    pub async fn get_quote(
        &self,
        request: FiatRequest,
        request_map: FiatMapping,
    ) -> Result<FiatQuote, reqwest::Error> {
        let url = format!(
            "{}/v1.6/widget/buy/rate?from={}&to={}&amount={}&widget_id={}",
            MERCURYO_API_BASE_URL, request.currency, request_map.symbol, request.amount, self.widget_id
        );
        let response = self.client.get(&url).send().await?;
        let quote = response.json::<MercyryoResponse<MercyryoQuote>>().await?.data;

        Ok(self.get_fiat_quote(request, quote))
    }

    fn get_fiat_quote(&self, request: FiatRequest, quote: MercyryoQuote) -> FiatQuote {
        return FiatQuote{
            provider: self.name().as_fiat_provider(),
            fiat_amount: request.amount,
            fiat_currency: request.currency,
            crypto_amount: quote.clone().amount.parse::<f64>().unwrap_or_default(),
            redirect_url: self.redirect_url(quote.clone(), request.wallet_address),
        }
    }

    pub fn redirect_url(&self, quote: MercyryoQuote, address: String) -> String {
        let mut components = Url::parse(MERCURYO_REDIRECT_URL).unwrap();

        components.query_pairs_mut()
            .append_pair("widget_id", self.widget_id.as_str())
            .append_pair("fiat_amount", &quote.fiat_amount.to_string())
            .append_pair("currency", &quote.currency)
            .append_pair("address", &address);

        return components.as_str().to_string()
    }
}

impl FiatClient for MercuryoClient {
    fn name(&self) -> FiatProviderName {
        return FiatProviderName::Mercuryo
    }
}