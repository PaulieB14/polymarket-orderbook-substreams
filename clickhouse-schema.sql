-- Polymarket Orderbook Substreams - Clickhouse Schema
-- Version: 0.2.0
-- Optimized for high-performance analytics queries

-- Order Fills Table
-- Uses MergeTree for fast time-series analytics
CREATE TABLE IF NOT EXISTS order_fills (
    id String,
    transaction_hash FixedString(66),
    block_number UInt64 Codec(Delta, ZSTD),
    block_timestamp DateTime Codec(DoubleDelta, ZSTD),
    order_hash FixedString(66),
    maker LowCardinality(String),
    taker LowCardinality(String),
    maker_asset_id String,
    taker_asset_id String,
    maker_amount_filled UInt256,
    taker_amount_filled UInt256,
    fee UInt256,
    side LowCardinality(String),
    price Decimal(38, 18),

    -- Materialized columns for analytics
    date Date MATERIALIZED toDate(block_timestamp),
    hour DateTime MATERIALIZED toStartOfHour(block_timestamp)
)
ENGINE = MergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (block_timestamp, maker_asset_id, id)
SETTINGS index_granularity = 8192;

-- Market Analytics Table
-- ReplacingMergeTree for handling updates
CREATE TABLE IF NOT EXISTS market_analytics (
    id String,
    condition_id String,
    trades_quantity UInt64,
    buys_quantity UInt64,
    sells_quantity UInt64,
    collateral_volume UInt256,
    average_trade_size Decimal(38, 18),
    total_fees UInt256,
    mid_price Decimal(38, 18),
    volume_24h Decimal(38, 18),
    volume_7d Decimal(38, 18),
    liquidity_score Decimal(38, 18),
    last_updated_block UInt64,
    updated_at DateTime DEFAULT now()
)
ENGINE = ReplacingMergeTree(updated_at)
ORDER BY (id)
SETTINGS index_granularity = 8192;

-- Trader Analytics Table
-- ReplacingMergeTree for efficient updates
CREATE TABLE IF NOT EXISTS trader_analytics (
    id String,
    trades_quantity UInt64,
    total_volume UInt256,
    total_fees UInt256,
    volume_24h Decimal(38, 18),
    volume_7d Decimal(38, 18),
    markets_traded UInt64,
    is_active UInt8,
    trader_type LowCardinality(String),
    updated_at DateTime DEFAULT now()
)
ENGINE = ReplacingMergeTree(updated_at)
ORDER BY (id)
SETTINGS index_granularity = 8192;

-- Global Analytics Table
CREATE TABLE IF NOT EXISTS global_analytics (
    id String DEFAULT 'global',
    trades_quantity UInt64,
    buys_quantity UInt64,
    sells_quantity UInt64,
    collateral_volume UInt256,
    total_fees UInt256,
    unique_traders UInt64,
    active_markets UInt64,
    volume_24h Decimal(38, 18),
    volume_7d Decimal(38, 18),
    platform_fee_revenue UInt256,
    updated_at DateTime DEFAULT now()
)
ENGINE = ReplacingMergeTree(updated_at)
ORDER BY (id)
SETTINGS index_granularity = 8192;

-- Hourly Volume Aggregation
-- SummingMergeTree for efficient rollups
CREATE TABLE IF NOT EXISTS hourly_volume (
    hour DateTime,
    market_id String,
    trades_count UInt64,
    buy_volume UInt256,
    sell_volume UInt256,
    total_volume UInt256,
    total_fees UInt256,
    unique_makers AggregateFunction(uniq, String),
    unique_takers AggregateFunction(uniq, String)
)
ENGINE = SummingMergeTree((trades_count, buy_volume, sell_volume, total_volume, total_fees))
PARTITION BY toYYYYMM(hour)
ORDER BY (hour, market_id)
SETTINGS index_granularity = 8192;

-- Materialized view for hourly aggregation
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_hourly_volume TO hourly_volume AS
SELECT
    toStartOfHour(block_timestamp) AS hour,
    maker_asset_id AS market_id,
    count() AS trades_count,
    sumIf(taker_amount_filled, side = 'buy') AS buy_volume,
    sumIf(taker_amount_filled, side = 'sell') AS sell_volume,
    sum(taker_amount_filled) AS total_volume,
    sum(fee) AS total_fees,
    uniqState(maker) AS unique_makers,
    uniqState(taker) AS unique_takers
FROM order_fills
GROUP BY hour, market_id;

-- Daily Volume Aggregation
CREATE TABLE IF NOT EXISTS daily_volume (
    date Date,
    market_id String,
    trades_count UInt64,
    buy_volume UInt256,
    sell_volume UInt256,
    total_volume UInt256,
    total_fees UInt256,
    avg_price Decimal(38, 18),
    high_price Decimal(38, 18),
    low_price Decimal(38, 18)
)
ENGINE = SummingMergeTree((trades_count, buy_volume, sell_volume, total_volume, total_fees))
PARTITION BY toYYYYMM(date)
ORDER BY (date, market_id)
SETTINGS index_granularity = 8192;

-- Materialized view for daily aggregation
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_daily_volume TO daily_volume AS
SELECT
    toDate(block_timestamp) AS date,
    maker_asset_id AS market_id,
    count() AS trades_count,
    sumIf(taker_amount_filled, side = 'buy') AS buy_volume,
    sumIf(taker_amount_filled, side = 'sell') AS sell_volume,
    sum(taker_amount_filled) AS total_volume,
    sum(fee) AS total_fees,
    avg(price) AS avg_price,
    max(price) AS high_price,
    min(price) AS low_price
FROM order_fills
GROUP BY date, market_id;

-- Cursors table for substreams-sink-sql
CREATE TABLE IF NOT EXISTS cursors (
    id String,
    cursor String,
    block_num UInt64,
    block_id String
)
ENGINE = ReplacingMergeTree()
ORDER BY (id);

-- Example Queries:

-- Top markets by 24h volume
-- SELECT
--     market_id,
--     sum(total_volume) as volume_24h,
--     sum(trades_count) as trades_24h
-- FROM hourly_volume
-- WHERE hour >= now() - INTERVAL 24 HOUR
-- GROUP BY market_id
-- ORDER BY volume_24h DESC
-- LIMIT 10;

-- Trader leaderboard
-- SELECT
--     id,
--     total_volume,
--     trades_quantity,
--     total_fees
-- FROM trader_analytics FINAL
-- WHERE is_active = 1
-- ORDER BY total_volume DESC
-- LIMIT 100;

-- Price history for a market
-- SELECT
--     date,
--     avg_price,
--     high_price,
--     low_price,
--     total_volume
-- FROM daily_volume
-- WHERE market_id = 'market:12345'
-- ORDER BY date DESC
-- LIMIT 30;
