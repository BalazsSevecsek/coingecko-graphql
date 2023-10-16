use crate::{ApiOperations, CurrentPrice, CurrentPriceDto, DbOperations, PriceInfoEntity};
use log::info;

use super::symbol_cache_service::SymbolCacheOperations;

pub async fn get_current_price_service(
    symbol_cache: &impl SymbolCacheOperations,
    db_connection: &impl DbOperations,
    api_caller: impl ApiOperations,
    crypto_id: String,
    currency_ticker: String,
) -> anyhow::Result<CurrentPriceDto, Box<dyn std::error::Error>> {
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
            let current_price: CurrentPrice = api_caller
                .get_current_price(&symbol_info.id, &currency_ticker)
                .await?;
            let entity: PriceInfoEntity = current_price.clone().into();

            db_connection.insert_prices(vec![entity]).await?;
            return Ok(current_price.into());
        }
        None => anyhow::Result::Err("No such crypto ticker supported".into()),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        services::symbol_cache_service::MockSymbolCacheOperations, MockApiOperations,
        MockDbOperations, TokenInfo,
    };

    #[tokio::test]
    async fn test_get_current_price_service_gets_value_from_database_when_it_exists() {
        let mut symbol_cache_operations_mock = MockSymbolCacheOperations::new();
        symbol_cache_operations_mock
            .expect_find_crypto_by_id()
            .withf(|crypto_id| crypto_id == "valid_crypto_id")
            .times(1)
            .return_const(Some(TokenInfo {
                id: "valid_crypto_id".to_string(),
                name: "some_crypto_name".to_string(),
                symbol: "crypto_symbol".into(),
            }));

        let mut db_operations_mock = MockDbOperations::new();
        db_operations_mock
            .expect_get_latest_price_within_12_minutes()
            .times(1)
            .returning(|_, _| Ok(Some(PriceInfoEntity::default())));

        db_operations_mock.expect_insert_prices().times(0);

        let mut api_calls_mock = MockApiOperations::new();
        api_calls_mock.expect_get_current_price().times(0);

        // // Act
        let result = get_current_price_service(
            &symbol_cache_operations_mock,
            &db_operations_mock,
            api_calls_mock,
            "valid_crypto_id".to_string(),
            "usd".to_string(),
        )
        .await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_current_price_service_gets_value_from_api_when_no_data_in_db() {
        let mut symbol_cache_operations_mock = MockSymbolCacheOperations::new();
        symbol_cache_operations_mock
            .expect_find_crypto_by_id()
            .withf(|crypto_id| crypto_id == "valid_crypto_id")
            .times(1)
            .return_const(Some(TokenInfo {
                id: "valid_crypto_id".to_string(),
                name: "some_crypto_name".to_string(),
                symbol: "crypto_symbol".into(),
            }));

        let mut db_operations_mock = MockDbOperations::new();
        db_operations_mock
            .expect_get_latest_price_within_12_minutes()
            .times(1)
            .returning(|_, _| Ok(None));

        db_operations_mock
            .expect_insert_prices()
            .times(1)
            .returning(|_| Ok(1));

        let mut api_calls_mock = MockApiOperations::new();
        api_calls_mock
            .expect_get_current_price()
            .times(1)
            .returning(|_, _| Ok(CurrentPrice::default()));

        // // Act
        let result = get_current_price_service(
            &symbol_cache_operations_mock,
            &db_operations_mock,
            api_calls_mock,
            "valid_crypto_id".to_string(),
            "usd".to_string(),
        )
        .await;

        // Assert
        // assert!(result.is_ok());
        assert_eq!(result.unwrap(), CurrentPriceDto::default());
    }
}
