use super::symbol_cache_service::SymbolCacheOperations;
pub fn get_crypto_ids_service(
    symbol_cache: &impl SymbolCacheOperations,
    crypto_ticker: String,
) -> Vec<String> {
    return symbol_cache.get_list_of_ids_for_symbol(crypto_ticker);
}
