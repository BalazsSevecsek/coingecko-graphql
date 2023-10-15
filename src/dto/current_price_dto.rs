use async_graphql::SimpleObject;

use crate::{CurrentPrice, PriceInfoEntity};

#[derive(SimpleObject, Debug)]
pub struct CurrentPriceDto {
    pub crypto_id: String,
    pub currency_ticker: String,
    pub price: f64,
}

impl From<PriceInfoEntity> for CurrentPriceDto {
    fn from(value: PriceInfoEntity) -> Self {
        CurrentPriceDto {
            crypto_id: value.crypto_id,
            currency_ticker: value.currency_ticker,
            price: value.price.to_string().parse::<f64>().unwrap(),
        }
    }
}

impl From<CurrentPrice> for CurrentPriceDto {
    fn from(value: CurrentPrice) -> Self {
        CurrentPriceDto {
            crypto_id: value.crypto_id,
            currency_ticker: value.currency_ticker,
            price: value.price,
        }
    }
}
