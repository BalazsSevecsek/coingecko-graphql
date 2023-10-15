// https://api.coingecko.com/api/v3/simple/price?ids=zombie-inu-2&vs_currencies=usd
/*
{
  "zombie-inu-2": {
    "usd": 0.00078097
  }
}
*/

use crate::http_get::get;
use log::debug;
use log::error;
use serde::Deserialize;
use std::collections::HashMap;
use std::{env, error};

#[derive(Debug, Deserialize)]
struct CurrencyDataRaw {
    #[serde(flatten)]
    crypto_tickers: HashMap<String, PriceDataRow>,
}

#[derive(Debug, Deserialize)]
struct PriceDataRow {
    #[serde(flatten)]
    prices: HashMap<String, f64>,
}

#[derive(Debug, Deserialize)]
pub struct CurrentPrice {
    pub crypto_ticker: String,
    pub currency_ticker: String,
    pub price: f64,
}

pub async fn get_current_price(
    symbol: &str,
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
        .append_pair("ids", symbol)
        .append_pair("vs_currencies", currency_ticker)
        .finish();

    let query_result: CurrencyDataRaw = get::<CurrencyDataRaw>(url, USER_ERROR).await?;

    let result = query_result
        .crypto_tickers
        .iter()
        .map(|(crypto_name, value)| {
            let currency_ticker = value.prices.keys().take(1).next().unwrap();
            let price = value.prices.get(currency_ticker).unwrap_or(&0.0f64);
            return CurrentPrice {
                crypto_ticker: crypto_name.clone(),
                currency_ticker: currency_ticker.clone(),
                price: price.clone(),
            };
        })
        .take(1)
        .next()
        .expect(USER_ERROR);

    println!("{:?}", result);
    return Ok(result);
}
