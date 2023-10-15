use crate::{api_calls, insert_prices, HistoricalPriceDto, PriceInfoEntity, SymbolCache};
use sqlx::{Pool, Postgres};
use time::OffsetDateTime;

pub async fn get_historical_data_service(
    symbol_cache: &SymbolCache,
    db_connection: &Pool<Postgres>,
    crypto_id: String,
    currency_ticker: String,
    from: OffsetDateTime,
    to: OffsetDateTime,
) -> Result<Vec<HistoricalPriceDto>, Box<dyn std::error::Error>> {
    return match symbol_cache.find_crypto_by_id(crypto_id.clone()) {
        Some(symbol_info) => {
            // let price_type = PricePointType::Daily;

            // if is_within_today(&from, &to) {
            //     price_type = PricePointType::Min5;
            // } else {
            //     if is_within_2_to_90_days_of_range(&from, &to) {
            //         price_type = PricePointType::Hourly;
            //     }
            // }

            // info!("price_type {:?}", price_type);

            let rows = api_calls::get_historical_price(
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
                insert_prices(db_connection, converted_entities.clone()).await?;
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

// fn is_within_today(from: &OffsetDateTime, to: &OffsetDateTime) -> bool {
//     let utc_current_time = time::OffsetDateTime::now_utc();
//     let one_day_duration = time::Duration::days(1);

//     let start_range_utc_time = utc_current_time - one_day_duration;

//     let inter_from_to = to.clone() - from.clone();

//     inter_from_to <= one_day_duration && from >= &start_range_utc_time
// }

// fn is_within_2_to_90_days_of_range(from: &OffsetDateTime, to: &OffsetDateTime) -> bool {
//     let range_from_to = to.clone() - from.clone();
//     let days_90_duration = Duration::days(90);
//     return range_from_to <= days_90_duration && range_from_to > Duration::days(1);
// }

// fn number_of_day_in_range(from: &OffsetDateTime, to: &OffsetDateTime) -> i32 {

// }
