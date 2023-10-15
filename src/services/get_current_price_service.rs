use crate::{
    api_calls, get_latest_price_within_5_minutes, insert_prices, CurrentPrice, CurrentPriceDto,
    SymbolCache,
};
use anyhow::Result;
use log::info;
use sqlx::{Pool, Postgres};

pub async fn get_current_price_service(
    symbol_cache: &SymbolCache,
    db_connection: &Pool<Postgres>,
    crypto_id: String,
    currency_ticker: String,
) -> Result<CurrentPriceDto, Box<dyn std::error::Error>> {
    return match symbol_cache.find_crypto_by_id(crypto_id.clone()) {
        Some(symbol_info) => {
            let latest_from_within_5_minutes = get_latest_price_within_5_minutes(
                db_connection,
                crypto_id,
                currency_ticker.clone(),
            )
            .await?;

            if latest_from_within_5_minutes.is_some() {
                info!("Fetch current price from DB");
                let unwapped_record: crate::PriceInfoEntity = latest_from_within_5_minutes.unwrap();
                return Ok(unwapped_record.into());
            }

            info!("Fetch current price from API");
            //else we fetch and insert
            let current_price: CurrentPrice =
                api_calls::get_current_price(&symbol_info.id, &currency_ticker).await?;

            insert_prices(db_connection, vec![current_price.clone().into()]).await?;
            return Ok(current_price.into());
        }
        None => anyhow::Result::Err("No such crypto ticker supported".into()),
    };
}
