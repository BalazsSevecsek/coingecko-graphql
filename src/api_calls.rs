mod get_current_price;
mod get_historical_price;
mod get_list_of_currencies;
use async_trait::async_trait;
use mockall::automock;
use std::error;

pub use get_current_price::CurrentPrice;
pub use get_historical_price::HistoricalPrice;
pub use get_list_of_currencies::TokenInfo;

#[derive(Clone, Default)]
pub struct ApiClient;

#[automock]
#[async_trait]
pub trait ApiOperations {
    async fn get_historical_price(
        &self,
        crypto_id: &'_ str,
        currency_ticker: &'_ str,
        from: i64,
        to: i64,
    ) -> Result<Vec<HistoricalPrice>, Box<dyn error::Error>>;
    async fn get_current_price(
        &self,
        crypto_id: &str,
        currency_ticker: &str,
    ) -> Result<CurrentPrice, Box<dyn error::Error>>;
    async fn get_list_of_accepted_tickers_and_ids(
        &self,
    ) -> Result<Vec<TokenInfo>, Box<dyn error::Error>>;
}

#[async_trait]
impl ApiOperations for ApiClient {
    async fn get_historical_price(
        &self,
        crypto_id: &'_ str,
        currency_ticker: &'_ str,
        from: i64,
        to: i64,
    ) -> Result<Vec<HistoricalPrice>, Box<dyn error::Error>> {
        get_historical_price::get_historical_price(crypto_id, currency_ticker, from, to).await
    }

    async fn get_current_price(
        &self,
        crypto_id: &str,
        currency_ticker: &str,
    ) -> Result<CurrentPrice, Box<dyn error::Error>> {
        get_current_price::get_current_price(crypto_id, currency_ticker).await
    }

    async fn get_list_of_accepted_tickers_and_ids(
        &self,
    ) -> Result<Vec<TokenInfo>, Box<dyn error::Error>> {
        get_list_of_currencies::get_list_of_accepted_tickers_and_ids().await
    }
}
