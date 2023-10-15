use log::info;
use sqlx;
use sqlx::PgPool;
use time::OffsetDateTime;

use crate::db_operations::PriceInfoEntity;

#[derive(Debug, Clone)]
pub enum PricePointType {
    Min5,
    Hourly,
    Daily,
}

pub async fn get_historical_prices_with_granularity(
    pool: &PgPool,
    crypto_id: String,
    currency_ticker: String,
    from: OffsetDateTime,
    to: OffsetDateTime,
    // granularity: PricePointType,
) -> anyhow::Result<Vec<PriceInfoEntity>> {
    // let restriction: String = match granularity {
    //     PricePointType::Daily => "AND EXTRACT(HOUR FROM timestamp) = 0".to_string(),
    //     PricePointType::Hourly => "AND EXTRACT(HOUR FROM timestamp) = 0".to_string(),
    //     PricePointType::Min5 => "AND EXTRACT(HOUR FROM timestamp) = 0".to_string(),
    // };

    let records = sqlx::query_as!(
        PriceInfoEntity,
        r#"
        SELECT * 
        FROM price_info
        WHERE crypto_id = $1 
          AND currency_ticker = $2 
          AND timestamp >= $3 
          AND timestamp <= $4 
          AND EXTRACT(HOUR FROM timestamp) = 0; -- To filter hourly prices
        "#,
        crypto_id,
        currency_ticker,
        from,
        to
    )
    .fetch_all(pool)
    .await?;

    info!("Db records {:?}", records);

    Ok(records)
}
