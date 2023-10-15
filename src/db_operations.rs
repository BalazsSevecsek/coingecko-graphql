use std::str::FromStr;

use sqlx::types::BigDecimal;
use time::OffsetDateTime;

mod get_latest_price_from_12_min;
mod insert_prices;

pub use get_latest_price_from_12_min::get_latest_price_within_12_minutes;
pub use insert_prices::insert_prices;

use crate::{CurrentPrice, HistoricalPrice};

#[derive(Debug, Clone)]
pub struct PriceInfoEntity {
    pub timestamp: OffsetDateTime,
    pub crypto_id: String,
    pub currency_ticker: String,
    pub price: BigDecimal,
}

impl From<CurrentPrice> for PriceInfoEntity {
    fn from(value: CurrentPrice) -> Self {
        PriceInfoEntity {
            crypto_id: value.crypto_id,
            currency_ticker: value.currency_ticker,
            timestamp: OffsetDateTime::from_unix_timestamp(value.timestamp).unwrap(),
            price: BigDecimal::from_str(&value.price.to_string()).unwrap(),
        }
    }
}

impl From<HistoricalPrice> for PriceInfoEntity {
    fn from(value: HistoricalPrice) -> Self {
        PriceInfoEntity {
            crypto_id: value.crypto_id,
            currency_ticker: value.currency_ticker,
            timestamp: OffsetDateTime::from_unix_timestamp(value.timestamp_milisecond / 1000)
                .unwrap(),
            price: BigDecimal::from_str(&value.price.to_string()).unwrap(),
        }
    }
}
