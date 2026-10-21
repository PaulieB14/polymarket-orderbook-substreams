//! polymarket-orderbook-substreams v1.0.0
//!
//! `map_events` makes one pass over each block (lazy-decoded, behind an `ethcommon:index_events`
//! block filter) and extracts OrderFilled, OrdersMatched and FeeCharged from all five Polymarket
//! exchanges plus market resolutions. Stores aggregate per market (token), per trader (order
//! owner) and globally. `db_out` upserts absolute values, so it works on Postgres and ClickHouse.

mod decode;
mod utils;
#[cfg(test)]
mod tests;

#[path = "pb/mod.rs"]
#[allow(dead_code, unused_imports)]
pub mod pb;

use std::collections::BTreeMap;
use std::str::FromStr;

use buffa_types::google::protobuf::Timestamp;
use decode as d;
use pb::polymarket::orderbook::v1::{
    Account, Accounts, FeeChargedEvent, GlobalOrderbookStats, MarketOrderbook, MarketOrderbooks,
    OrderFilledEvent, OrderbookEvents, OrdersMatchedEvent, ResolutionEvent,
};
use substreams::scalar::BigInt;
use substreams::store::{
    StoreAdd, StoreAddBigInt, StoreGet, StoreGetBigInt, StoreGetString, StoreMax, StoreMaxInt64,
    StoreMin, StoreMinInt64, StoreNew, StoreSet, StoreSetString, StoreGetInt64,
};
use substreams::Hex;
use substreams_database_change::pb::sf::substreams::sink::database::v1::DatabaseChanges;
use substreams_database_change::tables::Tables;
use substreams_ethereum::pb::eth::v2 as eth;

substreams_ethereum::init!();

fn ts<T: From<Timestamp>>(seconds: i64) -> T {
    Timestamp { seconds, nanos: 0, ..Default::default() }.into()
}

fn bi(s: &str) -> BigInt {
    BigInt::from_str(s).unwrap_or_default()
}

// ============================================
// Layer 1: single-pass extraction
// ============================================

#[substreams::handlers::map]
pub fn map_events(blk: &eth::BlockLazyView<'_>) -> Result<OrderbookEvents, substreams::errors::Error> {
    let seconds = blk
        .header
        .get()?
        .map(|h| h.timestamp.seconds)
        .unwrap_or_default();
    let number = blk.number;
    let mut out = OrderbookEvents {
        block_number: number,
        block_hash: Hex::encode(blk.hash),
        timestamp: ts(seconds),
        ..Default::default()
    };

    for trx in blk.transactions() {
        let Some(receipt) = trx.receipt() else { continue };
        let tx_hash = Hex::encode(trx.hash);
        for log in receipt.logs.iter() {
            let log = log?;
            let topics: Vec<&[u8]> = log.topics.iter().copied().collect();
            let Some(topic0) = topics.first() else { continue };
            let li = log.index;
            let id = format!("{}-{}", tx_hash, li);

            if let Some((exchange, v1)) = d::exchange_of(log.address) {
                if let Some(f) = d::decode_fill(&topics, log.data) {
                    let (shares, collateral) = f.shares_and_collateral();
                    out.fills.push(OrderFilledEvent {
                        id,
                        transaction_hash: tx_hash.clone(),
                        timestamp: ts(seconds),
                        order_hash: f.order_hash,
                        maker: f.maker,
                        taker: f.taker,
                        maker_asset_id: f.maker_asset_id.to_string(),
                        taker_asset_id: f.taker_asset_id.to_string(),
                        maker_amount_filled: f.maker_amount.to_string(),
                        taker_amount_filled: f.taker_amount.to_string(),
                        fee: f.fee.to_string(),
                        block_number: number,
                        side: if f.buy { "buy" } else { "sell" }.to_string(),
                        price: d::price(&collateral, &shares),
                        ordinal: log.ordinal,
                        exchange_version: if v1 { "v1" } else if exchange == "exchange_v3" { "v3" } else { "v2" }.to_string(),
                        token_id: f.token_id.to_string(),
                        side_raw: f.side_raw,
                        builder: f.builder,
                        metadata: f.metadata,
                        exchange: exchange.to_string(),
                        log_index: li,
                        shares: shares.to_string(),
                        collateral: collateral.to_string(),
                        ..Default::default()
                    });
                } else if *topic0 == d::ORDERS_MATCHED_V1 || *topic0 == d::ORDERS_MATCHED_V2 {
                    if topics.len() < 3 { continue; }
                    let v2 = *topic0 == d::ORDERS_MATCHED_V2;
                    let (a0, a1) = (d::uint(log.data, 0), d::uint(log.data, 1));
                    let (Some(a0), Some(a1)) = (a0, a1) else { continue };
                    let (side_raw, token, ma, ta) = if v2 {
                        let side = a0.to_u64() as u32;
                        let (ma, ta) = if side == 0 { (BigInt::zero(), a1.clone()) } else { (a1.clone(), BigInt::zero()) };
                        (side, a1, ma, ta)
                    } else {
                        let buy = a0.is_zero();
                        (if buy { 0 } else { 1 }, if buy { a1.clone() } else { a0.clone() }, a0, a1)
                    };
                    out.matches.push(OrdersMatchedEvent {
                        id,
                        timestamp: ts(seconds),
                        maker_asset_id: ma.to_string(),
                        taker_asset_id: ta.to_string(),
                        maker_amount_filled: d::uint(log.data, 2).unwrap_or_default().to_string(),
                        taker_amount_filled: d::uint(log.data, 3).unwrap_or_default().to_string(),
                        block_number: number,
                        ordinal: log.ordinal,
                        exchange_version: if !v2 { "v1" } else if exchange == "exchange_v3" { "v3" } else { "v2" }.to_string(),
                        taker_order_hash: Hex::encode(topics[1]),
                        taker_order_maker: d::addr_of(topics[2]),
                        token_id: token.to_string(),
                        side_raw,
                        exchange: exchange.to_string(),
                        transaction_hash: tx_hash.clone(),
                        log_index: li,
                        ..Default::default()
                    });
                } else if (*topic0 == d::FEE_CHARGED_V1 || *topic0 == d::FEE_CHARGED_V2) && topics.len() >= 2 {
                    let v1fee = *topic0 == d::FEE_CHARGED_V1;
                    let amount = d::uint(log.data, if v1fee { 1 } else { 0 }).unwrap_or_default();
                    out.fees.push(FeeChargedEvent {
                        id,
                        transaction_hash: tx_hash.clone(),
                        log_index: li,
                        exchange: exchange.to_string(),
                        receiver: d::addr_of(topics[1]),
                        token_id: if v1fee { d::uint(log.data, 0).unwrap_or_default().to_string() } else { String::new() },
                        amount: amount.to_string(),
                        block_number: number,
                        timestamp: ts(seconds),
                        ..Default::default()
                    });
                }
                continue;
            }

            let source = if log.address == d::CONDITIONAL_TOKENS && *topic0 == d::CONDITION_RESOLUTION {
                Some(("ctf", 1usize))
            } else if *topic0 == d::CONDITION_RESOLVED && log.address == d::BINARY_MODULE {
                Some(("binary_module", 0))
            } else if *topic0 == d::CONDITION_RESOLVED && log.address == d::NEG_RISK_MODULE {
                Some(("neg_risk_module", 0))
            } else {
                None
            };
            if let (Some((src, arr)), Some(cond)) = (source, topics.get(1)) {
                out.resolutions.push(ResolutionEvent {
                    id,
                    transaction_hash: tx_hash.clone(),
                    log_index: li,
                    source: src.to_string(),
                    condition_id: Hex::encode(cond),
                    payouts: d::uint_array(log.data, arr).unwrap_or_default().iter().map(|p| p.to_string()).collect(),
                    block_number: number,
                    timestamp: ts(seconds),
                    ..Default::default()
                });
            }
        }
    }
    out.fills.sort_by_key(|e| e.ordinal);
    out.matches.sort_by_key(|e| e.ordinal);
    Ok(out)
}

/// Backwards-compatible view of the old `map_all_order_fills` output.
#[substreams::handlers::map]
pub fn map_all_order_fills(ev: OrderbookEvents) -> Result<pb::polymarket::orderbook::v1::OrderFilledEvents, substreams::errors::Error> {
    Ok(pb::polymarket::orderbook::v1::OrderFilledEvents {
        events: ev.fills,
        block_number: ev.block_number,
        block_hash: ev.block_hash,
        timestamp: ev.timestamp,
        ..Default::default()
    })
}

// ============================================
// Layer 2: stores
// ============================================

/// The taker order's own fill (taker == exchange) represents the match once.
fn is_match_fill(f: &OrderFilledEvent) -> bool {
    d::is_exchange_hex(&f.taker)
}

/// m:{token}:{trades|buys|sells|volume|fees}, t:{owner}:{trades|buys|sells|volume|fees},
/// g:{trades|buys|sells|volume|fees|fee_revenue}
#[substreams::handlers::store]
pub fn store_totals(ev: OrderbookEvents, store: StoreAddBigInt) {
    let one = BigInt::one;
    for f in &ev.fills {
        let side = if f.side == "buy" { "buys" } else { "sells" };
        let coll = bi(&f.collateral);
        let fee = bi(&f.fee);
        // trader stats: every fill belongs to its order owner (`maker`)
        let t = format!("t:{}", f.maker);
        store.add(f.ordinal, format!("{}:trades", t), one());
        store.add(f.ordinal, format!("{}:{}", t, side), one());
        store.add(f.ordinal, format!("{}:volume", t), coll.clone());
        store.add(f.ordinal, format!("{}:fees", t), fee.clone());
        // market / global stats: once per match
        if is_match_fill(f) {
            let m = format!("m:{}", f.token_id);
            for k in [format!("{}:trades", m), format!("{}:{}", m, side), "g:trades".into(), format!("g:{}", side)] {
                store.add(f.ordinal, k, one());
            }
            store.add(f.ordinal, format!("{}:volume", m), coll.clone());
            store.add(f.ordinal, "g:volume", coll);
        }
        store.add(f.ordinal, format!("m:{}:fees", f.token_id), fee.clone());
        store.add(f.ordinal, "g:fees", fee);
    }
    for x in &ev.fees {
        store.add(0, "g:fee_revenue", bi(&x.amount));
    }
}

/// {token} -> price of the latest match
#[substreams::handlers::store]
pub fn store_last_price(ev: OrderbookEvents, store: StoreSetString) {
    for f in ev.fills.iter().filter(|f| is_match_fill(f)) {
        store.set(f.ordinal, &f.token_id, &f.price);
    }
}

#[substreams::handlers::store]
pub fn store_first_trade(ev: OrderbookEvents, store: StoreMinInt64) {
    for f in &ev.fills {
        store.min(f.ordinal, &f.maker, f.timestamp.seconds);
    }
}

#[substreams::handlers::store]
pub fn store_last_trade(ev: OrderbookEvents, store: StoreMaxInt64) {
    for f in &ev.fills {
        store.max(f.ordinal, &f.maker, f.timestamp.seconds);
    }
}

// ============================================
// Layer 3: analytics
// ============================================

fn avg(total: &BigInt, n: u64) -> String {
    if n == 0 { "0".into() } else { (total.clone() / BigInt::from(n)).to_string() }
}

#[substreams::handlers::map]
pub fn map_market_orderbooks(
    ev: OrderbookEvents,
    totals: StoreGetBigInt,
    prices: StoreGetString,
) -> Result<MarketOrderbooks, substreams::errors::Error> {
    let mut tokens: BTreeMap<String, ()> = BTreeMap::new();
    for f in &ev.fills {
        tokens.insert(f.token_id.clone(), ());
    }
    let day = (ev.timestamp.seconds / 86400) as u64;
    let orderbooks = tokens
        .into_keys()
        .map(|t| {
            let g = |k: &str| totals.get_last(format!("m:{}:{}", t, k)).unwrap_or_default();
            let trades = g("trades").to_u64();
            let vol = g("volume");
            MarketOrderbook {
                id: t.clone(),
                condition_id: String::new(),
                trades_quantity: trades,
                buys_quantity: g("buys").to_u64(),
                sells_quantity: g("sells").to_u64(),
                collateral_volume: vol.to_string(),
                scaled_collateral_volume: utils::scale6(&vol),
                average_trade_size: avg(&vol, trades),
                total_fees: g("fees").to_string(),
                last_active_day: day,
                mid_price: prices.get_last(&t).unwrap_or_else(|| "0".into()),
                last_updated_block: ev.block_number,
                ..Default::default()
            }
        })
        .collect();
    Ok(MarketOrderbooks { orderbooks, block_number: ev.block_number, block_hash: ev.block_hash, timestamp: ev.timestamp, ..Default::default() })
}

#[substreams::handlers::map]
pub fn map_trader_accounts(
    ev: OrderbookEvents,
    totals: StoreGetBigInt,
    first: StoreGetInt64,
    last: StoreGetInt64,
) -> Result<Accounts, substreams::errors::Error> {
    let mut owners: BTreeMap<String, ()> = BTreeMap::new();
    for f in &ev.fills {
        owners.insert(f.maker.clone(), ());
    }
    let accounts = owners
        .into_keys()
        .map(|a| {
            let g = |k: &str| totals.get_last(format!("t:{}:{}", a, k)).unwrap_or_default();
            Account {
                id: a.clone(),
                trades_quantity: g("trades").to_u64(),
                total_volume: g("volume").to_string(),
                total_fees: g("fees").to_string(),
                first_trade: ts(first.get_last(&a).unwrap_or_default()),
                last_trade: ts(last.get_last(&a).unwrap_or_default()),
                is_active: true,
                ..Default::default()
            }
        })
        .collect();
    Ok(Accounts { accounts, block_number: ev.block_number, block_hash: ev.block_hash, timestamp: ev.timestamp, ..Default::default() })
}

#[substreams::handlers::map]
pub fn map_global_orderbook_stats(ev: OrderbookEvents, totals: StoreGetBigInt) -> Result<GlobalOrderbookStats, substreams::errors::Error> {
    let g = |k: &str| totals.get_last(format!("g:{}", k)).unwrap_or_default();
    let trades = g("trades").to_u64();
    let vol = g("volume");
    Ok(GlobalOrderbookStats {
        id: "global".into(),
        trades_quantity: trades,
        buys_quantity: g("buys").to_u64(),
        sells_quantity: g("sells").to_u64(),
        collateral_volume: vol.to_string(),
        scaled_collateral_volume: utils::scale6(&vol),
        total_fees: g("fees").to_string(),
        average_trade_size: avg(&vol, trades),
        last_updated: ev.timestamp,
        platform_fee_revenue: g("fee_revenue").to_string(),
        ..Default::default()
    })
}

// ============================================
// Layer 4: SQL
// ============================================

#[substreams::handlers::map]
pub fn db_out(
    ev: OrderbookEvents,
    market_orderbooks: MarketOrderbooks,
    trader_accounts: Accounts,
    global_stats: GlobalOrderbookStats,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    let bts = utils::unix_to_timestamp(ev.timestamp.seconds);

    for e in &ev.fills {
        tables
            .create_row("order_fills", &e.id)
            .set("block_number", e.block_number)
            .set("block_timestamp", &bts)
            .set("transaction_hash", &e.transaction_hash)
            .set("log_index", e.log_index)
            .set("exchange", &e.exchange)
            .set("exchange_version", &e.exchange_version)
            .set("order_hash", &e.order_hash)
            .set("maker", &e.maker)
            .set("taker", &e.taker)
            .set("maker_asset_id", &e.maker_asset_id)
            .set("taker_asset_id", &e.taker_asset_id)
            .set("maker_amount_filled", &e.maker_amount_filled)
            .set("taker_amount_filled", &e.taker_amount_filled)
            .set("fee", &e.fee)
            .set("shares", &e.shares)
            .set("collateral", &e.collateral)
            .set("side", &e.side)
            .set("side_raw", e.side_raw)
            .set("price", &e.price)
            .set("token_id", &e.token_id)
            .set("builder", &e.builder)
            .set("metadata", &e.metadata);
    }
    for m in &ev.matches {
        tables
            .create_row("orders_matched", &m.id)
            .set("block_number", m.block_number)
            .set("block_timestamp", &bts)
            .set("transaction_hash", &m.transaction_hash)
            .set("exchange", &m.exchange)
            .set("taker_order_hash", &m.taker_order_hash)
            .set("taker_order_maker", &m.taker_order_maker)
            .set("token_id", &m.token_id)
            .set("side_raw", m.side_raw)
            .set("maker_amount_filled", &m.maker_amount_filled)
            .set("taker_amount_filled", &m.taker_amount_filled);
    }
    for x in &ev.fees {
        tables
            .create_row("fees", &x.id)
            .set("block_number", x.block_number)
            .set("block_timestamp", &bts)
            .set("transaction_hash", &x.transaction_hash)
            .set("exchange", &x.exchange)
            .set("receiver", &x.receiver)
            .set("token_id", &x.token_id)
            .set("amount", &x.amount);
    }
    for r in &ev.resolutions {
        tables
            .create_row("resolutions", &r.id)
            .set("block_number", r.block_number)
            .set("block_timestamp", &bts)
            .set("transaction_hash", &r.transaction_hash)
            .set("source", &r.source)
            .set("condition_id", &r.condition_id)
            .set("payouts", r.payouts.join(","));
    }
    for o in &market_orderbooks.orderbooks {
        tables
            .upsert_row("market_orderbooks", &o.id)
            .set("trades_quantity", o.trades_quantity)
            .set("buys_quantity", o.buys_quantity)
            .set("sells_quantity", o.sells_quantity)
            .set("collateral_volume", &o.collateral_volume)
            .set("average_trade_size", &o.average_trade_size)
            .set("total_fees", &o.total_fees)
            .set("last_price", &o.mid_price)
            .set("last_updated_block", o.last_updated_block)
            .set("updated_at", &bts);
    }
    for a in &trader_accounts.accounts {
        tables
            .upsert_row("trader_accounts", &a.id)
            .set("trades_quantity", a.trades_quantity)
            .set("total_volume", &a.total_volume)
            .set("total_fees", &a.total_fees)
            .set("first_trade_at", utils::unix_to_timestamp(a.first_trade.seconds))
            .set("last_trade_at", utils::unix_to_timestamp(a.last_trade.seconds))
            .set("updated_at", &bts);
    }
    if !ev.fills.is_empty() || !ev.fees.is_empty() {
        tables
            .upsert_row("global_stats", "global")
            .set("trades_quantity", global_stats.trades_quantity)
            .set("buys_quantity", global_stats.buys_quantity)
            .set("sells_quantity", global_stats.sells_quantity)
            .set("collateral_volume", &global_stats.collateral_volume)
            .set("total_fees", &global_stats.total_fees)
            .set("average_trade_size", &global_stats.average_trade_size)
            .set("platform_fee_revenue", &global_stats.platform_fee_revenue)
            .set("last_updated_block", ev.block_number)
            .set("updated_at", &bts);
    }
    Ok(tables.to_database_changes())
}
