use crate::http_get::get;
use log::{error, info};
use serde::Deserialize;
use std::{env, error};
use time::format_description::well_known;

use time::OffsetDateTime;
#[derive(Debug, Deserialize)]
struct HistoricalData {
    prices: Vec<(i64, f64)>,
}

#[derive(Debug, Clone)]
pub struct HistoricalPrice {
    pub crypto_id: String,
    pub currency_ticker: String,
    pub price: f64,
    pub timestamp_milisecond: i64,
}

pub async fn get_historical_price(
    crypto_id: &'_ str,
    currency_ticker: &'_ str,
    from: i64,
    to: i64,
) -> Result<Vec<HistoricalPrice>, Box<dyn error::Error>> {
    let user_error: String = format!(
        "Could not get historical price of ticker from {} to {}",
        from, to
    );

    let base_path: String = env::var("BASE_PATH").expect("No base path in env file");
    let url_with_path = format!("{}/coins/{}/market_chart/range", base_path, crypto_id);

    let mut url = reqwest::Url::parse(&url_with_path).map_err(|err| {
        error!("Error: {} ", err);
        user_error.clone()
    })?;

    url.query_pairs_mut()
        .append_pair("vs_currency", currency_ticker)
        .append_pair("from", &from.to_string())
        .append_pair("to", &to.to_string())
        .finish();

    let query_result: HistoricalData = get::<HistoricalData>(url, &user_error).await?;

    let result = query_result
        .prices
        .iter()
        .map(|(timestamp, price)| {
            return HistoricalPrice {
                crypto_id: crypto_id.into(),
                currency_ticker: currency_ticker.into(),
                price: price.clone(),
                timestamp_milisecond: timestamp.clone(),
            };
        })
        .collect::<Vec<HistoricalPrice>>();

    let from_as_date = OffsetDateTime::from_unix_timestamp(from.clone())
        .ok()
        .unwrap();
    let to_as_date = OffsetDateTime::from_unix_timestamp(to.clone())
        .ok()
        .unwrap();
    let from_formatted = from_as_date.format(&well_known::Iso8601::DEFAULT)?;
    let to_formatted = to_as_date.format(&well_known::Iso8601::DEFAULT)?;
    info!("Got historical prices for crypto: {} for currency {} from {} to {} with number of results {}", crypto_id,currency_ticker,from_formatted,to_formatted, result.len());
    return Ok(result);
}
