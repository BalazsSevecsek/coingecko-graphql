use crate::http_get::get;
use log::debug;
use log::error;
use serde::Deserialize;
use std::{env, error};

#[derive(Debug, Deserialize)]
pub struct TokenInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

pub async fn get_list_of_accepted_tickers_and_ids() -> Result<Vec<TokenInfo>, Box<dyn error::Error>>
{
    let base_path: String = env::var("BASE_PATH").expect("No base path in env file");
    // let api_key: String = env::var("API_KEY").expect("No api key in env file");

    let url_with_path = format!("{}/coins/list", base_path);
    let user_error = "Could not get list of tickers";

    let mut url = reqwest::Url::parse(&url_with_path).map_err(|err| {
        error!("Error: {} ", err);
        user_error
    })?;
    url.set_query(Some("include_platform=false"));

    // url.set_query(Some(&format!("x_cg_pro_api_key={}", api_key)));

    let token_infos = get::<Vec<TokenInfo>>(url, user_error).await?;

    println!("REsp:{:?}", token_infos);

    debug!("{:?}", token_infos);

    return Ok(token_infos);
}
