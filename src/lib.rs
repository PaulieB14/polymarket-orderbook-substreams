use substreams::Hex;
use substreams::store::{StoreSetProto, StoreGetProto, DeltaProto, Deltas};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{DatabaseChanges, table_change::Operation};
use prost_types::Timestamp;
use bigdecimal::BigDecimal;
use std::str::FromStr;

mod abi;
mod utils;

#[path = "pb/mod.rs"]
#[allow(dead_code)]
pub mod pb;

use pb::polymarket::orderbook::v1::{
    OrderFilledEvent, OrderFilledEvents, OrdersMatchedEvent, OrdersMatchedEvents,
    MarketOrderbook, MarketOrderbooks, Account, Accounts,
    GlobalOrderbookStats, OrderbookAnalytics,
};

substreams_ethereum::init!();

/// Extract OrderFilled events from CTF Exchange
#[substreams::handlers::map]
pub fn map_ctf_exchange_order_filled(blk: eth::Block) -> Result<OrderFilledEvents, substreams::errors::Error> {
    let mut events = vec![];

    for trx in &blk.transaction_traces {
        for call in &trx.calls {
            for log in &call.logs {
                // CTF Exchange contract address
                if log.address != Hex::decode("4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e").unwrap() {
                    continue;
                }

                if let Some(event) = abi::ctf_exchange::events::OrderFilled::match_and_decode(&log) {
                    let order_id = utils::generate_order_id(&Hex::encode(&trx.hash), &Hex::encode(&event.order_hash));
                    
                    events.push(OrderFilledEvent {
                        id: order_id,
                        transaction_hash: Hex::encode(&trx.hash),
                        timestamp: Some(Timestamp {
                            seconds: blk.timestamp_seconds() as i64,
                            nanos: 0,
                        }),
                        order_hash: Hex::encode(&event.order_hash),
                        maker: Hex::encode(&event.maker),
                        taker: Hex::encode(&event.taker),
                        maker_asset_id: event.maker_asset_id.to_string(),
                        taker_asset_id: event.taker_asset_id.to_string(),
                        maker_amount_filled: event.maker_amount_filled.to_string(),
                        taker_amount_filled: event.taker_amount_filled.to_string(),
                        fee: event.fee.to_string(),
                        block_number: blk.number,
                        side: utils::determine_trade_side(
                            &event.maker_asset_id,
                            &event.taker_asset_id,
                            &event.maker_amount_filled,
                            &event.taker_amount_filled,
                        ),
                        price: utils::calculate_price(&event.maker_amount_filled, &event.taker_amount_filled).to_string(),
                        ordinal: log.ordinal,
                    });
                }
            }
        }
    }

    Ok(OrderFilledEvents { 
        events,
        block_number: blk.number,
        block_hash: Hex::encode(&blk.hash),
        timestamp: Some(Timestamp {
            seconds: blk.timestamp_seconds() as i64,
            nanos: 0,
        }),
    })
}

/// Extract OrderFilled events from Neg Risk Exchange
#[substreams::handlers::map]
pub fn map_neg_risk_exchange_order_filled(blk: eth::Block) -> Result<OrderFilledEvents, substreams::errors::Error> {
    let mut events = vec![];

    for trx in &blk.transaction_traces {
        for call in &trx.calls {
            for log in &call.logs {
                // Neg Risk Exchange contract address
                if log.address != Hex::decode("C5d563A36AE78145C45a50134d48A1215220f80a").unwrap() {
                    continue;
                }

                if let Some(event) = abi::neg_risk_exchange::events::OrderFilled::match_and_decode(&log) {
                    let order_id = utils::generate_order_id(&Hex::encode(&trx.hash), &Hex::encode(&event.order_hash));
                    
                    events.push(OrderFilledEvent {
                        id: order_id,
                        transaction_hash: Hex::encode(&trx.hash),
                        timestamp: Some(Timestamp {
                            seconds: blk.timestamp_seconds() as i64,
                            nanos: 0,
                        }),
                        order_hash: Hex::encode(&event.order_hash),
                        maker: Hex::encode(&event.maker),
                        taker: Hex::encode(&event.taker),
                        maker_asset_id: event.maker_asset_id.to_string(),
                        taker_asset_id: event.taker_asset_id.to_string(),
                        maker_amount_filled: event.maker_amount_filled.to_string(),
                        taker_amount_filled: event.taker_amount_filled.to_string(),
                        fee: event.fee.to_string(),
                        block_number: blk.number,
                        side: utils::determine_trade_side(
                            &event.maker_asset_id,
                            &event.taker_asset_id,
                            &event.maker_amount_filled,
                            &event.taker_amount_filled,
                        ),
                        price: utils::calculate_price(&event.maker_amount_filled, &event.taker_amount_filled).to_string(),
                        ordinal: log.ordinal,
                    });
                }
            }
        }
    }

    Ok(OrderFilledEvents { 
        events,
        block_number: blk.number,
        block_hash: Hex::encode(&blk.hash),
        timestamp: Some(Timestamp {
            seconds: blk.timestamp_seconds() as i64,
            nanos: 0,
        }),
    })
}

/// Extract OrdersMatched events from CTF Exchange
#[substreams::handlers::map]
pub fn map_ctf_exchange_orders_matched(blk: eth::Block) -> Result<OrdersMatchedEvents, substreams::errors::Error> {
    let mut events = vec![];

    for trx in &blk.transaction_traces {
        for call in &trx.calls {
            for log in &call.logs {
                // CTF Exchange contract address
                if log.address != Hex::decode("4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e").unwrap() {
                    continue;
                }

                if let Some(event) = abi::ctf_exchange::events::OrdersMatched::match_and_decode(&log) {
                    let event_id = format!("{}-{}", Hex::encode(&trx.hash), log.ordinal);
                    
                    events.push(OrdersMatchedEvent {
                        id: event_id,
                        timestamp: Some(Timestamp {
                            seconds: blk.timestamp_seconds() as i64,
                            nanos: 0,
                        }),
                        maker_asset_id: event.maker_asset_id.to_string(),
                        taker_asset_id: event.taker_asset_id.to_string(),
                        maker_amount_filled: event.maker_amount_filled.to_string(),
                        taker_amount_filled: event.taker_amount_filled.to_string(),
                        block_number: blk.number,
                        ordinal: log.ordinal,
                    });
                }
            }
        }
    }

    Ok(OrdersMatchedEvents { 
        events,
        block_number: blk.number,
        block_hash: Hex::encode(&blk.hash),
        timestamp: Some(Timestamp {
            seconds: blk.timestamp_seconds() as i64,
            nanos: 0,
        }),
    })
}

/// Extract OrdersMatched events from Neg Risk Exchange
#[substreams::handlers::map]
pub fn map_neg_risk_exchange_orders_matched(blk: eth::Block) -> Result<OrdersMatchedEvents, substreams::errors::Error> {
    let mut events = vec![];

    for trx in &blk.transaction_traces {
        for call in &trx.calls {
            for log in &call.logs {
                // Neg Risk Exchange contract address
                if log.address != Hex::decode("C5d563A36AE78145C45a50134d48A1215220f80a").unwrap() {
                    continue;
                }

                if let Some(event) = abi::neg_risk_exchange::events::OrdersMatched::match_and_decode(&log) {
                    let event_id = format!("{}-{}", Hex::encode(&trx.hash), log.ordinal);

                    events.push(OrdersMatchedEvent {
                        id: event_id,
                        timestamp: Some(Timestamp {
                            seconds: blk.timestamp_seconds() as i64,
                            nanos: 0,
                        }),
                        maker_asset_id: event.maker_asset_id.to_string(),
                        taker_asset_id: event.taker_asset_id.to_string(),
                        maker_amount_filled: event.maker_amount_filled.to_string(),
                        taker_amount_filled: event.taker_amount_filled.to_string(),
                        block_number: blk.number,
                        ordinal: log.ordinal,
                    });
                }
            }
        }
    }

    Ok(OrdersMatchedEvents {
        events,
        block_number: blk.number,
        block_hash: Hex::encode(&blk.hash),
        timestamp: Some(Timestamp {
            seconds: blk.timestamp_seconds() as i64,
            nanos: 0,
        }),
    })
}

// ============================================
// Combined Events Module (Layer 1.5)
// ============================================

/// Combines order fills from both CTF Exchange and Neg Risk Exchange
#[substreams::handlers::map]
pub fn map_all_order_fills(
    ctf_fills: OrderFilledEvents,
    neg_risk_fills: OrderFilledEvents,
) -> Result<OrderFilledEvents, substreams::errors::Error> {
    let mut all_events = ctf_fills.events;
    all_events.extend(neg_risk_fills.events);

    // Sort by ordinal for deterministic ordering
    all_events.sort_by_key(|e| e.ordinal);

    Ok(OrderFilledEvents {
        events: all_events,
        block_number: ctf_fills.block_number,
        block_hash: ctf_fills.block_hash,
        timestamp: ctf_fills.timestamp,
    })
}

// ============================================
// Foundational Stores (Layer 2)
// ============================================

/// Store tracking market-level statistics
#[substreams::handlers::store]
pub fn store_markets(fills: OrderFilledEvents, store: StoreSetProto<MarketOrderbook>) {
    for fill in &fills.events {
        let market_id = format!("market:{}", fill.maker_asset_id);

        // Get existing or create new market orderbook
        let mut orderbook = store.get_last(&market_id).unwrap_or_else(|| MarketOrderbook {
            id: market_id.clone(),
            condition_id: utils::extract_condition_id_from_str(&fill.maker_asset_id),
            trades_quantity: 0,
            buys_quantity: 0,
            sells_quantity: 0,
            collateral_volume: "0".to_string(),
            scaled_collateral_volume: "0".to_string(),
            average_trade_size: "0".to_string(),
            total_fees: "0".to_string(),
            last_active_day: 0,
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
            last_updated_block: 0,
        });

        // Update market stats
        orderbook.trades_quantity += 1;
        if fill.side == "buy" {
            orderbook.buys_quantity += 1;
        } else if fill.side == "sell" {
            orderbook.sells_quantity += 1;
        }

        // Update volume
        let fill_volume = BigDecimal::from_str(&fill.taker_amount_filled).unwrap_or_default();
        let current_volume = BigDecimal::from_str(&orderbook.collateral_volume).unwrap_or_default();
        orderbook.collateral_volume = (current_volume + fill_volume.clone()).to_string();

        // Update fees
        let fill_fee = BigDecimal::from_str(&fill.fee).unwrap_or_default();
        let current_fees = BigDecimal::from_str(&orderbook.total_fees).unwrap_or_default();
        orderbook.total_fees = (current_fees + fill_fee).to_string();

        // Update price
        orderbook.mid_price = fill.price.clone();

        // Update last active
        if let Some(ts) = &fill.timestamp {
            orderbook.last_active_day = utils::timestamp_to_day(ts.seconds as u64);
        }
        orderbook.last_updated_block = fill.block_number;

        // Calculate average trade size
        let total_volume = BigDecimal::from_str(&orderbook.collateral_volume).unwrap_or_default();
        if orderbook.trades_quantity > 0 {
            let avg = total_volume / BigDecimal::from(orderbook.trades_quantity);
            orderbook.average_trade_size = avg.to_string();
        }

        store.set(0, &market_id, &orderbook);
    }
}

/// Store tracking trader-level analytics
#[substreams::handlers::store]
pub fn store_traders(fills: OrderFilledEvents, store: StoreSetProto<Account>) {
    for fill in &fills.events {
        // Process maker
        let maker_id = format!("trader:{}", fill.maker);
        process_trader_fill(&maker_id, fill, true, &store);

        // Process taker
        let taker_id = format!("trader:{}", fill.taker);
        process_trader_fill(&taker_id, fill, false, &store);
    }
}

fn process_trader_fill(
    trader_id: &str,
    fill: &OrderFilledEvent,
    is_maker: bool,
    store: &StoreSetProto<Account>
) {
    let mut account = store.get_last(trader_id).unwrap_or_else(|| Account {
        id: trader_id.to_string(),
        trades_quantity: 0,
        total_volume: "0".to_string(),
        total_fees: "0".to_string(),
        first_trade: fill.timestamp.clone(),
        last_trade: None,
        is_active: true,
        trader_type: if is_maker { "maker".to_string() } else { "taker".to_string() },
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
    });

    // Update stats
    account.trades_quantity += 1;
    account.last_trade = fill.timestamp.clone();
    account.is_active = true;

    // Update volume
    let fill_volume = BigDecimal::from_str(&fill.taker_amount_filled).unwrap_or_default();
    let current_volume = BigDecimal::from_str(&account.total_volume).unwrap_or_default();
    account.total_volume = (current_volume + fill_volume).to_string();

    // Update fees (only for taker)
    if !is_maker {
        let fill_fee = BigDecimal::from_str(&fill.fee).unwrap_or_default();
        let current_fees = BigDecimal::from_str(&account.total_fees).unwrap_or_default();
        account.total_fees = (current_fees + fill_fee).to_string();
    }

    store.set(0, trader_id, &account);
}

/// Store tracking platform-wide statistics
#[substreams::handlers::store]
pub fn store_global_stats(fills: OrderFilledEvents, store: StoreSetProto<GlobalOrderbookStats>) {
    if fills.events.is_empty() {
        return;
    }

    let global_key = "global";

    let mut stats = store.get_last(global_key).unwrap_or_else(|| GlobalOrderbookStats {
        id: global_key.to_string(),
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
        maker_taker_ratio: "1".to_string(),
    });

    for fill in &fills.events {
        // Update trade counts
        stats.trades_quantity += 1;
        if fill.side == "buy" {
            stats.buys_quantity += 1;
        } else if fill.side == "sell" {
            stats.sells_quantity += 1;
        }

        // Update volume
        let fill_volume = BigDecimal::from_str(&fill.taker_amount_filled).unwrap_or_default();
        let current_volume = BigDecimal::from_str(&stats.collateral_volume).unwrap_or_default();
        stats.collateral_volume = (current_volume + fill_volume).to_string();

        // Update fees
        let fill_fee = BigDecimal::from_str(&fill.fee).unwrap_or_default();
        let current_fees = BigDecimal::from_str(&stats.total_fees).unwrap_or_default();
        stats.total_fees = (current_fees + fill_fee.clone()).to_string();

        // Update platform fee revenue (same as total fees for now)
        let platform_fees = BigDecimal::from_str(&stats.platform_fee_revenue).unwrap_or_default();
        stats.platform_fee_revenue = (platform_fees + fill_fee).to_string();

        // Update timestamp
        stats.last_updated = fill.timestamp.clone();
    }

    // Calculate average trade size
    let total_volume = BigDecimal::from_str(&stats.collateral_volume).unwrap_or_default();
    if stats.trades_quantity > 0 {
        let avg = total_volume / BigDecimal::from(stats.trades_quantity);
        stats.average_trade_size = avg.to_string();
    }

    store.set(0, global_key, &stats);
}

// ============================================
// Analytics Modules (Layer 3)
// ============================================

/// Outputs market orderbook snapshots on each store update
#[substreams::handlers::map]
pub fn map_market_orderbooks(
    deltas: Deltas<DeltaProto<MarketOrderbook>>,
) -> Result<MarketOrderbooks, substreams::errors::Error> {
    let orderbooks: Vec<MarketOrderbook> = deltas
        .deltas
        .into_iter()
        .filter_map(|delta| delta.new_value)
        .collect();

    let block_number = orderbooks.first().map(|o| o.last_updated_block).unwrap_or(0);

    Ok(MarketOrderbooks {
        orderbooks,
        block_number,
        block_hash: String::new(),
        timestamp: None,
    })
}

/// Outputs trader account updates on each store update
#[substreams::handlers::map]
pub fn map_trader_accounts(
    deltas: Deltas<DeltaProto<Account>>,
) -> Result<Accounts, substreams::errors::Error> {
    let accounts: Vec<Account> = deltas
        .deltas
        .into_iter()
        .filter_map(|delta| delta.new_value)
        .collect();

    Ok(Accounts {
        accounts,
        block_number: 0,
        block_hash: String::new(),
        timestamp: None,
    })
}

/// Outputs global platform statistics on each store update
#[substreams::handlers::map]
pub fn map_global_orderbook_stats(
    deltas: Deltas<DeltaProto<GlobalOrderbookStats>>,
) -> Result<GlobalOrderbookStats, substreams::errors::Error> {
    // Return the latest global stats
    deltas
        .deltas
        .into_iter()
        .filter_map(|delta| delta.new_value)
        .last()
        .ok_or_else(|| substreams::errors::Error::msg("No global stats updates"))
}

/// Comprehensive orderbook analytics combining all stores
#[substreams::handlers::map]
pub fn map_orderbook_analytics(
    clock: Clock,
    markets_store: StoreGetProto<MarketOrderbook>,
    traders_store: StoreGetProto<Account>,
    global_store: StoreGetProto<GlobalOrderbookStats>,
) -> Result<OrderbookAnalytics, substreams::errors::Error> {
    // Get global stats
    let global_stats = global_store.get_last("global");

    // This module provides a snapshot of the current state
    // In a real implementation, you'd query specific keys or use store deltas
    Ok(OrderbookAnalytics {
        market_orderbooks: vec![],  // Would be populated from store queries
        global_stats,
        top_traders: vec![],  // Would be populated from store queries
        block_number: clock.number,
        block_hash: clock.id,
        timestamp: clock.timestamp,
        market_alerts: vec![],
        arbitrage_opportunities: vec![],
        sentiment: None,
    })
}

// ============================================
// Database Sink Modules (Layer 4)
// ============================================

/// SQL Sink module - outputs DatabaseChanges for PostgreSQL
#[substreams::handlers::map]
pub fn db_out(
    fills: OrderFilledEvents,
    market_deltas: Deltas<DeltaProto<MarketOrderbook>>,
    trader_deltas: Deltas<DeltaProto<Account>>,
    global_deltas: Deltas<DeltaProto<GlobalOrderbookStats>>,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut changes = DatabaseChanges::default();

    // Process order fills
    for fill in &fills.events {
        changes
            .push_change("order_fills", &fill.id, 0, Operation::Create)
            .change("transaction_hash", ("", &fill.transaction_hash))
            .change("order_hash", ("", &fill.order_hash))
            .change("maker", ("", &fill.maker))
            .change("taker", ("", &fill.taker))
            .change("maker_asset_id", ("", &fill.maker_asset_id))
            .change("taker_asset_id", ("", &fill.taker_asset_id))
            .change("maker_amount_filled", ("", &fill.maker_amount_filled))
            .change("taker_amount_filled", ("", &fill.taker_amount_filled))
            .change("fee", ("", &fill.fee))
            .change("side", ("", &fill.side))
            .change("price", ("", &fill.price))
            .change("block_number", ("", fill.block_number.to_string().as_str()));
    }

    // Process market orderbook updates
    for delta in &market_deltas.deltas {
        if let Some(new_market) = &delta.new_value {
            let op = if delta.old_value.is_some() { Operation::Update } else { Operation::Create };
            changes
                .push_change("market_orderbooks", &new_market.id, 0, op)
                .change("condition_id", ("", &new_market.condition_id))
                .change("trades_quantity", ("", new_market.trades_quantity.to_string().as_str()))
                .change("buys_quantity", ("", new_market.buys_quantity.to_string().as_str()))
                .change("sells_quantity", ("", new_market.sells_quantity.to_string().as_str()))
                .change("collateral_volume", ("", &new_market.collateral_volume))
                .change("average_trade_size", ("", &new_market.average_trade_size))
                .change("total_fees", ("", &new_market.total_fees))
                .change("mid_price", ("", &new_market.mid_price))
                .change("last_updated_block", ("", new_market.last_updated_block.to_string().as_str()));
        }
    }

    // Process trader account updates
    for delta in &trader_deltas.deltas {
        if let Some(new_account) = &delta.new_value {
            let op = if delta.old_value.is_some() { Operation::Update } else { Operation::Create };
            changes
                .push_change("trader_accounts", &new_account.id, 0, op)
                .change("trades_quantity", ("", new_account.trades_quantity.to_string().as_str()))
                .change("total_volume", ("", &new_account.total_volume))
                .change("total_fees", ("", &new_account.total_fees))
                .change("is_active", ("", new_account.is_active.to_string().as_str()))
                .change("trader_type", ("", &new_account.trader_type));
        }
    }

    // Process global stats updates
    for delta in &global_deltas.deltas {
        if let Some(new_stats) = &delta.new_value {
            changes
                .push_change("global_stats", "global", 0, Operation::Update)
                .change("trades_quantity", ("", new_stats.trades_quantity.to_string().as_str()))
                .change("buys_quantity", ("", new_stats.buys_quantity.to_string().as_str()))
                .change("sells_quantity", ("", new_stats.sells_quantity.to_string().as_str()))
                .change("collateral_volume", ("", &new_stats.collateral_volume))
                .change("total_fees", ("", &new_stats.total_fees))
                .change("average_trade_size", ("", &new_stats.average_trade_size))
                .change("platform_fee_revenue", ("", &new_stats.platform_fee_revenue));
        }
    }

    Ok(changes)
}

/// Clickhouse Sink module - optimized for analytics queries
#[substreams::handlers::map]
pub fn clickhouse_out(
    fills: OrderFilledEvents,
    market_deltas: Deltas<DeltaProto<MarketOrderbook>>,
    trader_deltas: Deltas<DeltaProto<Account>>,
    global_deltas: Deltas<DeltaProto<GlobalOrderbookStats>>,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut changes = DatabaseChanges::default();

    // For Clickhouse, we denormalize data for faster analytics
    for fill in &fills.events {
        let timestamp_str = fill.timestamp
            .as_ref()
            .map(|t| t.seconds.to_string())
            .unwrap_or_default();

        changes
            .push_change("order_fills", &fill.id, 0, Operation::Create)
            .change("transaction_hash", ("", &fill.transaction_hash))
            .change("block_number", ("", fill.block_number.to_string().as_str()))
            .change("block_timestamp", ("", &timestamp_str))
            .change("order_hash", ("", &fill.order_hash))
            .change("maker", ("", &fill.maker))
            .change("taker", ("", &fill.taker))
            .change("maker_asset_id", ("", &fill.maker_asset_id))
            .change("taker_asset_id", ("", &fill.taker_asset_id))
            .change("maker_amount_filled", ("", &fill.maker_amount_filled))
            .change("taker_amount_filled", ("", &fill.taker_amount_filled))
            .change("fee", ("", &fill.fee))
            .change("side", ("", &fill.side))
            .change("price", ("", &fill.price));
    }

    // Market analytics for Clickhouse (denormalized)
    for delta in &market_deltas.deltas {
        if let Some(market) = &delta.new_value {
            changes
                .push_change("market_analytics", &market.id, 0, Operation::Create)
                .change("condition_id", ("", &market.condition_id))
                .change("trades_quantity", ("", market.trades_quantity.to_string().as_str()))
                .change("buys_quantity", ("", market.buys_quantity.to_string().as_str()))
                .change("sells_quantity", ("", market.sells_quantity.to_string().as_str()))
                .change("collateral_volume", ("", &market.collateral_volume))
                .change("average_trade_size", ("", &market.average_trade_size))
                .change("total_fees", ("", &market.total_fees))
                .change("mid_price", ("", &market.mid_price))
                .change("volume_24h", ("", &market.volume_24h))
                .change("volume_7d", ("", &market.volume_7d))
                .change("liquidity_score", ("", &market.liquidity_score))
                .change("last_updated_block", ("", market.last_updated_block.to_string().as_str()));
        }
    }

    // Trader analytics for Clickhouse
    for delta in &trader_deltas.deltas {
        if let Some(account) = &delta.new_value {
            changes
                .push_change("trader_analytics", &account.id, 0, Operation::Create)
                .change("trades_quantity", ("", account.trades_quantity.to_string().as_str()))
                .change("total_volume", ("", &account.total_volume))
                .change("total_fees", ("", &account.total_fees))
                .change("volume_24h", ("", &account.volume_24h))
                .change("volume_7d", ("", &account.volume_7d))
                .change("markets_traded", ("", account.markets_traded.to_string().as_str()))
                .change("is_active", ("", account.is_active.to_string().as_str()))
                .change("trader_type", ("", &account.trader_type));
        }
    }

    // Global stats for Clickhouse
    for delta in &global_deltas.deltas {
        if let Some(stats) = &delta.new_value {
            changes
                .push_change("global_analytics", "global", 0, Operation::Create)
                .change("trades_quantity", ("", stats.trades_quantity.to_string().as_str()))
                .change("buys_quantity", ("", stats.buys_quantity.to_string().as_str()))
                .change("sells_quantity", ("", stats.sells_quantity.to_string().as_str()))
                .change("collateral_volume", ("", &stats.collateral_volume))
                .change("total_fees", ("", &stats.total_fees))
                .change("unique_traders", ("", stats.unique_traders.to_string().as_str()))
                .change("active_markets", ("", stats.active_markets.to_string().as_str()))
                .change("volume_24h", ("", &stats.volume_24h))
                .change("volume_7d", ("", &stats.volume_7d))
                .change("platform_fee_revenue", ("", &stats.platform_fee_revenue));
        }
    }

    Ok(changes)
}
