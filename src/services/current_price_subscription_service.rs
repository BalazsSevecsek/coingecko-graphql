use crate::{get_current_price_service, CurrentPriceDto, SymbolCache};
use async_stream::try_stream;
use futures_core::Stream;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio_stream::StreamExt;

pub async fn subscribe_to_current_price<'a>(
    symbol_cache: &'a SymbolCache,
    db_connection: &'a Pool<Postgres>,
    crypto_id: String,
    currency_ticker: String,
) -> impl Stream<Item = Result<CurrentPriceDto, Box<dyn std::error::Error + 'a>>> {
    try_stream! {
        let mut interval_stream =
            tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(3)));

        while let Some(_) = interval_stream.next().await{
            let res:CurrentPriceDto = get_current_price_service(symbol_cache,db_connection,crypto_id.clone(),currency_ticker.clone()).await?;
            yield res;
        };
    }
}
