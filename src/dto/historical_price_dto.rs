use async_graphql::SimpleObject;
use time::OffsetDateTime;

use crate::{HistoricalPrice, PriceInfoEntity};

#[derive(SimpleObject)]
pub struct HistoricalPriceDto {
    pub crypto_id: String,
    pub currency_ticker: String,
    pub price: f64,
    pub utc_timestamp: OffsetDateTime,
}

impl From<PriceInfoEntity> for HistoricalPriceDto {
    fn from(value: PriceInfoEntity) -> Self {
        HistoricalPriceDto {
            utc_timestamp: value.timestamp,
            crypto_id: value.crypto_id,
            currency_ticker: value.currency_ticker,
            price: value.price.to_string().parse::<f64>().unwrap(),
        }
    }
}

impl From<HistoricalPrice> for HistoricalPriceDto {
    fn from(value: HistoricalPrice) -> Self {
        HistoricalPriceDto {
            utc_timestamp: OffsetDateTime::from_unix_timestamp(value.timestamp_milisecond).unwrap(),
            crypto_id: value.crypto_id,
            currency_ticker: value.currency_ticker,
            price: value.price,
        }
    }
}
