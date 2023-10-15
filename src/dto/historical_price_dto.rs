use async_graphql::SimpleObject;
use time::PrimitiveDateTime;

#[derive(SimpleObject)]
pub struct HistoricalPriceDto {
    pub crypto_id: String,
    pub currency_ticker: String,
    pub price: f64,
    pub utc_timestamp: PrimitiveDateTime,
}
