use substreams::Hex;
use substreams::store::{StoreNew, StoreSet, StoreSetProto, StoreGet, StoreGetProto, Deltas, DeltaProto};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use prost_types::Timestamp;
use std::str::FromStr;
use bigdecimal::BigDecimal;

mod abi;
mod utils;

#[path = "pb/mod.rs"]
#[allow(dead_code)]
pub mod pb;

use pb::polymarket::orderbook::v1::{
    OrderFilledEvent, OrderFilledEvents, OrdersMatchedEvent, OrdersMatchedEvents,
    MarketOrderbook, MarketOrderbooks, Account, Accounts, GlobalOrderbookStats,
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

/// Store market-level statistics indexed by asset ID
#[substreams::handlers::store]
pub fn store_markets(events: OrderFilledEvents, store: StoreSetProto<MarketOrderbook>) {
    for event in events.events {
        let market_id = utils::extract_condition_id_from_str(&event.maker_asset_id);

        // Get existing or create new market orderbook
        let mut orderbook = MarketOrderbook {
            id: market_id.clone(),
            condition_id: market_id.clone(),
            trades_quantity: 1,
            buys_quantity: if event.side == "buy" { 1 } else { 0 },
            sells_quantity: if event.side == "sell" { 1 } else { 0 },
            collateral_volume: event.taker_amount_filled.clone(),
            scaled_collateral_volume: scale_amount(&event.taker_amount_filled),
            average_trade_size: event.taker_amount_filled.clone(),
            total_fees: event.fee.clone(),
            last_active_day: utils::timestamp_to_day(event.timestamp.as_ref().map(|t| t.seconds as u64).unwrap_or(0)),
            mid_price: event.price.clone(),
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
            last_updated_block: events.block_number,
        };

        store.set(event.ordinal, format!("market:{}", market_id), &orderbook);
    }
}

/// Store trader-level statistics indexed by address
#[substreams::handlers::store]
pub fn store_traders(events: OrderFilledEvents, store: StoreSetProto<Account>) {
    for event in events.events {
        // Store for both maker and taker
        for (address, is_maker) in [(&event.maker, true), (&event.taker, false)] {
            let account = Account {
                id: address.clone(),
                trades_quantity: 1,
                total_volume: event.taker_amount_filled.clone(),
                total_fees: if is_maker { "0".to_string() } else { event.fee.clone() },
                first_trade: event.timestamp.clone(),
                last_trade: event.timestamp.clone(),
                is_active: true,
                trader_type: if is_maker { "maker".to_string() } else { "taker".to_string() },
                volume_24h: "0".to_string(),
                volume_7d: "0".to_string(),
                pnl_realized: "0".to_string(),
                pnl_unrealized: "0".to_string(),
                markets_traded: 1,
                win_rate: "0".to_string(),
                sharpe_ratio: "0".to_string(),
                max_drawdown: "0".to_string(),
                position_size: "0".to_string(),
                leverage: "1".to_string(),
                risk_score: "0".to_string(),
            };

            store.set(event.ordinal, format!("trader:{}", address), &account);
        }
    }
}

/// Store global platform statistics
#[substreams::handlers::store]
pub fn store_global_stats(events: OrderFilledEvents, store: StoreSetProto<GlobalOrderbookStats>) {
    if events.events.is_empty() {
        return;
    }

    let mut total_volume = BigDecimal::from_str("0").unwrap();
    let mut total_fees = BigDecimal::from_str("0").unwrap();
    let mut buys = 0u64;
    let mut sells = 0u64;

    for event in &events.events {
        if let Ok(vol) = BigDecimal::from_str(&event.taker_amount_filled) {
            total_volume = total_volume + vol;
        }
        if let Ok(fee) = BigDecimal::from_str(&event.fee) {
            total_fees = total_fees + fee;
        }
        if event.side == "buy" { buys += 1; } else { sells += 1; }
    }

    let trades_count = events.events.len() as u64;
    let avg_size = if trades_count > 0 {
        (&total_volume / BigDecimal::from(trades_count)).to_string()
    } else {
        "0".to_string()
    };

    let stats = GlobalOrderbookStats {
        id: "global".to_string(),
        trades_quantity: trades_count,
        buys_quantity: buys,
        sells_quantity: sells,
        collateral_volume: total_volume.to_string(),
        scaled_collateral_volume: scale_bigdecimal(&total_volume),
        total_fees: total_fees.to_string(),
        average_trade_size: avg_size,
        unique_traders: 0,
        active_markets: 0,
        last_updated: events.timestamp.clone(),
        total_liquidity: "0".to_string(),
        market_cap: "0".to_string(),
        volume_24h: "0".to_string(),
        volume_7d: "0".to_string(),
        new_traders_24h: 0,
        new_markets_24h: 0,
        average_spread: "0".to_string(),
        platform_fee_revenue: total_fees.to_string(),
        maker_taker_ratio: "0".to_string(),
    };

    store.set(0, "global", &stats);
}

// ============================================
// Analytics Outputs (Layer 3)
// ============================================

/// Emit market orderbook updates when markets change
#[substreams::handlers::map]
pub fn map_market_orderbooks(
    events: OrderFilledEvents,
    store_deltas: Deltas<DeltaProto<MarketOrderbook>>,
) -> Result<MarketOrderbooks, substreams::errors::Error> {
    let orderbooks: Vec<MarketOrderbook> = store_deltas
        .deltas
        .into_iter()
        .map(|delta| delta.new_value)
        .collect();

    Ok(MarketOrderbooks {
        orderbooks,
        block_number: events.block_number,
        block_hash: events.block_hash,
        timestamp: events.timestamp,
    })
}

/// Emit trader account updates when accounts change
#[substreams::handlers::map]
pub fn map_trader_accounts(
    events: OrderFilledEvents,
    store_deltas: Deltas<DeltaProto<Account>>,
) -> Result<Accounts, substreams::errors::Error> {
    let accounts: Vec<Account> = store_deltas
        .deltas
        .into_iter()
        .map(|delta| delta.new_value)
        .collect();

    Ok(Accounts {
        accounts,
        block_number: events.block_number,
        block_hash: events.block_hash,
        timestamp: events.timestamp,
    })
}

/// Emit global statistics updates
#[substreams::handlers::map]
pub fn map_global_orderbook_stats(
    events: OrderFilledEvents,
    store_deltas: Deltas<DeltaProto<GlobalOrderbookStats>>,
) -> Result<GlobalOrderbookStats, substreams::errors::Error> {
    // Return the latest global stats from deltas
    let stats = store_deltas
        .deltas
        .into_iter()
        .map(|delta| delta.new_value)
        .last()
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
            last_updated: events.timestamp.clone(),
            total_liquidity: "0".to_string(),
            market_cap: "0".to_string(),
            volume_24h: "0".to_string(),
            volume_7d: "0".to_string(),
            new_traders_24h: 0,
            new_markets_24h: 0,
            average_spread: "0".to_string(),
            platform_fee_revenue: "0".to_string(),
            maker_taker_ratio: "0".to_string(),
        });

    Ok(stats)
}

// ============================================
// Helper Functions
// ============================================

fn scale_amount(amount: &str) -> String {
    // Scale from wei (18 decimals) to human-readable
    if let Ok(val) = BigDecimal::from_str(amount) {
        let scaled = val / BigDecimal::from_str("1000000000000000000").unwrap();
        scaled.to_string()
    } else {
        "0".to_string()
    }
}

fn scale_bigdecimal(val: &BigDecimal) -> String {
    let scaled = val / BigDecimal::from_str("1000000000000000000").unwrap();
    scaled.to_string()
}
