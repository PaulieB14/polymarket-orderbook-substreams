# Polymarket Orderbook Substreams 📊

Real-time orderbook analytics for Polymarket prediction markets using Substreams technology.

## 🌟 Overview

This Substreams package provides comprehensive real-time orderbook tracking for Polymarket, offering superior performance and capabilities compared to traditional subgraph approaches.

## 🚀 Features

- **Real-time Order Tracking**: Live monitoring of order fills and matches
- **Market Analytics**: Per-market orderbook statistics and depth analysis  
- **Trader Profiling**: Individual trader statistics and behavior analysis
- **Global Metrics**: Platform-wide trading statistics
- **Multi-Exchange Support**: Tracks both CTF Exchange and Neg Risk Exchange
- **Enhanced Performance**: Substreams-powered real-time processing

## 📊 Data Entities

- **OrderFilledEvents**: Individual trade executions
- **OrdersMatchedEvents**: Order matching events
- **MarketOrderbooks**: Market-level orderbook state
- **Accounts**: Trader statistics and profiles
- **GlobalOrderbookStats**: Platform-wide metrics
- **OrderbookAnalytics**: Combined analytics output

## 🏗️ Architecture

### Tracked Contracts
- **CTF Exchange**: `0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e`
- **Neg Risk Exchange**: `0xC5d563A36AE78145C45a50134d48A1215220f80a`
- **USDC Collateral**: `0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174`

### Key Modules
1. `map_ctf_exchange_order_filled` - CTF exchange order fills
2. `map_neg_risk_exchange_order_filled` - Neg risk exchange order fills
3. `map_market_orderbooks` - Market-level aggregations
4. `map_trader_accounts` - Individual trader analytics
5. `map_global_orderbook_stats` - Platform statistics
6. `map_orderbook_analytics` - Combined analytics

## 🛠️ Quick Start

### Prerequisites
- [Substreams CLI](https://docs.substreams.dev/getting-started/installation)
- Rust toolchain
- Authentication token

### Build
```bash
# Generate protobuf types
substreams protogen

# Build the substream
substreams build

# Test locally
substreams run substreams.yaml map_orderbook_analytics -e polygon.streamingfast.io:443
```

### Usage Examples

#### Track Order Fills
```bash
substreams run substreams.yaml map_ctf_exchange_order_filled -s 66000000 -t +10
```

#### Monitor Market Orderbooks  
```bash
substreams run substreams.yaml map_market_orderbooks -s 66000000 -t +5
```

#### Get Global Statistics
```bash
substreams run substreams.yaml map_global_orderbook_stats -s 66000000 -t +1
```

## 🔄 Advantages over Subgraph

- ⚡ **Real-time Processing**: Live streaming vs eventual consistency
- 🚀 **Better Performance**: Optimized for high-frequency data
- 🛠️ **Flexible Transformations**: More powerful data processing
- 📊 **Enhanced Analytics**: Advanced calculations and aggregations
- 🔗 **Easy Integration**: Works with existing Substreams infrastructure

## 📈 Use Cases

- **Trading Dashboards**: Real-time orderbook visualization
- **Market Analysis**: Volume, liquidity, and price analytics
- **Trader Insights**: Individual performance tracking
- **Risk Management**: Position and exposure monitoring
- **Arbitrage Detection**: Cross-market opportunity identification

## 🤝 Related Projects

- [Polymarket P&L Substreams](https://substreams.dev/packages/polymarket-pnl/v0.3.1)
- [Polymarket Orders Subgraph](https://github.com/PaulieB14/Polymarket-Orders)

## 📄 License

MIT License

---

**Built with ❤️ for the Polymarket community using Substreams technology**
