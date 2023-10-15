use std::sync::Arc;

use log::info;

use crate::{get_list_of_accepted_tickers_and_ids, TokenInfo};

pub struct SymbolCache(Arc<Vec<TokenInfo>>);

impl SymbolCache {
    pub async fn populate() -> Result<SymbolCache, Box<dyn std::error::Error>> {
        let accepted_symbols = get_list_of_accepted_tickers_and_ids().await?;
        let cache = SymbolCache(Arc::new(accepted_symbols));
        return Ok(cache);
    }

    pub fn find_crypto_by_id(&self, id: String) -> Option<&TokenInfo> {
        let result = self.0.iter().find(|e| e.id == id);
        if result.is_some() {
            info!("cache hit: {:?}", result.unwrap());
        }
        return result;
    }

    pub fn get_list_of_ids_for_symbol(&self, symbol: String) -> Vec<String> {
        return self
            .0
            .iter()
            .filter(|e| e.symbol == symbol)
            .map(|e| e.id.clone())
            .collect::<Vec<String>>();
    }
}
