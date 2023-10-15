use log::info;
use sqlx;
use sqlx::PgPool;

use crate::db_operations::PriceInfoEntity;

pub async fn get_prices(pool: &PgPool) -> anyhow::Result<Vec<PriceInfoEntity>> {
    let records = sqlx::query_as!(
        PriceInfoEntity,
        r#"
          SELECT crypto_id,currency_ticker,price,timestamp
          FROM price_info as p
          ORDER BY p.timestamp;
        "#
    )
    .fetch_all(pool)
    .await?;

    info!("Db records {:?}", records);

    Ok(records)
}
