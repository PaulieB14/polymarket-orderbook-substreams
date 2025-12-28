use bigdecimal::BigDecimal;
use num_traits::Zero;
use std::str::FromStr;
use substreams::scalar::BigInt;

/// Convert BigInt to BigDecimal with scaling
pub fn bigint_to_scaled_decimal(value: &BigInt, decimals: u32) -> BigDecimal {
    let base = BigDecimal::from_str("10").unwrap().with_scale(0);
    let divisor = base.with_prec(decimals as u64).powi(decimals as i64);
    
    BigDecimal::from_str(&value.to_string())
        .unwrap_or_default()
        .with_scale(decimals as i64) / divisor
}

/// Calculate price from maker and taker amounts
pub fn calculate_price(maker_amount: &BigInt, taker_amount: &BigInt) -> BigDecimal {
    if taker_amount.is_zero() {
        return BigDecimal::from_str("0").unwrap();
    }
    
    let maker_decimal = BigDecimal::from_str(&maker_amount.to_string()).unwrap_or_default();
    let taker_decimal = BigDecimal::from_str(&taker_amount.to_string()).unwrap_or_default();
    
    if taker_decimal.is_zero() {
        BigDecimal::from_str("0").unwrap()
    } else {
        maker_decimal / taker_decimal
    }
}

/// Determine trade side based on asset IDs and amounts
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

/// Calculate average trade size
pub fn calculate_average_trade_size(total_volume: &BigDecimal, trade_count: u64) -> BigDecimal {
    if trade_count == 0 {
        return BigDecimal::from_str("0").unwrap();
    }
    
    total_volume / BigDecimal::from(trade_count)
}

/// Extract condition ID from asset ID (Polymarket specific logic)
pub fn extract_condition_id(asset_id: &BigInt) -> String {
    format!("condition_{}", asset_id.to_string())
}

/// Classify trader type based on trading patterns
pub fn classify_trader_type(
    trade_count: u64,
    total_volume: &BigDecimal,
    unique_markets: u64,
) -> String {
    let avg_trade_size = if trade_count > 0 {
        total_volume / BigDecimal::from(trade_count)
    } else {
        BigDecimal::from_str("0").unwrap()
    };
    
    let large_trade_threshold = BigDecimal::from_str("10000").unwrap(); // $10k
    let high_frequency_threshold = 100; // 100+ trades
    let market_maker_market_threshold = 5; // 5+ different markets
    
    if trade_count >= high_frequency_threshold && unique_markets >= market_maker_market_threshold {
        "market_maker".to_string()
    } else if avg_trade_size >= large_trade_threshold {
        "whale".to_string()
    } else if unique_markets >= market_maker_market_threshold {
        "arbitrageur".to_string()
    } else {
        "retail".to_string()
    }
}

/// Calculate percentage change
pub fn calculate_percentage_change(old_value: &BigDecimal, new_value: &BigDecimal) -> BigDecimal {
    if old_value.is_zero() {
        return BigDecimal::from_str("0").unwrap();
    }
    
    ((new_value - old_value) / old_value) * BigDecimal::from_str("100").unwrap()
}

/// Format timestamp for day calculation
pub fn timestamp_to_day(timestamp: u64) -> u64 {
    timestamp / 86400 // Convert seconds to days
}

/// Calculate market volatility
pub fn calculate_volatility(prices: &[BigDecimal]) -> BigDecimal {
    if prices.len() < 2 {
        return BigDecimal::from_str("0").unwrap();
    }
    
    let mean = prices.iter().fold(BigDecimal::from_str("0").unwrap(), |acc, p| acc + p) 
        / BigDecimal::from(prices.len());
    
    let variance = prices.iter()
        .map(|p| (p - &mean).with_scale(10).square())
        .fold(BigDecimal::from_str("0").unwrap(), |acc, v| acc + v) 
        / BigDecimal::from(prices.len());
    
    variance.sqrt().unwrap_or_default()
}

/// Calculate liquidity score for a market
pub fn calculate_liquidity_score(
    total_volume: &BigDecimal,
    spread: &BigDecimal,
    depth: &BigDecimal,
) -> BigDecimal {
    if spread.is_zero() || depth.is_zero() {
        return BigDecimal::from_str("0").unwrap();
    }
    
    // Liquidity score = (Volume * Depth) / Spread
    (total_volume * depth) / spread
}

/// Detect unusual trading activity
pub fn detect_unusual_activity(
    current_volume: &BigDecimal,
    historical_avg: &BigDecimal,
    threshold_multiplier: f64,
) -> bool {
    let threshold = historical_avg * BigDecimal::from_str(&threshold_multiplier.to_string()).unwrap();
    current_volume > &threshold
}

/// Calculate Sharpe ratio for trader performance
pub fn calculate_sharpe_ratio(
    returns: &[BigDecimal],
    risk_free_rate: &BigDecimal,
) -> BigDecimal {
    if returns.is_empty() {
        return BigDecimal::from_str("0").unwrap();
    }
    
    let mean_return = returns.iter().fold(BigDecimal::from_str("0").unwrap(), |acc, r| acc + r) 
        / BigDecimal::from(returns.len());
    
    let excess_return = mean_return - risk_free_rate;
    
    if returns.len() < 2 {
        return excess_return;
    }
    
    let variance = returns.iter()
        .map(|r| (r - &mean_return).with_scale(10).square())
        .fold(BigDecimal::from_str("0").unwrap(), |acc, v| acc + v) 
        / BigDecimal::from(returns.len() - 1);
    
    let std_dev = variance.sqrt().unwrap_or_default();
    
    if std_dev.is_zero() {
        BigDecimal::from_str("0").unwrap()
    } else {
        excess_return / std_dev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_price() {
        let maker_amount = BigInt::from(1000u64);
        let taker_amount = BigInt::from(500u64);
        let price = calculate_price(&maker_amount, &taker_amount);
        assert_eq!(price, BigDecimal::from_str("2").unwrap());
    }
    
    #[test]
    fn test_determine_trade_side() {
        let maker_asset_even = BigInt::from(2u64); // USDC
        let taker_asset_odd = BigInt::from(3u64);  // Outcome token
        let maker_amount = BigInt::from(1000u64);
        let taker_amount = BigInt::from(500u64);
        
        let side = determine_trade_side(&maker_asset_even, &taker_asset_odd, &maker_amount, &taker_amount);
        assert_eq!(side, "buy");
    }
    
    #[test]
    fn test_calculate_volatility() {
        let prices = vec![
            BigDecimal::from_str("100").unwrap(),
            BigDecimal::from_str("105").unwrap(),
            BigDecimal::from_str("95").unwrap(),
            BigDecimal::from_str("110").unwrap(),
        ];
        let volatility = calculate_volatility(&prices);
        assert!(volatility > BigDecimal::from_str("0").unwrap());
    }
    
    #[test]
    fn test_detect_unusual_activity() {
        let current = BigDecimal::from_str("1000").unwrap();
        let historical = BigDecimal::from_str("100").unwrap();
        assert!(detect_unusual_activity(&current, &historical, 5.0));
        assert!(!detect_unusual_activity(&current, &historical, 15.0));
    }
}
