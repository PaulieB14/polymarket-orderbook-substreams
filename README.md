<p align="center">
  <img src="./download.png" alt="Polymarket" width="200"/>
</p>

<h1 align="center">Polymarket Orderbook Substreams</h1>

<p align="center">
  <strong>Real-time orderbook analytics for Polymarket prediction markets on Polygon</strong>
</p>

<p align="center">
  <a href="https://substreams.dev/packages/polymarket-orderbook-substreams/v1.0.0">
    <img src="https://img.shields.io/badge/substreams.dev-v1.0.0-blue" alt="Substreams Package"/>
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


Polymarket orderbook flow on Polygon: every fill, match, fee and resolution from CLOB v1, CLOB v2 and Protocol V2 (**ExchangeV3**). Includes per-market, per-trader and global aggregates, with schemas for PostgreSQL and ClickHouse.

## What's new in v1.0.0 (breaking)

| # | Change | Why |
|---|---|---|
| 1 | **ExchangeV3** `0xe3333700cA9d93003F00f0F71f8515005F6c00Aa` (Protocol V2) is indexed | Its fills were missing (705 in a 500-block sample) |
| 2 | **Single pass**: `map_events` replaces the 8 per-exchange maps | One block decode instead of eight |
| 3 | **blockFilter** (`ethcommon:index_events`) plus **lazy block decode** (`&BlockLazyView`) and constant address bytes | Blocks with no Polymarket logs are skipped; no `Hex::decode` per log |
| 4 | **FeeCharged** (v1 and v2/V3) → `fees` table. **Resolutions** (CTF `ConditionResolution`, Protocol V2 `ConditionResolved`) → `resolutions` table | New data |
| 5 | **OrdersMatched** → `orders_matched` table. v1 matches are now decoded | The v0.x v1 ABI had no indexed fields, so v1 `OrdersMatched` never matched |
| 6 | Aggregates are **cumulative** (add stores) and written with **`upsert_row`** | v0.x wrote each fill's own values with `update_row` onto rows that were never created |
| 7 | `block_timestamp` on every event table | Time-range queries |
| 8 | Correct side and price: v1 side comes from `makerAssetId == 0`; price = collateral ÷ shares, fee-aware | v0.x used a digit-parity heuristic and maker ÷ taker amounts (inverted for sells) |
| 9 | ClickHouse schema with `PARTITION BY toYYYYMM(block_timestamp)` / `ORDER BY (token_id, block_timestamp, id)`, and `ReplacingMergeTree` for aggregates | Query performance |
| 10 | Deterministic ordering (BTreeMap) and unit tests | Reproducible output |

### Migrating from v0.x
- **Renamed or removed modules:**
  - `map_ctf_exchange_order_filled`, `map_neg_risk_exchange_order_filled`, `map_ctf_exchange_orders_matched`, `map_neg_risk_exchange_orders_matched`, and the four `*_v2_*` maps are replaced by **`map_events`** (`OrderbookEvents`: fills, matches, fees, resolutions).
  - **`map_all_order_fills`** is kept, with the same `OrderFilledEvents` type, now fed from `map_events`.
  - `store_markets`, `store_traders` and `store_global_stats` are replaced by **`store_totals`**, `store_last_price`, `store_first_trade` and `store_last_trade`.
  - `map_market_orderbooks`, `map_trader_accounts`, `map_global_orderbook_stats` and `db_out` keep their names.
- **Semantics:**
  - `order_fills.id` is now `tx_hash-log_index`.
  - `market_orderbooks.id` is the token id, and `condition_id` is dropped.
  - Market and global counts are per match (the taker order's fill). Trader stats are per order owner (`maker`), so the exchange address is no longer counted as a trader.
  - New `OrderFilledEvent` fields: `exchange`, `log_index`, `shares`, `collateral`.
- **SQL:** the schema changed. Recreate the tables (`substreams-sink-sql setup`) and resync from the start block.

## Quick start

```bash
substreams run polymarket-orderbook-substreams@v1.0.0 map_events -e polygon.streamingfast.io:443 -s -1000 -t 0
substreams-sink-sql setup "psql://user:pass@localhost:5432/ob?sslmode=disable" polymarket-orderbook-substreams@v1.0.0
substreams-sink-sql run   "psql://user:pass@localhost:5432/ob?sslmode=disable" polymarket-orderbook-substreams@v1.0.0
# ClickHouse: apply clickhouse-schema.sql, then use a clickhouse:// DSN
```

## Modules

```
Block ─(blockFilter, lazy)─ map_events ─┬─ map_all_order_fills
                                        ├─ store_totals ─────┬─ map_market_orderbooks ─┐
                                        ├─ store_last_price ─┘                         │
                                        ├─ store_first/last_trade ─ map_trader_accounts ┤
                                        ├─ map_global_orderbook_stats ──────────────────┤
                                        └───────────────────────────────────────────── db_out
```

## Semantics (from Polymarket `Trading.sol`)
- `maker` is the owner of the order that was filled, and `side` is that order's side. The taker order's own fill has the exchange as `taker`.
- v1: the owner receives `taking − fee`. v2/V3: the fee is collateral, so a BUY pays `makerAmountFilled + fee` and a SELL receives `takerAmountFilled − fee`. `shares` / `collateral` apply this.
- Amounts are raw 6-decimal base units.

## Contract addresses
From https://docs.polymarket.com/resources/contracts. The v1 exchanges come from github.com/Polymarket/ctf-exchange. All of them are in `src/decode.rs`.

| Contract | Address |
|---|---|
| CTF Exchange v1 / Neg Risk v1 | `0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e` / `0xC5d563A36AE78145C45a50134d48A1215220f80a` |
| CTF Exchange v2 / Neg Risk v2 | `0xE111180000d2663C0091e4f400237545B87B996B` / `0xe2222d279d744050d28e00520010520000310F59` |
| ExchangeV3 (Protocol V2) | `0xe3333700cA9d93003F00f0F71f8515005F6c00Aa` |
| Conditional Tokens | `0x4D97DCd97eC945f40cF65F87097ACe5EA0476045` |
| BinaryModule / NegRiskModule | `0x1000008dD9001B968442c1000017eaE6E0dA00Ba` / `0x200000900045e3B6259600682756002200028933` |

## Build
```bash
substreams build   # CLI >= 1.25, buffa codegen with lazy_views=true (buf.gen.yaml)
cargo test --lib
```

## Not included
- Unique-trader and active-market counts (not cheaply countable with stores). These fields stay 0.
- Order-book depth, spreads, volatility and similar fields in the proto are not computed. Fills don't carry resting-order state.

## License
MIT
