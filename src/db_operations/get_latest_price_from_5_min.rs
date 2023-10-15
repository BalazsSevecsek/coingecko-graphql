use log::info;
use sqlx;
use sqlx::PgPool;

use crate::db_operations::PriceInfoEntity;

pub async fn get_latest_price_within_10_minutes(
    pool: &PgPool,
    crypto_id: String,
    currency_ticker: String,
) -> anyhow::Result<Option<PriceInfoEntity>> {
    let record = sqlx::query_as!(
        PriceInfoEntity,
        r#"
          SELECT *
          FROM price_info
          WHERE crypto_id = $1
            AND currency_ticker = $2
            AND timestamp >= NOW() - INTERVAL '10 minutes'
          ORDER BY timestamp DESC
          LIMIT 1;
        "#,
        crypto_id,
        currency_ticker
    )
    .fetch_optional(pool)
    .await?;

    if record.is_some() {
        info!("Db records {:?}", record);
    }

    Ok(record)
}
