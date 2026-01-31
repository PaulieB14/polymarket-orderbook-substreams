-- Polymarket Orderbook Substreams - PostgreSQL Schema
-- Version: 0.2.0

-- Order Fills Table
-- Stores individual order fill events from both CTF and Neg Risk exchanges
CREATE TABLE IF NOT EXISTS order_fills (
    id VARCHAR PRIMARY KEY,
    transaction_hash VARCHAR(66) NOT NULL,
    order_hash VARCHAR(66) NOT NULL,
    maker VARCHAR(42) NOT NULL,
    taker VARCHAR(42) NOT NULL,
    maker_asset_id VARCHAR NOT NULL,
    taker_asset_id VARCHAR NOT NULL,
    maker_amount_filled NUMERIC(78, 0) NOT NULL,
    taker_amount_filled NUMERIC(78, 0) NOT NULL,
    fee NUMERIC(78, 0) NOT NULL,
    side VARCHAR(10) NOT NULL,
    price NUMERIC(38, 18) NOT NULL,
    block_number BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for order_fills
CREATE INDEX IF NOT EXISTS idx_order_fills_block ON order_fills(block_number);
CREATE INDEX IF NOT EXISTS idx_order_fills_maker ON order_fills(maker);
CREATE INDEX IF NOT EXISTS idx_order_fills_taker ON order_fills(taker);
CREATE INDEX IF NOT EXISTS idx_order_fills_maker_asset ON order_fills(maker_asset_id);
CREATE INDEX IF NOT EXISTS idx_order_fills_tx_hash ON order_fills(transaction_hash);

-- Market Orderbooks Table
-- Aggregated market-level statistics
CREATE TABLE IF NOT EXISTS market_orderbooks (
    id VARCHAR PRIMARY KEY,
    condition_id VARCHAR NOT NULL,
    trades_quantity BIGINT NOT NULL DEFAULT 0,
    buys_quantity BIGINT NOT NULL DEFAULT 0,
    sells_quantity BIGINT NOT NULL DEFAULT 0,
    collateral_volume NUMERIC(78, 0) NOT NULL DEFAULT 0,
    average_trade_size NUMERIC(78, 18) NOT NULL DEFAULT 0,
    total_fees NUMERIC(78, 0) NOT NULL DEFAULT 0,
    mid_price NUMERIC(38, 18) NOT NULL DEFAULT 0,
    last_updated_block BIGINT NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for market_orderbooks
CREATE INDEX IF NOT EXISTS idx_market_orderbooks_condition ON market_orderbooks(condition_id);
CREATE INDEX IF NOT EXISTS idx_market_orderbooks_volume ON market_orderbooks(collateral_volume DESC);
CREATE INDEX IF NOT EXISTS idx_market_orderbooks_trades ON market_orderbooks(trades_quantity DESC);

-- Trader Accounts Table
-- Aggregated trader-level analytics
CREATE TABLE IF NOT EXISTS trader_accounts (
    id VARCHAR PRIMARY KEY,
    trades_quantity BIGINT NOT NULL DEFAULT 0,
    total_volume NUMERIC(78, 0) NOT NULL DEFAULT 0,
    total_fees NUMERIC(78, 0) NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    trader_type VARCHAR(20) NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for trader_accounts
CREATE INDEX IF NOT EXISTS idx_trader_accounts_volume ON trader_accounts(total_volume DESC);
CREATE INDEX IF NOT EXISTS idx_trader_accounts_trades ON trader_accounts(trades_quantity DESC);
CREATE INDEX IF NOT EXISTS idx_trader_accounts_active ON trader_accounts(is_active);

-- Global Stats Table
-- Platform-wide aggregated statistics
CREATE TABLE IF NOT EXISTS global_stats (
    id VARCHAR PRIMARY KEY DEFAULT 'global',
    trades_quantity BIGINT NOT NULL DEFAULT 0,
    buys_quantity BIGINT NOT NULL DEFAULT 0,
    sells_quantity BIGINT NOT NULL DEFAULT 0,
    collateral_volume NUMERIC(78, 0) NOT NULL DEFAULT 0,
    total_fees NUMERIC(78, 0) NOT NULL DEFAULT 0,
    average_trade_size NUMERIC(78, 18) NOT NULL DEFAULT 0,
    platform_fee_revenue NUMERIC(78, 0) NOT NULL DEFAULT 0,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Cursors table for substreams-sink-sql
CREATE TABLE IF NOT EXISTS cursors (
    id VARCHAR PRIMARY KEY,
    cursor VARCHAR NOT NULL,
    block_num BIGINT NOT NULL,
    block_id VARCHAR NOT NULL
);

-- Views for common queries

-- Top markets by volume
CREATE OR REPLACE VIEW top_markets_by_volume AS
SELECT
    id,
    condition_id,
    trades_quantity,
    collateral_volume,
    average_trade_size,
    mid_price
FROM market_orderbooks
ORDER BY collateral_volume DESC
LIMIT 100;

-- Top traders by volume
CREATE OR REPLACE VIEW top_traders_by_volume AS
SELECT
    id,
    trades_quantity,
    total_volume,
    total_fees,
    trader_type
FROM trader_accounts
WHERE is_active = true
ORDER BY total_volume DESC
LIMIT 100;

-- Recent large trades
CREATE OR REPLACE VIEW recent_large_trades AS
SELECT
    id,
    transaction_hash,
    maker,
    taker,
    taker_amount_filled,
    price,
    side,
    block_number
FROM order_fills
WHERE taker_amount_filled > 1000000000000000000000  -- > 1000 USDC
ORDER BY block_number DESC
LIMIT 100;
