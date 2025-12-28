use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use prost_types::Timestamp;

mod abi;
mod utils;

#[path = "pb/mod.rs"]
#[allow(dead_code)]
pub mod pb;

use pb::polymarket::orderbook::v1::{OrderFilledEvent, OrderFilledEvents, OrdersMatchedEvent, OrdersMatchedEvents};

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
