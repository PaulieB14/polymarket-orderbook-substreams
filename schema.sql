-- Polymarket Orderbook Substreams v1.0.0 - PostgreSQL schema
-- Amounts are raw base units (6 decimals). Addresses are lowercase hex without 0x (as in v0.x).

CREATE TABLE IF NOT EXISTS order_fills (
    id                  VARCHAR(80) PRIMARY KEY,    -- tx_hash-log_index (v0.x: tx_hash-order_hash)
    block_number        BIGINT NOT NULL,
    block_timestamp     TIMESTAMP NOT NULL,
    transaction_hash    VARCHAR(64) NOT NULL,
    log_index           INTEGER NOT NULL,
    exchange            VARCHAR(16) NOT NULL,       -- ctf_v1 | neg_risk_v1 | ctf_v2 | neg_risk_v2 | exchange_v3
    exchange_version    VARCHAR(4) NOT NULL,        -- v1 | v2 | v3
    order_hash          VARCHAR(64) NOT NULL,
    maker               VARCHAR(40) NOT NULL,       -- order owner
    taker               VARCHAR(40) NOT NULL,       -- counterparty, or the exchange on the taker order's own fill
    maker_asset_id      VARCHAR(78) NOT NULL,
    taker_asset_id      VARCHAR(78) NOT NULL,
    maker_amount_filled NUMERIC(78, 0) NOT NULL,
    taker_amount_filled NUMERIC(78, 0) NOT NULL,
    fee                 NUMERIC(78, 0) NOT NULL,
    shares              NUMERIC(78, 0) NOT NULL,
    collateral          NUMERIC(78, 0) NOT NULL,
    side                VARCHAR(4) NOT NULL,        -- buy | sell (owner's side)
    side_raw            SMALLINT NOT NULL,
    price               NUMERIC(20, 6) NOT NULL,    -- collateral / shares
    token_id            VARCHAR(78) NOT NULL,
    builder             VARCHAR(64),
    metadata            VARCHAR(64)
);
CREATE INDEX IF NOT EXISTS idx_order_fills_block ON order_fills(block_number);
CREATE INDEX IF NOT EXISTS idx_order_fills_ts ON order_fills(block_timestamp);
CREATE INDEX IF NOT EXISTS idx_order_fills_maker ON order_fills(maker);
CREATE INDEX IF NOT EXISTS idx_order_fills_token ON order_fills(token_id);
CREATE INDEX IF NOT EXISTS idx_order_fills_tx ON order_fills(transaction_hash);
CREATE INDEX IF NOT EXISTS idx_order_fills_builder ON order_fills(builder);

CREATE TABLE IF NOT EXISTS orders_matched (
    id VARCHAR(80) PRIMARY KEY, block_number BIGINT NOT NULL, block_timestamp TIMESTAMP NOT NULL,
    transaction_hash VARCHAR(64) NOT NULL, exchange VARCHAR(16) NOT NULL,
    taker_order_hash VARCHAR(64) NOT NULL, taker_order_maker VARCHAR(40) NOT NULL,
    token_id VARCHAR(78) NOT NULL, side_raw SMALLINT NOT NULL,
    maker_amount_filled NUMERIC(78, 0) NOT NULL, taker_amount_filled NUMERIC(78, 0) NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_orders_matched_token ON orders_matched(token_id);

CREATE TABLE IF NOT EXISTS fees (
    id VARCHAR(80) PRIMARY KEY, block_number BIGINT NOT NULL, block_timestamp TIMESTAMP NOT NULL,
    transaction_hash VARCHAR(64) NOT NULL, exchange VARCHAR(16) NOT NULL,
    receiver VARCHAR(40) NOT NULL, token_id VARCHAR(78), amount NUMERIC(78, 0) NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_fees_receiver ON fees(receiver);

CREATE TABLE IF NOT EXISTS resolutions (
    id VARCHAR(80) PRIMARY KEY, block_number BIGINT NOT NULL, block_timestamp TIMESTAMP NOT NULL,
    transaction_hash VARCHAR(64) NOT NULL, source VARCHAR(16) NOT NULL, -- ctf | binary_module | neg_risk_module
    condition_id VARCHAR(64) NOT NULL, payouts TEXT
);
CREATE INDEX IF NOT EXISTS idx_resolutions_condition ON resolutions(condition_id);

-- Per token (outcome). id = token_id (v0.x used "condition_<maker_asset_id>").
CREATE TABLE IF NOT EXISTS market_orderbooks (
    id                 VARCHAR(78) PRIMARY KEY,
    trades_quantity    BIGINT NOT NULL DEFAULT 0,   -- matches
    buys_quantity      BIGINT NOT NULL DEFAULT 0,   -- by taker side
    sells_quantity     BIGINT NOT NULL DEFAULT 0,
    collateral_volume  NUMERIC(78, 0) NOT NULL DEFAULT 0,
    average_trade_size NUMERIC(78, 0) NOT NULL DEFAULT 0,
    total_fees         NUMERIC(78, 0) NOT NULL DEFAULT 0,
    last_price         NUMERIC(20, 6) NOT NULL DEFAULT 0,
    last_updated_block BIGINT NOT NULL,
    updated_at         TIMESTAMP NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_market_orderbooks_volume ON market_orderbooks(collateral_volume DESC);

-- Per order owner.
CREATE TABLE IF NOT EXISTS trader_accounts (
    id              VARCHAR(40) PRIMARY KEY,
    trades_quantity BIGINT NOT NULL DEFAULT 0,
    total_volume    NUMERIC(78, 0) NOT NULL DEFAULT 0,
    total_fees      NUMERIC(78, 0) NOT NULL DEFAULT 0,
    first_trade_at  TIMESTAMP,
    last_trade_at   TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_trader_accounts_volume ON trader_accounts(total_volume DESC);

CREATE TABLE IF NOT EXISTS global_stats (
    id                   VARCHAR(16) PRIMARY KEY,
    trades_quantity      BIGINT NOT NULL DEFAULT 0,
    buys_quantity        BIGINT NOT NULL DEFAULT 0,
    sells_quantity       BIGINT NOT NULL DEFAULT 0,
    collateral_volume    NUMERIC(78, 0) NOT NULL DEFAULT 0,
    total_fees           NUMERIC(78, 0) NOT NULL DEFAULT 0,
    average_trade_size   NUMERIC(78, 0) NOT NULL DEFAULT 0,
    platform_fee_revenue NUMERIC(78, 0) NOT NULL DEFAULT 0,  -- sum of FeeCharged
    last_updated_block   BIGINT NOT NULL,
    updated_at           TIMESTAMP NOT NULL
);

CREATE OR REPLACE VIEW top_markets_by_volume AS
SELECT * FROM market_orderbooks ORDER BY collateral_volume DESC LIMIT 100;
CREATE OR REPLACE VIEW top_traders_by_volume AS
SELECT * FROM trader_accounts ORDER BY total_volume DESC LIMIT 100;
CREATE OR REPLACE VIEW recent_large_trades AS
SELECT * FROM order_fills WHERE collateral >= 1000000000 ORDER BY block_number DESC LIMIT 100;  -- >= 1000 USD
