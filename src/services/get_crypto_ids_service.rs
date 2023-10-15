use crate::SymbolCache;
pub fn get_crypto_ids_service(symbol_cache: &SymbolCache, crypto_ticker: String) -> Vec<String> {
    return symbol_cache.get_list_of_ids_for_symbol(crypto_ticker);
}
