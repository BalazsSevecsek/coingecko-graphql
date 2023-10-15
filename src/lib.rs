mod api_calls;
mod db_operations;
mod dto;
mod graphql;
mod http_get;
mod services;

pub use api_calls::*;
pub use db_operations::*;
pub use dto::*;
pub use graphql::*;
pub use services::*;

// let prices = vec![
//     PriceInfoEntity {
//         crypto_ticker: "zombie-inu-2".into(),
//         currency_ticker: "usd".into(),
//         price: BigDecimal::from_str("0.55").unwrap(),
//         timestamp: OffsetDateTime::from_unix_timestamp(1697361531).unwrap(),
//     },
//     PriceInfoEntity {
//         crypto_ticker: "zombie-inu-2".into(),
//         currency_ticker: "usd".into(),
//         price: BigDecimal::from_str("0.6").unwrap(),
//         timestamp: OffsetDateTime::from_unix_timestamp(1697372559).unwrap(),
//     },
// ];
// insert_prices(&pool, prices).await?;
// get_prices(&pool).await?;
// coingecko_graphql::get_list_of_accepted_tickers_and_ids().await?;
// coingecko_graphql::get_current_price("zombie-inu-2", "usd").await?;
// coingecko_graphql::get_historical_price(
//     "zombie-inu-2",
//     "usd",
//     1689370624,
//     1697319614,
//     coingecko_graphql::PricePointType::Daily,
// )
// .await?;
