use time::{macros::datetime, OffsetDateTime};

use crate::HistoricalPriceDto;

pub async fn get_historical_data_service(
    crypto_id: String,
    currency_ticker: String,
    from: OffsetDateTime,
    to: OffsetDateTime,
) -> HistoricalPriceDto {
    return HistoricalPriceDto {
        crypto_id: crypto_id.clone(),
        currency_ticker: currency_ticker.clone(),
        price: 1.0,
        utc_timestamp: datetime!(2020-01-01 0:00),
    };
}
