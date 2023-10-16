use crate::http_get::get;
use log::error;
use log::info;
use serde::Deserialize;
use std::{env, error};

#[derive(Debug, Deserialize, Clone)]
pub struct TokenInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

pub async fn get_list_of_accepted_tickers_and_ids() -> Result<Vec<TokenInfo>, Box<dyn error::Error>>
{
    let base_path: String = env::var("BASE_PATH").expect("No base path in env file");

    let url_with_path = format!("{}/coins/list", base_path);
    let user_error = "Could not get list of tickers";

    let mut url = reqwest::Url::parse(&url_with_path).map_err(|err| {
        error!("Error: {} ", err);
        user_error
    })?;
    url.set_query(Some("include_platform=false"));

    let token_infos = get::<Vec<TokenInfo>>(url, user_error).await?;

    info!(
        "List of currencies retrieved with {} number of elements",
        token_infos.len()
    );

    return Ok(token_infos);
}
