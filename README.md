<p align="center">
  <img src="./download.png" alt="Polymarket" width="200"/>
</p>

<h1 align="center">Polymarket Orderbook Substreams</h1>

<p align="center">
  <strong>Real-time orderbook analytics for Polymarket prediction markets on Polygon</strong>
</p>

<p align="center">
  <a href="https://substreams.dev/packages/polymarket-orderbook-substreams/v0.4.0">
    <img src="https://img.shields.io/badge/substreams.dev-v0.4.0-blue" alt="Substreams Package"/>
  </a>
  <a href="https://docs.polymarket.com/v2-migration">
    <img src="https://img.shields.io/badge/CLOB-v1%20%2B%20v2-brightgreen" alt="CLOB v1+v2"/>
  </a>
  <a href="https://polygon.technology/">
    <img src="https://img.shields.io/badge/network-Polygon-8247E5" alt="Polygon"/>
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-green" alt="License"/>
  </a>
  <a href="https://www.postgresql.org/">
    <img src="https://img.shields.io/badge/sink-PostgreSQL-336791" alt="PostgreSQL"/>
  </a>
  <a href="https://clickhouse.com/">
    <img src="https://img.shields.io/badge/sink-Clickhouse-FFCC01" alt="Clickhouse"/>
  </a>
</p>

---

## Overview

High-performance Substreams modules for extracting, processing, and persisting orderbook events from Polymarket's CTF Exchange and Neg Risk Exchange contracts on Polygon — across **both CLOB v1 and CLOB v2**. Built with foundational stores for efficient parallel execution and ready-to-use SQL and Clickhouse sinks.

---

## CLOB v2 ready (since v0.4.0)

Polymarket migrated to [CLOB v2](https://docs.polymarket.com/v2-migration) at the **2026-04-28 ~11:00 UTC** cutover. v2 ships fresh Exchange contracts at new addresses, a redesigned `OrderFilled` event, a different fee model, and a new collateral wrapper (pUSD). This package indexes both contract generations side-by-side so a single output stream spans the migration without breaking existing consumers.

### What changed in CLOB v2

| Concept | CLOB v1 | CLOB v2 |
|---------|---------|---------|
| **Exchange contracts** | `0x4bfb…982e` (CTF) / `0xC5d5…f80a` (NegRisk) | `0xE111…996B` (CTF) / `0xe222…0F59` (NegRisk) — fresh deploys |
| **Order uniqueness** | `nonce` per maker | `timestamp` (ms) — nonces removed |
| **Order side** | Inferred from `makerAssetId == 0` | Explicit `side` enum on the order and event (`BUY=0`, `SELL=1`) |
| **OrderFilled event** | 8 fields, including `makerAssetId` + `takerAssetId` | 10 fields: single `tokenId` + `side` + new `builder` (bytes32) + `metadata` (bytes32) |
| **OrdersMatched event** | `makerAssetId` + `takerAssetId` + amounts | `takerOrderHash` (indexed) + `takerOrderMaker` (indexed) + `side` + `tokenId` + amounts |
| **Fees** | Embedded in order (`feeRateBps`), maker + taker | Protocol-determined at match time, **taker only**, dynamic per market via `getClobMarketInfo()` |
| **Collateral (wallet)** | USDC.e directly | **pUSD** — a 1:1-backed ERC-20 wrapper. USDC.e converts via `CollateralOnramp.wrap()` |
| **Collateral (CTF level)** | USDC.e | USDC.e (unchanged — pUSD is purely wallet-facing) |
| **Builder attribution** | HMAC headers on API orders | Single `builderCode` (bytes32) on the order, surfaced as `builder` on the event |
| **EIP-712 domain version** | `"1"` | `"2"` for exchange signing (L1 API auth still `"1"`) |
| **Open orders at cutover** | — | All wiped during the maintenance window |

### How this package handles it

1. **Parallel module set.** Four new map modules at `initialBlock: 84,902,353` (v2 deploy block) extract OrderFilled and OrdersMatched events from the two v2 Exchange contracts. The four legacy v1 modules continue at `initialBlock: 57,000,000` for full historical backfill.
2. **Unified output.** `map_all_order_fills` merges all four fill streams (v1 CTF + v1 NegRisk + v2 CTF + v2 NegRisk) and sorts by ordinal, so downstream stores and analytics see one continuous order flow that spans the cutover with no gap.
3. **`exchange_version` column.** Every `OrderFilledEvent` and `OrdersMatchedEvent` carries an `exchange_version` field (`"v1"` or `"v2"`) so you can filter, partition, or audit by generation.
4. **v2-only fields surfaced.** `token_id`, `side_raw`, `builder`, and `metadata` are first-class columns on `order_fills` for v2 rows (empty / `0` for v1).
5. **Backward-compat shape.** Legacy `maker_asset_id` / `taker_asset_id` fields are *populated for v2 fills* using the `(side, tokenId)` mapping (BUY: `maker="0"`, `taker=tokenId`; SELL: `maker=tokenId`, `taker="0"`). Existing queries that key on these fields — including the foundational stores and Clickhouse materialized views shipped here — keep working unchanged.
6. **Authoritative side.** v2 ships the trade direction directly in the event (`side` enum). v0.4.0 uses this for v2 rows; v1 rows continue to use the legacy parity-based heuristic.
7. **Fee column semantics.** v2 fees are taker-only and protocol-determined at match time; the same `fee` field surfaces a single realized taker fee.

### What stays the same

- Same chain (Polygon), same Firehose source.
- Same `db_out` SQL/Clickhouse sink path.
- Same start block for v1 history (57,000,000); v2 fills appear automatically at deploy block 84,902,353.
- Same module names and proto field numbers — v0.4.0 is a strict superset of v0.3.1.

### Key Features

| Feature | Description |
|---------|-------------|
| **CLOB v1 + v2** | Indexes both legacy and new Exchange contracts in a single unified stream |
| **Dual Exchange Support** | Tracks CTF Exchange and Neg Risk Exchange on each generation |
| **Order Fill Events** | Trade execution data with price calculations and authoritative v2 `side` |
| **Builder Attribution** | v2 `builder` and `metadata` (bytes32) surfaced as columns |
| **Market Analytics** | Volume, trades, buy/sell ratios, average trade sizes |
| **Trader Analytics** | Per-trader volume, trade counts, activity tracking |
| **Global Statistics** | Platform-wide metrics and fee revenue |
| **PostgreSQL Sink** | Ready-to-use SQL schema for relational queries |
| **Clickhouse Sink** | High-performance analytics with materialized views |

---

## Quick Start

### Installation

```bash
# Install Substreams CLI
brew install streamingfast/tap/substreams

# Authenticate
substreams auth
```

### Run Streaming Modules

```bash
# Stream all order fills from both exchanges
substreams run https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.4.0.spkg \
  map_all_order_fills \
  -e polygon.substreams.pinax.network:443 \
  -s 57000000 -t +1000

# Stream market analytics
substreams run https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.4.0.spkg \
  map_market_orderbooks \
  -e polygon.substreams.pinax.network:443 \
  -s 57000000 -t +1000

# Stream global platform stats
substreams run https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.4.0.spkg \
  map_global_orderbook_stats \
  -e polygon.substreams.pinax.network:443 \
  -s 57000000 -t +1000
```

---

## Architecture

```
                              Polygon Blockchain
                                     │
                                     ▼
                            ┌─────────────────┐
                            │ Firehose Blocks │
                            └─────────────────┘
                                     │
                    ┌────────────────┴────────────────┐
                    ▼                                 ▼
        ┌───────────────────┐             ┌───────────────────┐
        │   CTF Exchange    │             │  Neg Risk Exchange│
        │ Order Fill Events │             │  Order Fill Events│
        └───────────────────┘             └───────────────────┘
                    │                                 │
                    └────────────┬────────────────────┘
                                 ▼
                    ┌────────────────────────┐
                    │   map_all_order_fills  │
                    │  (Combined Event Stream)│
                    └────────────────────────┘
                                 │
            ┌────────────────────┼────────────────────┐
            ▼                    ▼                    ▼
   ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
   │  store_markets  │  │  store_traders  │  │store_global_stats│
   │ (Market Stats)  │  │ (Trader Stats)  │  │ (Platform Stats) │
   └─────────────────┘  └─────────────────┘  └─────────────────┘
            │                    │                    │
            ▼                    ▼                    ▼
   ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
   │map_market_      │  │map_trader_      │  │map_global_      │
   │orderbooks       │  │accounts         │  │orderbook_stats  │
   └─────────────────┘  └─────────────────┘  └─────────────────┘
            │                    │                    │
            └────────────────────┼────────────────────┘
                                 ▼
                    ┌────────────────────────┐
                    │        db_out          │
                    │   (SQL/Clickhouse)     │
                    └────────────────────────┘
                                 │
                    ┌────────────┴────────────┐
                    ▼                         ▼
           ┌─────────────────┐      ┌─────────────────┐
           │   PostgreSQL    │      │   Clickhouse    │
           └─────────────────┘      └─────────────────┘
```

---

## Modules

### Layer 1: Event Extraction (CLOB v1 — pre-cutover history)

| Module | Description | Initial Block |
|--------|-------------|--------------:|
| `map_ctf_exchange_order_filled` | OrderFilled events from CTF Exchange v1 | 57,000,000 |
| `map_neg_risk_exchange_order_filled` | OrderFilled events from Neg Risk Exchange v1 | 57,000,000 |
| `map_ctf_exchange_orders_matched` | OrdersMatched events from CTF Exchange v1 | 57,000,000 |
| `map_neg_risk_exchange_orders_matched` | OrdersMatched events from Neg Risk Exchange v1 | 57,000,000 |

### Layer 1: Event Extraction (CLOB v2 — deployed 2026-03-31, cutover 2026-04-28)

| Module | Description | Initial Block |
|--------|-------------|--------------:|
| `map_ctf_exchange_v2_order_filled` | OrderFilled events from CTF Exchange V2 | 84,902,353 |
| `map_neg_risk_exchange_v2_order_filled` | OrderFilled events from Neg Risk CTF Exchange V2 | 84,902,353 |
| `map_ctf_exchange_v2_orders_matched` | OrdersMatched events from CTF Exchange V2 | 84,902,353 |
| `map_neg_risk_exchange_v2_orders_matched` | OrdersMatched events from Neg Risk CTF Exchange V2 | 84,902,353 |

### Layer 1.5: Combined Events

| Module | Description |
|--------|-------------|
| `map_all_order_fills` | Merges v1 + v2 fills from CTF and Neg Risk into a single ordinal-sorted stream |

### Layer 2: Foundational Stores

| Store | Key Pattern | Description |
|-------|-------------|-------------|
| `store_markets` | `market:{asset_id}` | Market-level statistics (volume, trades, prices) |
| `store_traders` | `trader:{address}` | Trader analytics (volume, trade count, fees) |
| `store_global_stats` | `global` | Platform-wide metrics |

### Layer 3: Analytics Outputs

| Module | Description |
|--------|-------------|
| `map_market_orderbooks` | Real-time market snapshots on updates |
| `map_trader_accounts` | Trader account updates for leaderboards |
| `map_global_orderbook_stats` | Global platform statistics |
| `map_orderbook_analytics` | Comprehensive analytics combining all stores |

### Layer 4: Database Sinks

| Module | Description |
|--------|-------------|
| `db_out` | PostgreSQL sink with normalized tables |
| `clickhouse_out` | Clickhouse sink optimized for analytics |

---

## SQL Sink (PostgreSQL)

### Setup

```bash
# Create database
createdb polymarket_orderbook

# Apply schema
psql -d polymarket_orderbook -f schema.sql

# Setup sink
substreams-sink-sql setup \
  "psql://user:pass@localhost:5432/polymarket_orderbook?sslmode=disable" \
  https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.4.0.spkg

# Run sink
substreams-sink-sql run \
  "psql://user:pass@localhost:5432/polymarket_orderbook?sslmode=disable" \
  https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.4.0.spkg \
  -e polygon.substreams.pinax.network:443
```

### Example Queries

```sql
-- Top markets by volume
SELECT * FROM top_markets_by_volume;

-- Top traders by volume
SELECT * FROM top_traders_by_volume;

-- Recent large trades (> 1000 USDC)
SELECT * FROM recent_large_trades;

-- Market activity over time
SELECT
  DATE(created_at) as date,
  COUNT(*) as trades,
  SUM(taker_amount_filled::numeric / 1e18) as volume
FROM order_fills
GROUP BY DATE(created_at)
ORDER BY date DESC;
```

---

## Clickhouse Sink

### Setup

```bash
# Create database
clickhouse-client -q "CREATE DATABASE polymarket_orderbook"

# Apply schema
clickhouse-client -d polymarket_orderbook < clickhouse-schema.sql

# Setup sink
substreams-sink-sql setup \
  "clickhouse://default:@localhost:9000/polymarket_orderbook" \
  https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.4.0.spkg

# Run sink
substreams-sink-sql run \
  "clickhouse://default:@localhost:9000/polymarket_orderbook" \
  https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.4.0.spkg \
  -e polygon.substreams.pinax.network:443
```

### Example Queries

```sql
-- Top markets by 24h volume
SELECT
    market_id,
    sum(total_volume) as volume_24h,
    sum(trades_count) as trades_24h
FROM hourly_volume
WHERE hour >= now() - INTERVAL 24 HOUR
GROUP BY market_id
ORDER BY volume_24h DESC
LIMIT 10;

-- Trader leaderboard
SELECT
    id,
    total_volume,
    trades_quantity,
    total_fees
FROM trader_analytics FINAL
WHERE is_active = 1
ORDER BY total_volume DESC
LIMIT 100;

-- Hourly volume trend
SELECT
    hour,
    sum(total_volume) as volume,
    sum(trades_count) as trades
FROM hourly_volume
WHERE hour >= now() - INTERVAL 7 DAY
GROUP BY hour
ORDER BY hour;
```

---

## Data Schema

### OrderFilledEvent

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique event identifier |
| `transaction_hash` | string | Transaction hash |
| `order_hash` | string | Order hash |
| `maker` | string | Maker address |
| `taker` | string | Taker address |
| `maker_asset_id` | string | Maker's asset token ID (v2: derived from `side`+`token_id` for backward compat) |
| `taker_asset_id` | string | Taker's asset token ID (v2: derived from `side`+`token_id`) |
| `maker_amount_filled` | string | Amount filled for maker |
| `taker_amount_filled` | string | Amount filled for taker |
| `fee` | string | Realized taker fee (v2 fees are protocol-determined at match time) |
| `side` | string | Trade side string (`buy` / `sell`) |
| `price` | string | Calculated execution price |
| `block_number` | uint64 | Block number |
| `exchange_version` | string | `"v1"` or `"v2"` — identifies which Exchange generation emitted the fill |
| `token_id` | string | Conditional token ID (v1: derived non-zero asset; v2: emitted directly) |
| `side_raw` | uint32 | v2 side enum: `0`=BUY, `1`=SELL (`0` for v1) |
| `builder` | string | bytes32 builder attribution code, hex-encoded (v2 only; empty for v1) |
| `metadata` | string | bytes32 order metadata, hex-encoded (v2 only; empty for v1) |

### MarketOrderbook

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Market identifier |
| `trades_quantity` | uint64 | Total trade count |
| `buys_quantity` | uint64 | Buy trade count |
| `sells_quantity` | uint64 | Sell trade count |
| `collateral_volume` | string | Total volume |
| `average_trade_size` | string | Average trade size |
| `total_fees` | string | Total fees collected |
| `mid_price` | string | Current mid price |

---

## Contract Addresses

### CLOB v1 (legacy — historical fills only after the 2026-04-28 cutover)

| Contract | Address |
|----------|---------|
| CTF Exchange v1 | `0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e` |
| Neg Risk Exchange v1 | `0xC5d563A36AE78145C45a50134d48A1215220f80a` |

### CLOB v2 (deployed 2026-03-31 by Polymarket Deployer 1)

| Contract | Address |
|----------|---------|
| CTF Exchange V2 | `0xE111180000d2663C0091e4f400237545B87B996B` |
| Neg Risk CTF Exchange V2 | `0xe2222d279d744050d28e00520010520000310F59` |

V2 deploy block: **84,902,353** · Cutover: **2026-04-28 ~11:00 UTC**

---

## Using as a Dependency

Import this package to build higher-level analytics:

```yaml
# substreams.yaml
imports:
  polymarket: https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.4.0.spkg

modules:
  - name: my_analytics_module
    kind: map
    inputs:
      - map: polymarket:map_all_order_fills
    output:
      type: proto:my.custom.Analytics
```

---

## Build from Source

```bash
# Clone repository
git clone https://github.com/PaulieB14/polymarket-orderbook-substreams
cd polymarket-orderbook-substreams

# Build
substreams build

# Run locally
substreams run substreams.yaml map_all_order_fills \
  -e polygon.substreams.pinax.network:443 \
  -s 57000000 -t +100
```

---

## Migrating from v0.3.1

If you ran v0.3.1 (or earlier) against a live sink, follow these steps before pointing it at v0.4.0:

1. **Add the new columns** (defaults preserve v1 history):
   ```sql
   ALTER TABLE order_fills
     ADD COLUMN exchange_version VARCHAR(4) NOT NULL DEFAULT 'v1',
     ADD COLUMN token_id VARCHAR,
     ADD COLUMN side_raw SMALLINT NOT NULL DEFAULT 0,
     ADD COLUMN builder VARCHAR(66),
     ADD COLUMN metadata VARCHAR(66);
   CREATE INDEX idx_order_fills_token_id ON order_fills(token_id);
   CREATE INDEX idx_order_fills_version ON order_fills(exchange_version);
   ```
2. **Re-point the sink at v0.4.0** — no resync required; v2 modules pick up at block 84,902,353 and v1 modules continue from your existing cursor.
3. **Optional**: rebuild Clickhouse materialized views to aggregate by `token_id` instead of `maker_asset_id` (the v1 schema lumped all collateral=0 buys under one key).

## Performance

| Metric | Value |
|--------|-------|
| Start Block (v1) | 57,000,000 (Polymarket launch) |
| Start Block (v2) | 84,902,353 (CLOB v2 deploy) |
| Parallel Execution | Optimized with foundational stores |
| Latency | Low latency with direct event extraction |
| Sink Support | PostgreSQL, Clickhouse |

---

## Related Projects

- [Polymarket P&L Subgraph](https://github.com/PaulieB14/polymarket-profit-and-loss-) - Track profit & loss
- [Substreams Documentation](https://substreams.streamingfast.io) - Learn more about Substreams
- [Substreams Sink SQL](https://github.com/streamingfast/substreams-sink-sql) - SQL sink documentation

---

## License

MIT License - see [LICENSE](LICENSE) for details.

---

<p align="center">
  <strong>Built for the Polymarket community</strong><br/>
  <sub>Powered by StreamingFast Substreams</sub>
</p>
