// 'https://api.coingecko.com/api/v3/coins/zombie-inu-2/market_chart/range?vs_currency=usd&from=1689370624&to=1697319614
//   /coins/{id}/market_chart/range
/*
{
  "prices": [
    [
      1689379200000,
      0.0010693574236728504
    ],
    [
      1689465600000,
      0.0010667337644652307
    ],
    [
      1689552000000,
      0.0010751823140497318
    ],
    [
      1689638400000,
      0.0010640355902136805
    ],
  ]
}
*/

/**
 *
 *
 * Get historical market data include price, market cap, and 24h volume (granularity auto)

Data granularity is automatic (cannot be adjusted)

1 day from current time = 5 minute interval data
2 - 90 days of date range = hourly data
above 90 days of date range = daily data (00:00 UTC)
Cache / Update Frequency: every 5 minutes.
The last completed UTC day (00:00) is available 35 minutes after midnight on the next UTC day (00:35).
 */
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
pub enum PricePointType {
    Min5,
    Hourly,
    Daily,
}

#[derive(Debug)]
pub struct HistoricalPrice {
    pub crypto_id: String,
    pub currency_ticker: String,
    pub price: f64,
    pub timestamp: i64,
    pub type_of: PricePointType,
}

pub async fn get_historical_price(
    symbol: &str,
    currency_ticker: &str,
    from: i64,
    to: i64,
    type_of: PricePointType,
) -> Result<Vec<HistoricalPrice>, Box<dyn error::Error>> {
    let user_error: String = format!(
        "Could not get historical price of ticker from {} to {}",
        from, to
    );

    let base_path: String = env::var("BASE_PATH").expect("No base path in env file");
    let url_with_path = format!("{}/coins/{}/market_chart/range", base_path, symbol);

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
                crypto_id: symbol.into(),
                currency_ticker: currency_ticker.into(),
                price: price.clone(),
                timestamp: timestamp.clone(),
                type_of: type_of.clone(),
            };
        })
        .collect::<Vec<HistoricalPrice>>();

    let from_as_date = OffsetDateTime::from_unix_timestamp(from).ok().unwrap();
    let to_as_date = OffsetDateTime::from_unix_timestamp(to).ok().unwrap();
    let from_formatted = from_as_date.format(&well_known::Iso8601::DEFAULT)?;
    let to_formatted = to_as_date.format(&well_known::Iso8601::DEFAULT)?;
    info!("Got historical prices for crypto:{} for currency {} from {} to {} with number of results {}", symbol,currency_ticker,from_formatted,to_formatted, result.len());
    return Ok(result);
}
