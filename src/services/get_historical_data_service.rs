use crate::{
    api_calls, ApiCalls, ApiClient, DbConnection, DbOperations, HistoricalPriceDto,
    PriceInfoEntity, SymbolCache,
};
use sqlx::{Pool, Postgres};
use time::OffsetDateTime;

pub async fn get_historical_data_service(
    symbol_cache: &SymbolCache,
    db_connection: &DbConnection,
    crypto_id: String,
    currency_ticker: String,
    from: OffsetDateTime,
    to: OffsetDateTime,
) -> Result<Vec<HistoricalPriceDto>, Box<dyn std::error::Error>> {
    return match symbol_cache.find_crypto_by_id(crypto_id.clone()) {
        Some(symbol_info) => {
            let rows = ApiClient::get_historical_price(
                &symbol_info.id,
                &currency_ticker,
                from.unix_timestamp(),
                to.unix_timestamp(),
            )
            .await?;

            let converted_entities = rows
                .iter()
                .map(|e| (*e).clone().into())
                .collect::<Vec<PriceInfoEntity>>();

            if converted_entities.len() > 0 {
                db_connection
                    .insert_prices(converted_entities.clone())
                    .await?;
            }

            let result_form = converted_entities
                .iter()
                .map(|e| (*e).clone().into())
                .collect();
            return Ok(result_form);
        }
        None => anyhow::Result::Err("No such crypto ticker supported".into()),
    };
}
