use log::info;
use sqlx::PgPool;
use sqlx::{self, Postgres, QueryBuilder};

use crate::db_operations::PriceInfoEntity;

pub async fn insert_prices(pool: &PgPool, prices: Vec<PriceInfoEntity>) -> anyhow::Result<u64> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"INSERT INTO price_info("timestamp", "currency_ticker", "crypto_id", "price") "#,
    );

    query_builder
        .push_values(prices, |mut b, price: PriceInfoEntity| {
            b.push_bind(price.timestamp)
                .push_bind(price.currency_ticker)
                .push_bind(price.crypto_id)
                .push_bind(price.price);
        })
        .push(" ON CONFLICT DO NOTHING; ");

    let query = query_builder.build();

    let num_of_rows = query.execute(pool).await?;

    info!(
        "Number of prices inserted {:?}",
        num_of_rows.rows_affected()
    );
    return Ok(num_of_rows.rows_affected());
}
