use std::str::FromStr;

use async_trait::async_trait;
use mockall::automock;
use sqlx::{types::BigDecimal, PgPool};
use time::OffsetDateTime;

mod get_latest_price_from_12_min;
mod insert_prices;

use crate::{CurrentPrice, HistoricalPrice};

#[derive(Debug, Clone)]
pub struct PriceInfoEntity {
    pub timestamp: OffsetDateTime,
    pub crypto_id: String,
    pub currency_ticker: String,
    pub price: BigDecimal,
}

impl Default for PriceInfoEntity {
    fn default() -> Self {
        PriceInfoEntity {
            timestamp: OffsetDateTime::now_utc(),
            crypto_id: "".to_string(),
            currency_ticker: "".to_string(),
            price: BigDecimal::from_str("0").unwrap(),
        }
    }
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

#[derive(Debug, Clone)]
pub struct DbConnection(PgPool);

#[automock]
impl DbConnection {
    pub fn new(pool: PgPool) -> Self {
        DbConnection(pool)
    }

    // #[cfg(test)]
    // pub fn new_internal() -> Self {
    //     use std::sync::Arc;

    //     DbConnection(Arc::new(Vec::new()))
    // }
}

#[automock]
#[async_trait]
pub trait DbOperations {
    async fn get_latest_price_within_12_minutes(
        &self,
        crypto_id: String,
        currency_ticker: String,
    ) -> anyhow::Result<Option<PriceInfoEntity>>;
    async fn insert_prices(&self, prices: Vec<PriceInfoEntity>) -> anyhow::Result<u64>;
}

#[async_trait]
impl DbOperations for DbConnection {
    async fn get_latest_price_within_12_minutes(
        &self,
        crypto_id: String,
        currency_ticker: String,
    ) -> anyhow::Result<Option<PriceInfoEntity>> {
        get_latest_price_from_12_min::get_latest_price_within_12_minutes(
            &self.0,
            crypto_id,
            currency_ticker,
        )
        .await
    }
    async fn insert_prices(&self, prices: Vec<PriceInfoEntity>) -> anyhow::Result<u64> {
        insert_prices::insert_prices(&self.0, prices).await
    }
}
