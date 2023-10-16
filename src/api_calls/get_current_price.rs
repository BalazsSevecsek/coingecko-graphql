use crate::http_get::get;
use crate::PriceInfoEntity;
use log::error;
use serde::Deserialize;
use std::collections::HashMap;
use std::{env, error};

#[derive(Debug, Deserialize)]
struct CurrencyDataRaw {
    #[serde(flatten)]
    crypto_ids: HashMap<String, PriceDataRow>,
}
#[derive(Debug, Deserialize)]
struct PriceDataRow {
    #[serde(flatten)]
    prices: HashMap<String, f64>,
    last_updated_at: i64,
}

#[derive(Debug, Deserialize, Clone, Default, PartialEq, PartialOrd)]
pub struct CurrentPrice {
    pub crypto_id: String,
    pub currency_ticker: String,
    pub price: f64,
    pub timestamp: i64,
}

impl From<PriceInfoEntity> for CurrentPrice {
    fn from(value: PriceInfoEntity) -> Self {
        let price_as_string = value.price.to_string();
        CurrentPrice {
            crypto_id: value.crypto_id,
            currency_ticker: value.currency_ticker,
            price: price_as_string.parse::<f64>().unwrap(),
            timestamp: value.timestamp.unix_timestamp(),
        }
    }
}

pub async fn get_current_price(
    crypto_id: &str,
    currency_ticker: &str,
) -> Result<CurrentPrice, Box<dyn error::Error>> {
    const USER_ERROR: &'static str = "Could not get current price of ticker";

    let base_path: String = env::var("BASE_PATH").expect("No base path in env file");
    let url_with_path = format!("{}/simple/price", base_path);

    let mut url = reqwest::Url::parse(&url_with_path).map_err(|err| {
        error!("Error: {} ", err);
        USER_ERROR
    })?;

    url.query_pairs_mut()
        .append_pair("ids", crypto_id)
        .append_pair("vs_currencies", currency_ticker)
        .append_pair("include_last_updated_at", "true")
        .finish();

    let query_result: CurrencyDataRaw = get::<CurrencyDataRaw>(url, USER_ERROR).await?;

    let result = query_result
        .crypto_ids
        .iter()
        .map(|(crypto_name, value)| {
            let timestamp = value.last_updated_at;
            let currency_ticker = value.prices.keys().take(1).next().unwrap();
            let price = value.prices.get(currency_ticker).unwrap_or(&0.0f64);
            return CurrentPrice {
                crypto_id: crypto_name.clone(),
                currency_ticker: currency_ticker.clone(),
                price: price.clone(),
                timestamp: timestamp.clone(),
            };
        })
        .take(1)
        .next()
        .expect(USER_ERROR);

    return Ok(result);
}
