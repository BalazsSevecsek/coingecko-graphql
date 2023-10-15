use std::error;

use log::error;
use serde::de::DeserializeOwned;

fn handle_error<E: std::error::Error>(err: E, user_error: &str) -> &str {
    error!("Error: {} ", err);
    user_error
}

pub async fn get<T: DeserializeOwned>(
    url: reqwest::Url,
    error_format: &str,
) -> Result<T, Box<dyn error::Error>> {
    let resp = reqwest::get(url)
        .await
        .map_err(|err| handle_error(err, error_format))?;
    let status_checked_resp = resp
        .error_for_status()
        .map_err(|err| handle_error(err, error_format))?;
    let result: T = status_checked_resp
        .json::<T>()
        .await
        .map_err(|err| handle_error(err, error_format))?;
    Ok(result)
}
