use bigdecimal::BigDecimal;
use std::str::FromStr;
use substreams::scalar::BigInt;

/// Calculate price from maker and taker amounts
pub fn calculate_price(maker_amount: &BigInt, taker_amount: &BigInt) -> BigDecimal {
    if taker_amount.to_string() == "0" {
        return BigDecimal::from_str("0").unwrap();
    }
    
    let maker_decimal = BigDecimal::from_str(&maker_amount.to_string()).unwrap_or_default();
    let taker_decimal = BigDecimal::from_str(&taker_amount.to_string()).unwrap_or_default();
    
    if taker_decimal == BigDecimal::from_str("0").unwrap() {
        BigDecimal::from_str("0").unwrap()
    } else {
        maker_decimal / taker_decimal
    }
}

/// Determine trade side based on asset IDs
pub fn determine_trade_side(
    maker_asset_id: &BigInt,
    taker_asset_id: &BigInt,
    _maker_amount: &BigInt,
    _taker_amount: &BigInt,
) -> String {
    // For very large BigInt values, we'll use string-based logic
    let maker_id_str = maker_asset_id.to_string();
    let taker_id_str = taker_asset_id.to_string();
    
    // Check if the last digit is even (0,2,4,6,8) or odd (1,3,5,7,9)
    let maker_is_even = maker_id_str.chars().last().map_or(false, |c| matches!(c, '0'|'2'|'4'|'6'|'8'));
    let taker_is_even = taker_id_str.chars().last().map_or(false, |c| matches!(c, '0'|'2'|'4'|'6'|'8'));
    
    // USDC collateral typically has even asset IDs (divisible by 2)
    // Outcome tokens typically have odd asset IDs
    match (maker_is_even, taker_is_even) {
        (true, false) => "buy".to_string(),   // Maker provides USDC, gets outcome token
        (false, true) => "sell".to_string(),  // Maker provides outcome token, gets USDC
        _ => "unknown".to_string(),
    }
}

/// Generate unique ID from transaction hash and order hash
pub fn generate_order_id(tx_hash: &str, order_hash: &str) -> String {
    format!("{}-{}", tx_hash, order_hash)
}

/// Extract condition ID from asset ID (Polymarket specific logic)
pub fn extract_condition_id(asset_id: &BigInt) -> String {
    format!("condition_{}", asset_id.to_string())
}

/// Extract condition ID from asset ID string
pub fn extract_condition_id_from_str(asset_id: &str) -> String {
    format!("condition_{}", asset_id)
}

/// Format timestamp for day calculation
pub fn timestamp_to_day(timestamp: u64) -> u64 {
    timestamp / 86400 // Convert seconds to days
}
