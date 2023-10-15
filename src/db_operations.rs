use std::str::FromStr;

use sqlx::types::BigDecimal;
use time::OffsetDateTime;

mod get_all_prices;
mod get_latest_price_from_5_min;
mod insert_prices;

pub use get_all_prices::get_prices;
pub use get_latest_price_from_5_min::get_latest_price_within_5_minutes;
pub use insert_prices::insert_prices;

use crate::CurrentPrice;

#[derive(Debug)]
pub struct PriceInfoEntity {
    pub timestamp: OffsetDateTime,
    pub crypto_id: String,
    pub currency_ticker: String,
    pub price: BigDecimal,
}

impl From<CurrentPrice> for PriceInfoEntity {
    fn from(value: CurrentPrice) -> Self {
        // let price_as_string = value.price.to_string();
        PriceInfoEntity {
            crypto_id: value.crypto_id,
            currency_ticker: value.currency_ticker,
            timestamp: OffsetDateTime::from_unix_timestamp(value.timestamp).unwrap(),
            price: BigDecimal::from_str(&value.price.to_string()).unwrap(),
        }
    }
}
