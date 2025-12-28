# Polymarket Orderbook Substreams

The Polymarket Orderbook Substreams contains a comprehensive set of modules that allow you to easily retrieve real-time orderbook analytics from Polymarket prediction markets on Polygon, including order fills, market statistics, trader analytics, and advanced market microstructure data.

![Polymarket Logo](./polymarket-logo.png)

The `substreams.yaml` file defines all the different modules available, and also provides you with documentation about the usage of every module.

## Using this module to speed up a substreams

### Using the full "ethereum block" object (simplest if changing an existing substreams)

In your `substreams.yaml`:

**Import this .spkg:**
```yaml
imports:
  polymarket: https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.1.0.spkg
```

**Replace any `source: sf.ethereum.type.v2.Block` input** with `map: polymarket:map_orderbook_analytics` (you will be getting comprehensive orderbook analytics with market data, trader statistics, and real-time insights)

**Add block filtering to your "entry modules"** (any module reading blocks or transactions before emitting your custom types):

If you want to track specific markets by asset ID, use the market filtering like this:
```yaml
blockFilter:
  module: polymarket:store_markets
  query:
    string: "market:12345 || market:67890"
```

If you need to track specific traders, use the trader filtering like this:
```yaml
blockFilter:
  module: polymarket:store_traders  
  query:
    string: "trader:0x1234... || trader:0x5678..."
```

### Using the new 'orderbook analytics' object (simplest if writing a new substreams)

In your `substreams.yaml`:

**Import this .spkg:**
```yaml
imports:
  polymarket: https://spkg.io/PaulieB14/polymarket-orderbook-substreams-v0.1.0.spkg
```

**Set one of the analytics modules** (along with `source: sf.substreams.v1.Clock` if you need block number/timestamp) as your module input:

```yaml
- name: my_polymarket_module
  kind: map
  inputs:
    - source: sf.substreams.v1.Clock
    - map: polymarket:map_orderbook_analytics
```

**Set parameters** to filter the data that you want to be fed to your module:

```yaml
params:
  polymarket:store_markets: "asset_id:12345"
  polymarket:store_traders: "min_volume:10000"
```

**Run `substreams protogen`** against your `substreams.yaml` to create the rust bindings of the protobuf definitions inside the substreams.

## Modules

### `map_orderbook_analytics` (map)
This module provides comprehensive orderbook analytics combining market data, trader statistics, global metrics, market alerts, and arbitrage opportunities. It's the main output module that aggregates all orderbook intelligence.

### `store_markets` (store)
This foundational store efficiently tracks market-level orderbook data including:
- Trade volumes and counts
- Price movements and volatility
- Liquidity scores and market depth
- Real-time bid/ask levels

**Example keys:** `market:12345`, `market:67890`

Use it to only get blocks that contain trades for specific markets:
```yaml
- name: my_module
  ...
  blockFilter:
    module: store_markets
    query:
      string: "market:12345 || market:67890"
```

### `store_traders` (store)
This foundational store tracks comprehensive trader analytics including:
- Trading volumes and P&L
- Risk metrics and Sharpe ratios
- Trader classification (retail, whale, market maker, arbitrageur)
- Win rates and performance metrics

**Example keys:** `trader:0x1234...`, `trader:0x5678...`

Use it to track specific high-value traders:
```yaml
- name: my_module
  ...
  blockFilter:
    module: store_traders
    query:
      string: "trader:0x1234567890abcdef || min_volume:100000"
```

### `map_ctf_exchange_order_filled` (map)
This module extracts and processes OrderFilled events from the CTF Exchange contract (`0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e`), providing detailed trade execution data with price calculations and trade side determination.

### `map_neg_risk_exchange_order_filled` (map)
This module extracts and processes OrderFilled events from the Neg Risk CTF Exchange contract (`0xC5d563A36AE78145C45a50134d48A1215220f80a`), handling negative risk markets with specialized processing.

### `map_market_orderbooks` (map)
This module uses the `store_markets` foundational store to provide real-time market orderbook snapshots with:
- Current market statistics
- Liquidity analysis
- Price level data
- Market health indicators

### `map_trader_accounts` (map)
This module uses the `store_traders` foundational store to provide comprehensive trader analytics with:
- Portfolio performance metrics
- Risk assessment scores
- Trading behavior classification
- Historical performance data

### `map_global_orderbook_stats` (map)
This module aggregates platform-wide statistics including:
- Total trading volumes
- Active market counts
- Unique trader metrics
- Platform health indicators

## Advanced Features

### 🚀 **Parallel Execution Optimized**
- Designed for Substreams' 25K block segments
- Leverages backward/forward parallel execution
- Foundational stores for efficient state management
- Optimized for production-grade performance

### 📊 **Market Microstructure Analytics**
- Real-time liquidity scoring
- Volatility calculations
- Market depth analysis
- Spread monitoring

### 🎯 **Trader Intelligence**
- Sharpe ratio calculations
- Win rate analysis
- Risk scoring algorithms
- Automated trader classification

### ⚡ **Real-time Insights**
- Market alert system
- Arbitrage opportunity detection
- Unusual activity monitoring
- Price movement alerts

## Contract Addresses

- **CTF Exchange:** `0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e`
- **Neg Risk Exchange:** `0xC5d563A36AE78145C45a50134d48A1215220f80a`
- **USDC Collateral:** `0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174`

## Quick Start

### Installation
```bash
# Install Substreams CLI
curl -sSf https://substreams.streamingfast.io/install | bash

# Clone and build
git clone https://github.com/PaulieB14/polymarket-orderbook-substreams
cd polymarket-orderbook-substreams
substreams build
```

### Usage Examples

#### Track All Orderbook Activity
```bash
substreams run substreams.yaml map_orderbook_analytics \
  -e polygon.streamingfast.io:443 \
  -s 66000000 -t +100
```

#### Monitor Specific Markets
```bash
substreams run substreams.yaml map_market_orderbooks \
  -e polygon.streamingfast.io:443 \
  -s 66000000 -t +50
```

#### Analyze Trader Behavior
```bash
substreams run substreams.yaml map_trader_accounts \
  -e polygon.streamingfast.io:443 \
  -s 66000000 -t +25
```

#### Get Global Platform Stats
```bash
substreams run substreams.yaml map_global_orderbook_stats \
  -e polygon.streamingfast.io:443 \
  -s 66000000 -t +10
```

## Performance Benchmarks

- **3-5x faster** than traditional subgraph indexing
- **Real-time processing** with <1 second latency
- **Parallel execution** across 15 workers in production
- **Efficient caching** via foundational stores

## Documentation

### Maps
- `map_orderbook_analytics` - Comprehensive orderbook analytics
- `map_market_orderbooks` - Market-level orderbook data  
- `map_trader_accounts` - Trader analytics and performance
- `map_global_orderbook_stats` - Platform-wide statistics
- `map_ctf_exchange_order_filled` - CTF exchange order fills
- `map_neg_risk_exchange_order_filled` - Neg risk exchange order fills

### Stores
- `store_markets` - Foundational market state management
- `store_traders` - Foundational trader analytics
- `store_global_stats` - Platform-wide metrics store

## Architecture

Built on [Substreams architecture](https://docs.substreams.dev/reference-material/architecture) with:
- **Foundational Stores** for efficient state management
- **Parallel Execution** for maximum performance  
- **Delta Tracking** for minimal update overhead
- **Advanced Analytics** for market intelligence

## Related Projects

- [Polymarket P&L Substreams](https://substreams.dev/packages/polymarket-pnl/v0.3.1)
- [Polymarket Orders Subgraph](https://github.com/PaulieB14/Polymarket-Orders)

## License

MIT License

---

**Built with ❤️ for the Polymarket community using advanced Substreams technology**

*Powered by StreamingFast infrastructure for maximum performance and reliability*
