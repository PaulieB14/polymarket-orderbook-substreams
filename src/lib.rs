mod abi;
mod utils;

use substreams::errors::Error;
use substreams::log;
use substreams::store::{StoreGet, StoreNew, StoreSet};
use substreams::{hex};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

use crate::abi::{ctf_exchange, neg_risk_exchange};
use crate::utils::*;

// Import protobuf types
use crate::pb::polymarket::orderbook::v1::{
    Account, Accounts, GlobalOrderbookStats, MarketOrderbook, MarketOrderbooks,
    OrderFilledEvent, OrderFilledEvents, OrderbookAnalytics, OrdersMatchedEvent,
    OrdersMatchedEvents,
};

// Contract addresses
const CTF_EXCHANGE_ADDRESS: &str = "4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e";
const NEG_RISK_EXCHANGE_ADDRESS: &str = "C5d563A36AE78145C45a50134d48A1215220f80a";

/// Extract CTF Exchange OrderFilled events - optimized for parallel execution
#[substreams::handlers::map]
fn map_ctf_exchange_order_filled(blk: eth::Block) -> Result<OrderFilledEvents, Error> {
    let mut events = Vec::new();
    let mut ordinal = 0u64;

    for trx in blk.transaction_traces.iter() {
        for log in trx.receipt.as_ref().unwrap().logs.iter() {
            if hex::encode(&log.address) != CTF_EXCHANGE_ADDRESS {
                continue;
            }

            if let Some(event) = ctf_exchange::events::OrderFilled::match_and_decode(log) {
                let order_filled_event = OrderFilledEvent {
                    id: generate_order_id(&hex::encode(&trx.hash), &hex::encode(&event.order_hash)),
                    transaction_hash: hex::encode(&trx.hash),
                    timestamp: Some(blk.timestamp_seconds().into()),
                    order_hash: hex::encode(&event.order_hash),
                    maker: hex::encode(&event.maker),
                    taker: hex::encode(&event.taker),
                    maker_asset_id: event.maker_asset_id.to_string(),
                    taker_asset_id: event.taker_asset_id.to_string(),
                    maker_amount_filled: event.maker_amount_filled.to_string(),
                    taker_amount_filled: event.taker_amount_filled.to_string(),
                    fee: event.fee.to_string(),
                    block_number: blk.number,
                    side: determine_trade_side(
                        &event.maker_asset_id,
                        &event.taker_asset_id,
                        &event.maker_amount_filled,
                        &event.taker_amount_filled,
                    ),
                    price: calculate_price(&event.maker_amount_filled, &event.taker_amount_filled)
                        .to_string(),
                    ordinal,
                };

                events.push(order_filled_event);
                ordinal += 1;
                log::info!("CTF Exchange OrderFilled: {} (ordinal: {})", 
                    hex::encode(&event.order_hash), ordinal);
            }
        }
    }

    Ok(OrderFilledEvents {
        events,
        block_number: blk.number,
        block_hash: hex::encode(&blk.hash),
        timestamp: Some(blk.timestamp_seconds().into()),
    })
}

/// Extract Neg Risk Exchange OrderFilled events
#[substreams::handlers::map]
fn map_neg_risk_exchange_order_filled(blk: eth::Block) -> Result<OrderFilledEvents, Error> {
    let mut events = Vec::new();
    let mut ordinal = 0u64;

    for trx in blk.transaction_traces.iter() {
        for log in trx.receipt.as_ref().unwrap().logs.iter() {
            if hex::encode(&log.address) != NEG_RISK_EXCHANGE_ADDRESS {
                continue;
            }

            if let Some(event) = neg_risk_exchange::events::OrderFilled::match_and_decode(log) {
                let order_filled_event = OrderFilledEvent {
                    id: generate_order_id(&hex::encode(&trx.hash), &hex::encode(&event.order_hash)),
                    transaction_hash: hex::encode(&trx.hash),
                    timestamp: Some(blk.timestamp_seconds().into()),
                    order_hash: hex::encode(&event.order_hash),
                    maker: hex::encode(&event.maker),
                    taker: hex::encode(&event.taker),
                    maker_asset_id: event.maker_asset_id.to_string(),
                    taker_asset_id: event.taker_asset_id.to_string(),
                    maker_amount_filled: event.maker_amount_filled.to_string(),
                    taker_amount_filled: event.taker_amount_filled.to_string(),
                    fee: event.fee.to_string(),
                    block_number: blk.number,
                    side: determine_trade_side(
                        &event.maker_asset_id,
                        &event.taker_asset_id,
                        &event.maker_amount_filled,
                        &event.taker_amount_filled,
                    ),
                    price: calculate_price(&event.maker_amount_filled, &event.taker_amount_filled)
                        .to_string(),
                    ordinal,
                };

                events.push(order_filled_event);
                ordinal += 1;
                log::info!("Neg Risk Exchange OrderFilled: {} (ordinal: {})", 
                    hex::encode(&event.order_hash), ordinal);
            }
        }
    }

    Ok(OrderFilledEvents {
        events,
        block_number: blk.number,
        block_hash: hex::encode(&blk.hash),
        timestamp: Some(blk.timestamp_seconds().into()),
    })
}

/// Extract CTF Exchange OrdersMatched events
#[substreams::handlers::map]
fn map_ctf_exchange_orders_matched(blk: eth::Block) -> Result<OrdersMatchedEvents, Error> {
    let mut events = Vec::new();
    let mut ordinal = 0u64;

    for trx in blk.transaction_traces.iter() {
        for log in trx.receipt.as_ref().unwrap().logs.iter() {
            if hex::encode(&log.address) != CTF_EXCHANGE_ADDRESS {
                continue;
            }

            if let Some(event) = ctf_exchange::events::OrdersMatched::match_and_decode(log) {
                let orders_matched_event = OrdersMatchedEvent {
                    id: hex::encode(&trx.hash),
                    timestamp: Some(blk.timestamp_seconds().into()),
                    maker_asset_id: event.maker_asset_id.to_string(),
                    taker_asset_id: event.taker_asset_id.to_string(),
                    maker_amount_filled: event.maker_amount_filled.to_string(),
                    taker_amount_filled: event.taker_amount_filled.to_string(),
                    block_number: blk.number,
                    ordinal,
                };

                events.push(orders_matched_event);
                ordinal += 1;
                log::info!("CTF Exchange OrdersMatched: {} (ordinal: {})", 
                    hex::encode(&trx.hash), ordinal);
            }
        }
    }

    Ok(OrdersMatchedEvents {
        events,
        block_number: blk.number,
        block_hash: hex::encode(&blk.hash),
        timestamp: Some(blk.timestamp_seconds().into()),
    })
}

/// Extract Neg Risk Exchange OrdersMatched events
#[substreams::handlers::map]
fn map_neg_risk_exchange_orders_matched(blk: eth::Block) -> Result<OrdersMatchedEvents, Error> {
    let mut events = Vec::new();
    let mut ordinal = 0u64;

    for trx in blk.transaction_traces.iter() {
        for log in trx.receipt.as_ref().unwrap().logs.iter() {
            if hex::encode(&log.address) != NEG_RISK_EXCHANGE_ADDRESS {
                continue;
            }

            if let Some(event) = neg_risk_exchange::events::OrdersMatched::match_and_decode(log) {
                let orders_matched_event = OrdersMatchedEvent {
                    id: hex::encode(&trx.hash),
                    timestamp: Some(blk.timestamp_seconds().into()),
                    maker_asset_id: event.maker_asset_id.to_string(),
                    taker_asset_id: event.taker_asset_id.to_string(),
                    maker_amount_filled: event.maker_amount_filled.to_string(),
                    taker_amount_filled: event.taker_amount_filled.to_string(),
                    block_number: blk.number,
                    ordinal,
                };

                events.push(orders_matched_event);
                ordinal += 1;
                log::info!("Neg Risk Exchange OrdersMatched: {} (ordinal: {})", 
                    hex::encode(&trx.hash), ordinal);
            }
        }
    }

    Ok(OrdersMatchedEvents {
        events,
        block_number: blk.number,
        block_hash: hex::encode(&blk.hash),
        timestamp: Some(blk.timestamp_seconds().into()),
    })
}

/// Foundational store for markets - leverages parallel execution
#[substreams::handlers::store]
fn store_markets(
    ctf_order_filled: OrderFilledEvents,
    neg_risk_order_filled: OrderFilledEvents,
    ctf_orders_matched: OrdersMatchedEvents,
    neg_risk_orders_matched: OrdersMatchedEvents,
    store: StoreSet<MarketOrderbook>,
) {
    use std::collections::HashMap;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    let mut market_updates: HashMap<String, MarketOrderbook> = HashMap::new();

    // Process CTF order filled events
    for event in ctf_order_filled.events.iter() {
        let market_id = &event.maker_asset_id;
        let volume = BigDecimal::from_str(&event.taker_amount_filled).unwrap_or_default();
        
        let market = market_updates.entry(market_id.clone()).or_insert_with(|| {
            // Try to get existing market from store
            store.get_last(market_id)
                .unwrap_or_else(|| MarketOrderbook {
                    id: market_id.clone(),
                    condition_id: extract_condition_id(&event.maker_asset_id.parse().unwrap_or_default()),
                    trades_quantity: 0,
                    buys_quantity: 0,
                    sells_quantity: 0,
                    collateral_volume: "0".to_string(),
                    scaled_collateral_volume: "0".to_string(),
                    average_trade_size: "0".to_string(),
                    total_fees: "0".to_string(),
                    last_active_day: timestamp_to_day(ctf_order_filled.timestamp.as_ref().unwrap().seconds as u64),
                    mid_price: "0".to_string(),
                    spread: "0".to_string(),
                    volume_24h: "0".to_string(),
                    volume_7d: "0".to_string(),
                    price_change_24h: "0".to_string(),
                    volatility: "0".to_string(),
                    unique_traders_24h: 0,
                    bid_levels: vec![],
                    ask_levels: vec![],
                    liquidity_score: "0".to_string(),
                    market_depth: "0".to_string(),
                    last_updated_block: ctf_order_filled.block_number,
                })
        });

        market.trades_quantity += 1;
        if event.side == "buy" {
            market.buys_quantity += 1;
        } else if event.side == "sell" {
            market.sells_quantity += 1;
        }

        let current_volume = BigDecimal::from_str(&market.collateral_volume).unwrap_or_default();
        market.collateral_volume = (current_volume + &volume).to_string();
        
        let current_fees = BigDecimal::from_str(&market.total_fees).unwrap_or_default();
        let event_fee = BigDecimal::from_str(&event.fee).unwrap_or_default();
        market.total_fees = (current_fees + event_fee).to_string();

        market.average_trade_size = calculate_average_trade_size(
            &BigDecimal::from_str(&market.collateral_volume).unwrap_or_default(),
            market.trades_quantity
        ).to_string();

        market.last_active_day = timestamp_to_day(ctf_order_filled.timestamp.as_ref().unwrap().seconds as u64);
        market.last_updated_block = ctf_order_filled.block_number;
    }

    // Process Neg Risk order filled events (similar logic)
    for event in neg_risk_order_filled.events.iter() {
        let market_id = &event.maker_asset_id;
        let volume = BigDecimal::from_str(&event.taker_amount_filled).unwrap_or_default();
        
        let market = market_updates.entry(market_id.clone()).or_insert_with(|| {
            store.get_last(market_id)
                .unwrap_or_else(|| MarketOrderbook {
                    id: market_id.clone(),
                    condition_id: extract_condition_id(&event.maker_asset_id.parse().unwrap_or_default()),
                    trades_quantity: 0,
                    buys_quantity: 0,
                    sells_quantity: 0,
                    collateral_volume: "0".to_string(),
                    scaled_collateral_volume: "0".to_string(),
                    average_trade_size: "0".to_string(),
                    total_fees: "0".to_string(),
                    last_active_day: timestamp_to_day(neg_risk_order_filled.timestamp.as_ref().unwrap().seconds as u64),
                    mid_price: "0".to_string(),
                    spread: "0".to_string(),
                    volume_24h: "0".to_string(),
                    volume_7d: "0".to_string(),
                    price_change_24h: "0".to_string(),
                    volatility: "0".to_string(),
                    unique_traders_24h: 0,
                    bid_levels: vec![],
                    ask_levels: vec![],
                    liquidity_score: "0".to_string(),
                    market_depth: "0".to_string(),
                    last_updated_block: neg_risk_order_filled.block_number,
                })
        });

        market.trades_quantity += 1;
        if event.side == "buy" {
            market.buys_quantity += 1;
        } else if event.side == "sell" {
            market.sells_quantity += 1;
        }

        let current_volume = BigDecimal::from_str(&market.collateral_volume).unwrap_or_default();
        market.collateral_volume = (current_volume + &volume).to_string();

        let current_fees = BigDecimal::from_str(&market.total_fees).unwrap_or_default();
        let event_fee = BigDecimal::from_str(&event.fee).unwrap_or_default();
        market.total_fees = (current_fees + event_fee).to_string();

        market.average_trade_size = calculate_average_trade_size(
            &BigDecimal::from_str(&market.collateral_volume).unwrap_or_default(),
            market.trades_quantity
        ).to_string();

        market.last_active_day = timestamp_to_day(neg_risk_order_filled.timestamp.as_ref().unwrap().seconds as u64);
        market.last_updated_block = neg_risk_order_filled.block_number;
    }

    // Store all market updates
    for (market_id, market) in market_updates {
        store.set(0, &market_id, &market);
        log::info!("Updated market store for {}: {} trades", market_id, market.trades_quantity);
    }
}

/// Foundational store for traders
#[substreams::handlers::store]
fn store_traders(
    ctf_order_filled: OrderFilledEvents,
    neg_risk_order_filled: OrderFilledEvents,
    store: StoreSet<Account>,
) {
    use std::collections::HashMap;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    let mut trader_updates: HashMap<String, Account> = HashMap::new();

    // Process CTF order filled events
    for event in ctf_order_filled.events.iter() {
        // Process maker
        let maker_account = trader_updates.entry(event.maker.clone()).or_insert_with(|| {
            store.get_last(&event.maker)
                .unwrap_or_else(|| Account {
                    id: event.maker.clone(),
                    trades_quantity: 0,
                    total_volume: "0".to_string(),
                    total_fees: "0".to_string(),
                    first_trade: event.timestamp.clone(),
                    last_trade: event.timestamp.clone(),
                    is_active: true,
                    trader_type: "retail".to_string(),
                    volume_24h: "0".to_string(),
                    volume_7d: "0".to_string(),
                    pnl_realized: "0".to_string(),
                    pnl_unrealized: "0".to_string(),
                    markets_traded: 0,
                    win_rate: "0".to_string(),
                    sharpe_ratio: "0".to_string(),
                    max_drawdown: "0".to_string(),
                    position_size: "0".to_string(),
                    leverage: "1".to_string(),
                    risk_score: "0".to_string(),
                })
        });

        maker_account.trades_quantity += 1;
        let current_volume = BigDecimal::from_str(&maker_account.total_volume).unwrap_or_default();
        let event_volume = BigDecimal::from_str(&event.maker_amount_filled).unwrap_or_default();
        maker_account.total_volume = (current_volume + event_volume).to_string();

        maker_account.last_trade = event.timestamp.clone();
        maker_account.trader_type = classify_trader_type(
            maker_account.trades_quantity,
            &BigDecimal::from_str(&maker_account.total_volume).unwrap_or_default(),
            1,
        );

        // Process taker (similar logic)
        let taker_account = trader_updates.entry(event.taker.clone()).or_insert_with(|| {
            store.get_last(&event.taker)
                .unwrap_or_else(|| Account {
                    id: event.taker.clone(),
                    trades_quantity: 0,
                    total_volume: "0".to_string(),
                    total_fees: "0".to_string(),
                    first_trade: event.timestamp.clone(),
                    last_trade: event.timestamp.clone(),
                    is_active: true,
                    trader_type: "retail".to_string(),
                    volume_24h: "0".to_string(),
                    volume_7d: "0".to_string(),
                    pnl_realized: "0".to_string(),
                    pnl_unrealized: "0".to_string(),
                    markets_traded: 0,
                    win_rate: "0".to_string(),
                    sharpe_ratio: "0".to_string(),
                    max_drawdown: "0".to_string(),
                    position_size: "0".to_string(),
                    leverage: "1".to_string(),
                    risk_score: "0".to_string(),
                })
        });

        taker_account.trades_quantity += 1;
        let current_volume = BigDecimal::from_str(&taker_account.total_volume).unwrap_or_default();
        let event_volume = BigDecimal::from_str(&event.taker_amount_filled).unwrap_or_default();
        taker_account.total_volume = (current_volume + event_volume).to_string();

        taker_account.last_trade = event.timestamp.clone();
        taker_account.trader_type = classify_trader_type(
            taker_account.trades_quantity,
            &BigDecimal::from_str(&taker_account.total_volume).unwrap_or_default(),
            1,
        );
    }

    // Store all trader updates
    for (trader_id, trader) in trader_updates {
        store.set(0, &trader_id, &trader);
        log::info!("Updated trader store for {}: {} trades", trader_id, trader.trades_quantity);
    }
}

/// Foundational store for global stats
#[substreams::handlers::store]
fn store_global_stats(
    markets_store: StoreGet<MarketOrderbook>,
    traders_store: StoreGet<Account>,
    store: StoreSet<GlobalOrderbookStats>,
) {
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    let mut total_trades = 0u64;
    let mut total_volume = BigDecimal::from_str("0").unwrap();
    let mut active_markets = 0u64;
    let mut unique_traders = 0u64;

    // Aggregate from markets store
    markets_store.scan_prefix(0, "", |_key, market| {
        total_trades += market.trades_quantity;
        let market_volume = BigDecimal::from_str(&market.collateral_volume).unwrap_or_default();
        total_volume += market_volume;
        if market.trades_quantity > 0 {
            active_markets += 1;
        }
    });

    // Count unique traders
    traders_store.scan_prefix(0, "", |_key, _trader| {
        unique_traders += 1;
    });

    let global_stats = GlobalOrderbookStats {
        id: "global".to_string(),
        trades_quantity: total_trades,
        buys_quantity: 0, // Would need more complex aggregation
        sells_quantity: 0,
        collateral_volume: total_volume.to_string(),
        scaled_collateral_volume: bigint_to_scaled_decimal(
            &total_volume.to_string().parse().unwrap_or_default(),
            6
        ).to_string(),
        total_fees: "0".to_string(), // Would aggregate from markets
        average_trade_size: calculate_average_trade_size(&total_volume, total_trades).to_string(),
        unique_traders,
        active_markets,
        last_updated: None,
        total_liquidity: "0".to_string(),
        market_cap: "0".to_string(),
        volume_24h: "0".to_string(),
        volume_7d: "0".to_string(),
        new_traders_24h: 0,
        new_markets_24h: 0,
        average_spread: "0".to_string(),
        platform_fee_revenue: "0".to_string(),
        maker_taker_ratio: "0".to_string(),
    };

    store.set(0, "global", &global_stats);
    log::info!("Updated global stats: {} trades, {} markets, {} traders", 
        total_trades, active_markets, unique_traders);
}

/// Output market orderbooks from store
#[substreams::handlers::map]
fn map_market_orderbooks(markets_store: StoreGet<MarketOrderbook>) -> Result<MarketOrderbooks, Error> {
    let mut orderbooks = Vec::new();

    markets_store.scan_prefix(0, "", |_key, market| {
        orderbooks.push(market.clone());
    });

    Ok(MarketOrderbooks {
        orderbooks,
        block_number: 0, // Would need to track current block
        block_hash: "".to_string(),
        timestamp: None,
    })
}

/// Output trader accounts from store
#[substreams::handlers::map]
fn map_trader_accounts(traders_store: StoreGet<Account>) -> Result<Accounts, Error> {
    let mut accounts = Vec::new();

    traders_store.scan_prefix(0, "", |_key, trader| {
        accounts.push(trader.clone());
    });

    Ok(Accounts {
        accounts,
        block_number: 0,
        block_hash: "".to_string(),
        timestamp: None,
    })
}

/// Output global stats from store
#[substreams::handlers::map]
fn map_global_orderbook_stats(global_store: StoreGet<GlobalOrderbookStats>) -> Result<GlobalOrderbookStats, Error> {
    Ok(global_store.get_last("global")
        .unwrap_or_else(|| GlobalOrderbookStats {
            id: "global".to_string(),
            trades_quantity: 0,
            buys_quantity: 0,
            sells_quantity: 0,
            collateral_volume: "0".to_string(),
            scaled_collateral_volume: "0".to_string(),
            total_fees: "0".to_string(),
            average_trade_size: "0".to_string(),
            unique_traders: 0,
            active_markets: 0,
            last_updated: None,
            total_liquidity: "0".to_string(),
            market_cap: "0".to_string(),
            volume_24h: "0".to_string(),
            volume_7d: "0".to_string(),
            new_traders_24h: 0,
            new_markets_24h: 0,
            average_spread: "0".to_string(),
            platform_fee_revenue: "0".to_string(),
            maker_taker_ratio: "0".to_string(),
        }))
}

/// Combine all analytics into final output - leverages foundational stores
#[substreams::handlers::map]
fn map_orderbook_analytics(
    market_orderbooks: MarketOrderbooks,
    trader_accounts: Accounts,
    global_stats: GlobalOrderbookStats,
) -> Result<OrderbookAnalytics, Error> {
    // Get top 10 traders by volume
    let mut top_traders = trader_accounts.accounts.clone();
    top_traders.sort_by(|a, b| {
        let a_volume = a.total_volume.parse::<f64>().unwrap_or(0.0);
        let b_volume = b.total_volume.parse::<f64>().unwrap_or(0.0);
        b_volume.partial_cmp(&a_volume).unwrap_or(std::cmp::Ordering::Equal)
    });
    top_traders.truncate(10);

    Ok(OrderbookAnalytics {
        market_orderbooks: market_orderbooks.orderbooks,
        global_stats: Some(global_stats),
        top_traders,
        block_number: market_orderbooks.block_number,
        block_hash: market_orderbooks.block_hash,
        timestamp: market_orderbooks.timestamp,
        market_alerts: vec![], // Would implement alert detection
        arbitrage_opportunities: vec![], // Would implement arbitrage detection
        sentiment: None, // Would implement sentiment analysis
    })
}

// Protobuf generated code will be included here
substreams_ethereum::init!();

#[path = "pb/polymarket.orderbook.v1.rs"]
#[allow(dead_code)]
pub mod pb;
