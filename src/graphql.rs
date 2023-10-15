use crate::{
    get_crypto_ids_service, get_current_price_service, get_historical_data_service,
    CurrentPriceDto, HistoricalPriceDto, SymbolCache,
};
use async_graphql::Result;
use async_graphql::{Context, Object};
use log::info;
use sqlx::{Pool, Postgres};
use time::OffsetDateTime;

pub struct Query;

#[Object]
impl Query {
    pub async fn crypto_ticker_ids<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        crypto_ticker: String,
    ) -> Result<Vec<String>> {
        let symbol_cache = ctx.data::<SymbolCache>().clone().unwrap();
        let res = get_crypto_ids_service(symbol_cache, crypto_ticker);
        info!("Retrieved list of ids for crypto ticker {:?}", res);
        return Ok(res);
    }

    pub async fn get_current_price<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        crypto_id: String,
        currency_ticker: String,
    ) -> Result<CurrentPriceDto> {
        let db_connection = ctx.data::<Pool<Postgres>>().clone().unwrap();
        let symbol_cache = ctx.data::<SymbolCache>().clone().unwrap();
        let res =
            get_current_price_service(symbol_cache, db_connection, crypto_id, currency_ticker)
                .await
                .map_err(|err| async_graphql::Error::new(err.to_string()))?;
        info!("Retrieved current price {:?}", res);
        return Ok(res);
    }

    pub async fn get_historical_price(
        &self,
        crypto_id: String,
        currency_ticker: String,
        from: OffsetDateTime,
        to: OffsetDateTime,
    ) -> HistoricalPriceDto {
        get_historical_data_service(crypto_id, currency_ticker, from, to).await
    }
}
