-- Polymarket Orderbook Substreams v1.0.0 - ClickHouse schema
-- substreams-sink-sql with a clickhouse:// DSN. Event tables are append-only MergeTree,
-- partitioned by month and ordered for time-range + token/owner scans. Aggregate tables
-- receive absolute values on every change, so ReplacingMergeTree(version) keeps the latest row
-- (query with FINAL or argMax).

CREATE TABLE IF NOT EXISTS order_fills (
    id String, block_number UInt64 CODEC(Delta, ZSTD), block_timestamp DateTime CODEC(DoubleDelta, ZSTD),
    transaction_hash String, log_index UInt32, exchange LowCardinality(String), exchange_version LowCardinality(String),
    order_hash String, maker String, taker String, maker_asset_id String, taker_asset_id String,
    maker_amount_filled UInt256, taker_amount_filled UInt256, fee UInt256, shares UInt256, collateral UInt256,
    side LowCardinality(String), side_raw UInt8, price Decimal(20, 6), token_id String, builder String, metadata String
) ENGINE = ReplacingMergeTree
PARTITION BY toYYYYMM(block_timestamp)
ORDER BY (token_id, block_timestamp, id);

CREATE TABLE IF NOT EXISTS orders_matched (
    id String, block_number UInt64, block_timestamp DateTime, transaction_hash String, exchange LowCardinality(String),
    taker_order_hash String, taker_order_maker String, token_id String, side_raw UInt8,
    maker_amount_filled UInt256, taker_amount_filled UInt256
) ENGINE = ReplacingMergeTree
PARTITION BY toYYYYMM(block_timestamp)
ORDER BY (token_id, block_timestamp, id);

CREATE TABLE IF NOT EXISTS fees (
    id String, block_number UInt64, block_timestamp DateTime, transaction_hash String, exchange LowCardinality(String),
    receiver String, token_id String, amount UInt256
) ENGINE = ReplacingMergeTree
PARTITION BY toYYYYMM(block_timestamp)
ORDER BY (receiver, block_timestamp, id);

CREATE TABLE IF NOT EXISTS resolutions (
    id String, block_number UInt64, block_timestamp DateTime, transaction_hash String, source LowCardinality(String),
    condition_id String, payouts String
) ENGINE = ReplacingMergeTree
ORDER BY (condition_id, id);

CREATE TABLE IF NOT EXISTS market_orderbooks (
    id String, trades_quantity UInt64, buys_quantity UInt64, sells_quantity UInt64,
    collateral_volume UInt256, average_trade_size UInt256, total_fees UInt256, last_price Decimal(20, 6),
    last_updated_block UInt64, updated_at DateTime
) ENGINE = ReplacingMergeTree(last_updated_block)
ORDER BY id;

CREATE TABLE IF NOT EXISTS trader_accounts (
    id String, trades_quantity UInt64, total_volume UInt256, total_fees UInt256,
    first_trade_at DateTime, last_trade_at DateTime, updated_at DateTime
) ENGINE = ReplacingMergeTree(updated_at)
ORDER BY id;

CREATE TABLE IF NOT EXISTS global_stats (
    id String, trades_quantity UInt64, buys_quantity UInt64, sells_quantity UInt64,
    collateral_volume UInt256, total_fees UInt256, average_trade_size UInt256, platform_fee_revenue UInt256,
    last_updated_block UInt64, updated_at DateTime
) ENGINE = ReplacingMergeTree(last_updated_block)
ORDER BY id;
