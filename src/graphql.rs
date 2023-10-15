use std::time::Duration;

use crate::{
    get_crypto_ids_service, get_current_price_service, get_historical_data_service,
    CurrentPriceDto, HistoricalPriceDto, SymbolCache,
};
use async_graphql::{Context, Object};
use async_graphql::{Result, Subscription};
use async_stream::try_stream;
use futures_core::Stream;
use log::info;
use sqlx::{Pool, Postgres};
use time::OffsetDateTime;
use tokio::time::sleep;

pub struct Query;

#[Object]
impl Query {
    #[graphql(cache_control(max_age = 3600))]
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

    #[graphql(cache_control(max_age = 300))]
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

    #[graphql(cache_control(max_age = 600))]
    pub async fn get_historical_price<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        crypto_id: String,
        currency_ticker: String,
        from: OffsetDateTime,
        to: OffsetDateTime,
    ) -> Result<Vec<HistoricalPriceDto>> {
        let db_connection = ctx.data::<Pool<Postgres>>().clone().unwrap();
        let symbol_cache = ctx.data::<SymbolCache>().clone().unwrap();
        let res = get_historical_data_service(
            symbol_cache,
            db_connection,
            crypto_id,
            currency_ticker,
            from,
            to,
        )
        .await
        .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        info!("Retrieved historical prices {:?}", res.len());

        return Ok(res);
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn current_price<'a, 'ctx>(
        &self,
        ctx: &'a Context<'ctx>,
        crypto_id: String,
        currency_ticker: String,
    ) -> impl Stream<Item = Result<Option<CurrentPriceDto>, String>> + 'a {
        let db_connection = ctx.data::<Pool<Postgres>>().clone().unwrap();
        let symbol_cache = ctx.data::<SymbolCache>().clone().unwrap();

        try_stream! {
            // let mut interval_stream =
            //     tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(60*5)));

            // while let Some(_) = interval_stream.next().await{
            //     let res = get_current_price_service(symbol_cache,db_connection,crypto_id.clone(),currency_ticker.clone()).await.ok();
            //     yield res;
            // };
            loop{
                let res = get_current_price_service(symbol_cache,db_connection,crypto_id.clone(),currency_ticker.clone()).await.ok();
                yield res;
                sleep(Duration::from_secs(3)).await;
            }
        }
    }
}
