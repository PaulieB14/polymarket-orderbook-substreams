//! Log decoding for the Polymarket exchanges, written against raw topics/data so it works on
//! lazy log views. Event layouts and fee semantics:
//! - CTF Exchange v1: github.com/Polymarket/ctf-exchange src/exchange/interfaces/ITrading.sol,
//!   src/exchange/interfaces/IFees.sol, src/exchange/mixins/Trading.sol
//! - CTF Exchange v2 / Neg Risk v2: github.com/Polymarket/ctf-exchange-v2 (abis/ctf_exchange_v2.json)
//! - ExchangeV3 (Protocol V2): github.com/Polymarket/polymarket-v2-external src/exchange/Exchange.sol
//! - Conditional Tokens ConditionResolution; V2 modules ConditionResolved:
//!   polymarket-v2-external src/modules/abstract/BaseModule.sol
//! Addresses: https://docs.polymarket.com/resources/contracts (v1 exchanges: Polymarket/ctf-exchange).
use hex_literal::hex;
use substreams::scalar::BigInt;
use substreams::Hex;

pub type Addr = [u8; 20];

pub const CTF_EXCHANGE_V1: Addr = hex!("4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e");
pub const NEG_RISK_EXCHANGE_V1: Addr = hex!("c5d563a36ae78145c45a50134d48a1215220f80a");
pub const CTF_EXCHANGE_V2: Addr = hex!("e111180000d2663c0091e4f400237545b87b996b");
pub const NEG_RISK_EXCHANGE_V2: Addr = hex!("e2222d279d744050d28e00520010520000310f59");
pub const EXCHANGE_V3: Addr = hex!("e3333700ca9d93003f00f0f71f8515005f6c00aa");
pub const CONDITIONAL_TOKENS: Addr = hex!("4d97dcd97ec945f40cf65f87097ace5ea0476045");
pub const BINARY_MODULE: Addr = hex!("1000008dd9001b968442c1000017eae6e0da00ba");
pub const NEG_RISK_MODULE: Addr = hex!("200000900045e3b6259600682756002200028933");

/// keccak256("OrderFilled(bytes32,address,address,uint256,uint256,uint256,uint256,uint256)")
pub const ORDER_FILLED_V1: [u8; 32] = hex!("d0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f6");
/// keccak256("OrderFilled(bytes32,address,address,uint8,uint256,uint256,uint256,uint256,bytes32,bytes32)")
pub const ORDER_FILLED_V2: [u8; 32] = hex!("d543adfd945773f1a62f74f0ee55a5e3b9b1a28262980ba90b1a89f2ea84d8ee");
/// keccak256("OrdersMatched(bytes32,address,uint256,uint256,uint256,uint256)")
pub const ORDERS_MATCHED_V1: [u8; 32] = hex!("63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c");
/// keccak256("OrdersMatched(bytes32,address,uint8,uint256,uint256,uint256)")
pub const ORDERS_MATCHED_V2: [u8; 32] = hex!("174b3811690657c217184f89418266767c87e4805d09680c39fc9c031c0cab7c");
/// keccak256("FeeCharged(address,uint256,uint256)") — v1
pub const FEE_CHARGED_V1: [u8; 32] = hex!("acffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4");
/// keccak256("FeeCharged(address,uint256)") — v2 / V3
pub const FEE_CHARGED_V2: [u8; 32] = hex!("55bb3cade9d43b798a4fe5ffdd05024b2d7870df53920673bfc7e68047cd0ab1");
/// keccak256("ConditionResolution(bytes32,address,bytes32,uint256,uint256[])")
pub const CONDITION_RESOLUTION: [u8; 32] = hex!("b44d84d3289691f71497564b85d4233648d9dbae8cbdbb4329f301c3a0185894");
/// keccak256("ConditionResolved(bytes31,uint256[])")
pub const CONDITION_RESOLVED: [u8; 32] = hex!("e560c1a2847d4cf2e66de11fe592605556989449f38cc3ab96e2e287daa68caa");

pub fn exchange_of(a: &[u8]) -> Option<(&'static str, bool)> {
    // (label, is_v1)
    if a == CTF_EXCHANGE_V2 { return Some(("ctf_v2", false)); }
    if a == NEG_RISK_EXCHANGE_V2 { return Some(("neg_risk_v2", false)); }
    if a == EXCHANGE_V3 { return Some(("exchange_v3", false)); }
    if a == CTF_EXCHANGE_V1 { return Some(("ctf_v1", true)); }
    if a == NEG_RISK_EXCHANGE_V1 { return Some(("neg_risk_v1", true)); }
    None
}

pub fn is_exchange_hex(a: &str) -> bool {
    let a = a.trim_start_matches("0x");
    [CTF_EXCHANGE_V1, NEG_RISK_EXCHANGE_V1, CTF_EXCHANGE_V2, NEG_RISK_EXCHANGE_V2, EXCHANGE_V3]
        .iter()
        .any(|e| Hex::encode(e) == a)
}

pub fn word(data: &[u8], i: usize) -> Option<&[u8]> {
    data.get(i * 32..i * 32 + 32)
}
pub fn uint(data: &[u8], i: usize) -> Option<BigInt> {
    word(data, i).map(BigInt::from_unsigned_bytes_be)
}
pub fn addr_of(w: &[u8]) -> String {
    Hex::encode(&w[12..32])
}
pub fn uint_array(data: &[u8], i: usize) -> Option<Vec<BigInt>> {
    let off = uint(data, i)?.to_u64() as usize;
    let len = uint(&data[off..], 0)?.to_u64() as usize;
    (0..len).map(|k| uint(&data[off + 32..], k)).collect()
}

pub struct Fill {
    pub order_hash: String,
    pub maker: String,
    pub taker: String,
    pub buy: bool,
    pub side_raw: u32,
    pub token_id: BigInt,
    pub maker_asset_id: BigInt,
    pub taker_asset_id: BigInt,
    pub maker_amount: BigInt,
    pub taker_amount: BigInt,
    pub fee: BigInt,
    pub builder: String,
    pub metadata: String,
    pub v1: bool,
}

fn sat_sub(a: &BigInt, b: &BigInt) -> BigInt {
    if a > b { a.clone() - b.clone() } else { BigInt::zero() }
}

impl Fill {
    /// (shares, collateral) for the order owner. v1: owner receives `taking - fee` of the asset
    /// it takes. v2/V3: fee is collateral; BUY pays maker_amount + fee, SELL receives taker_amount - fee.
    pub fn shares_and_collateral(&self) -> (BigInt, BigInt) {
        match (self.v1, self.buy) {
            (true, true) => (sat_sub(&self.taker_amount, &self.fee), self.maker_amount.clone()),
            (true, false) => (self.maker_amount.clone(), sat_sub(&self.taker_amount, &self.fee)),
            (false, true) => (self.taker_amount.clone(), self.maker_amount.clone() + self.fee.clone()),
            (false, false) => (self.maker_amount.clone(), sat_sub(&self.taker_amount, &self.fee)),
        }
    }
}

pub fn decode_fill(topics: &[&[u8]], data: &[u8]) -> Option<Fill> {
    if topics.len() < 4 { return None; }
    let (order_hash, maker, taker) = (Hex::encode(topics[1]), addr_of(topics[2]), addr_of(topics[3]));
    if topics[0] == ORDER_FILLED_V1 {
        let ma = uint(data, 0)?;
        let ta = uint(data, 1)?;
        let buy = ma.is_zero();
        Some(Fill {
            order_hash, maker, taker, buy, side_raw: if buy { 0 } else { 1 },
            token_id: if buy { ta.clone() } else { ma.clone() },
            maker_asset_id: ma, taker_asset_id: ta,
            maker_amount: uint(data, 2)?, taker_amount: uint(data, 3)?, fee: uint(data, 4)?,
            builder: String::new(), metadata: String::new(), v1: true,
        })
    } else if topics[0] == ORDER_FILLED_V2 {
        let side = uint(data, 0)?.to_u64() as u32;
        let token = uint(data, 1)?;
        let buy = side == 0;
        Some(Fill {
            order_hash, maker, taker, buy, side_raw: side,
            maker_asset_id: if buy { BigInt::zero() } else { token.clone() },
            taker_asset_id: if buy { token.clone() } else { BigInt::zero() },
            token_id: token,
            maker_amount: uint(data, 2)?, taker_amount: uint(data, 3)?, fee: uint(data, 4)?,
            builder: Hex::encode(word(data, 5)?), metadata: Hex::encode(word(data, 6)?), v1: false,
        })
    } else {
        None
    }
}

/// collateral / shares as a decimal string with 6 places (prices are 0..1).
pub fn price(collateral: &BigInt, shares: &BigInt) -> String {
    if shares.is_zero() { return "0.000000".into(); }
    let p = collateral.clone() * BigInt::from(1_000_000u64) / shares.clone();
    let s = BigInt::from(1_000_000u64);
    let int = p.clone() / s.clone();
    let frac = (p - int.clone() * s).to_u64();
    format!("{}.{:06}", int, frac)
}
