<p align="center">
  <img src="./download.png" alt="Polymarket" width="200"/>
</p>

<h1 align="center">Polymarket Orderbook Substreams</h1>

<p align="center">
  <strong>Real-time orderbook analytics for Polymarket prediction markets on Polygon</strong>
</p>

<p align="center">
  <a href="https://substreams.dev/packages/polymarket-orderbook-substreams/v0.2.0">
    <img src="https://img.shields.io/badge/substreams.dev-v0.2.0-blue" alt="Substreams Package"/>
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

High-performance Substreams modules for extracting, processing, and persisting orderbook events from Polymarket's CTF Exchange and Neg Risk Exchange contracts on Polygon. Built with foundational stores for efficient parallel execution and includes ready-to-use SQL and Clickhouse sinks.

### Key Features

| Feature | Description |
|---------|-------------|
| **Dual Exchange Support** | Tracks both CTF Exchange and Neg Risk Exchange contracts |
| **Order Fill Events** | Detailed trade execution data with price calculations |
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
substreams run https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.2.0.spkg \
  map_all_order_fills \
  -e polygon.substreams.pinax.network:443 \
  -s 57000000 -t +1000

# Stream market analytics
substreams run https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.2.0.spkg \
  map_market_orderbooks \
  -e polygon.substreams.pinax.network:443 \
  -s 57000000 -t +1000

# Stream global platform stats
substreams run https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.2.0.spkg \
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

### Layer 1: Event Extraction

| Module | Description |
|--------|-------------|
| `map_ctf_exchange_order_filled` | Extracts OrderFilled events from CTF Exchange |
| `map_neg_risk_exchange_order_filled` | Extracts OrderFilled events from Neg Risk Exchange |
| `map_ctf_exchange_orders_matched` | Extracts OrdersMatched events from CTF Exchange |
| `map_neg_risk_exchange_orders_matched` | Extracts OrdersMatched events from Neg Risk Exchange |

### Layer 1.5: Combined Events

| Module | Description |
|--------|-------------|
| `map_all_order_fills` | Combines order fills from both exchanges into unified stream |

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
  https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.2.0.spkg

# Run sink
substreams-sink-sql run \
  "psql://user:pass@localhost:5432/polymarket_orderbook?sslmode=disable" \
  https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.2.0.spkg \
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
  https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.2.0.spkg

# Run sink
substreams-sink-sql run \
  "clickhouse://default:@localhost:9000/polymarket_orderbook" \
  https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.2.0.spkg \
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
| `maker_asset_id` | string | Maker's asset token ID |
| `taker_asset_id` | string | Taker's asset token ID |
| `maker_amount_filled` | string | Amount filled for maker |
| `taker_amount_filled` | string | Amount filled for taker |
| `fee` | string | Transaction fee |
| `side` | string | Trade side (buy/sell) |
| `price` | string | Calculated execution price |
| `block_number` | uint64 | Block number |

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

| Contract | Address | Description |
|----------|---------|-------------|
| CTF Exchange | `0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e` | Main prediction market exchange |
| Neg Risk Exchange | `0xC5d563A36AE78145C45a50134d48A1215220f80a` | Negative risk market exchange |

---

## Using as a Dependency

Import this package to build higher-level analytics:

```yaml
# substreams.yaml
imports:
  polymarket: https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.2.0.spkg

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

## Performance

| Metric | Value |
|--------|-------|
| Start Block | 57,000,000 (Polymarket launch) |
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
