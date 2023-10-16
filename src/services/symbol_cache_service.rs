use std::sync::Arc;

use log::info;
use mockall::automock;

use crate::{ApiOperations, TokenInfo};

pub struct SymbolCache(Arc<Vec<TokenInfo>>);

#[cfg(test)]
impl Default for SymbolCache {
    fn default() -> Self {
        SymbolCache(Arc::new(Vec::new()))
    }
}
impl Clone for SymbolCache {
    fn clone(&self) -> Self {
        SymbolCache(self.0.clone())
    }
}

impl SymbolCache {
    pub fn new() -> Self {
        SymbolCache(Arc::new(Vec::new()))
    }

    pub async fn populate(
        &mut self,
        api_calls: &impl ApiOperations,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let accepted_symbols = api_calls.get_list_of_accepted_tickers_and_ids().await?;
        let cache = SymbolCache(Arc::new(accepted_symbols));
        return Ok(cache);
    }
}

#[automock]
pub trait SymbolCacheOperations {
    fn get_list_of_ids_for_symbol(&self, symbol: String) -> Vec<String>;
    fn find_crypto_by_id(&self, id: String) -> Option<TokenInfo>;
}

impl SymbolCacheOperations for SymbolCache {
    fn find_crypto_by_id(&self, id: String) -> Option<TokenInfo> {
        let result = self.0.iter().find(|e| e.id == id).map(|v| (*v).clone());
        if result.is_some() {
            info!("cache hit: {:?}", result.clone().unwrap());
        }
        return result;
    }

    fn get_list_of_ids_for_symbol(&self, symbol: String) -> Vec<String> {
        return self
            .0
            .iter()
            .filter(|e| e.symbol == symbol)
            .map(|e| e.id.clone())
            .collect::<Vec<String>>();
    }
}
