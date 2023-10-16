use crate::{
    api_calls, ApiCalls, ApiClient, CurrentPrice, CurrentPriceDto, DbConnection, DbOperations,
    PriceInfoEntity, SymbolCache,
};
use anyhow::Result;
use log::info;
use sqlx::{Pool, Postgres};

pub async fn get_current_price_service(
    symbol_cache: &SymbolCache,
    db_connection: &DbConnection,
    crypto_id: String,
    currency_ticker: String,
) -> Result<CurrentPriceDto, Box<dyn std::error::Error>> {
    return match symbol_cache.find_crypto_by_id(crypto_id.clone()) {
        Some(symbol_info) => {
            let latest_from_within_10_minutes = db_connection
                .get_latest_price_within_12_minutes(crypto_id, currency_ticker.clone())
                .await?;

            if latest_from_within_10_minutes.is_some() {
                info!("Fetch current price from DB");
                let unwapped_record: crate::PriceInfoEntity =
                    latest_from_within_10_minutes.unwrap();
                return Ok(unwapped_record.into());
            }

            info!("Fetch current price from API");
            //else we fetch and insert
            let current_price: CurrentPrice =
                ApiClient::get_current_price(&symbol_info.id, &currency_ticker).await?;
            let entity: PriceInfoEntity = current_price.clone().into();

            // info!("API fetched {:?}", entity.clone());
            db_connection.insert_prices(vec![entity]).await?;
            return Ok(current_price.into());
        }
        None => anyhow::Result::Err("No such crypto ticker supported".into()),
    };
}
