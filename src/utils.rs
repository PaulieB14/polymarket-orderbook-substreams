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
    let maker_id_num = maker_asset_id.to_u64();
    let taker_id_num = taker_asset_id.to_u64();
    
    // USDC collateral typically has even asset IDs (divisible by 2)
    // Outcome tokens typically have odd asset IDs
    match (maker_id_num % 2, taker_id_num % 2) {
        (0, 1) => "buy".to_string(),   // Maker provides USDC, gets outcome token
        (1, 0) => "sell".to_string(),  // Maker provides outcome token, gets USDC
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

/// Format timestamp for day calculation
pub fn timestamp_to_day(timestamp: u64) -> u64 {
    timestamp / 86400 // Convert seconds to days
}
