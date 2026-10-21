use super::*;

fn w(v: u64) -> Vec<u8> { let mut b = vec![0u8; 32]; b[24..].copy_from_slice(&v.to_be_bytes()); b }
fn a(last: u8) -> Vec<u8> { let mut b = vec![0u8; 32]; b[31] = last; b }

#[test]
fn v2_buy_fill() {
    let data = [w(0), w(7), w(600), w(1000), w(6), vec![0; 32], vec![0; 32]].concat();
    let t = [d::ORDER_FILLED_V2.to_vec(), a(1), a(2), a(3)];
    let topics: Vec<&[u8]> = t.iter().map(|x| x.as_slice()).collect();
    let f = d::decode_fill(&topics, &data).unwrap();
    assert!(f.buy);
    assert_eq!(f.maker_asset_id.to_string(), "0");
    assert_eq!(f.taker_asset_id.to_string(), "7");
    let (s, c) = f.shares_and_collateral();
    assert_eq!((s.to_string(), c.to_string()), ("1000".into(), "606".into()));
    assert_eq!(d::price(&c, &s), "0.606000");
}

#[test]
fn v1_sell_fill_side_from_asset_ids() {
    // v1 SELL: makerAssetId = token, takerAssetId = 0 (collateral); fee taken from proceeds
    let data = [w(9), w(0), w(1000), w(400), w(4)].concat();
    let t = [d::ORDER_FILLED_V1.to_vec(), a(1), a(2), a(3)];
    let topics: Vec<&[u8]> = t.iter().map(|x| x.as_slice()).collect();
    let f = d::decode_fill(&topics, &data).unwrap();
    assert!(!f.buy && f.v1);
    assert_eq!(f.token_id.to_string(), "9");
    let (s, c) = f.shares_and_collateral();
    assert_eq!((s.to_string(), c.to_string()), ("1000".into(), "396".into()));
}

#[test]
fn exchanges_and_formatting() {
    assert_eq!(d::exchange_of(&d::EXCHANGE_V3), Some(("exchange_v3", false)));
    assert!(d::is_exchange_hex("e3333700ca9d93003f00f0f71f8515005f6c00aa"));
    assert!(!d::is_exchange_hex("0000000000000000000000000000000000000001"));
    assert_eq!(utils::scale6(&BigInt::from(1_500_000u64)), "1.500000");
    assert_eq!(utils::unix_to_timestamp(0), "1970-01-01 00:00:00");
    assert_eq!(d::price(&BigInt::from(1u64), &BigInt::zero()), "0.000000");
}

#[test]
fn resolution_payouts_array() {
    // ConditionResolved data: offset(32), len 2, [1, 0]
    let data = [w(32), w(2), w(1), w(0)].concat();
    let p: Vec<String> = d::uint_array(&data, 0).unwrap().iter().map(|v| v.to_string()).collect();
    assert_eq!(p, vec!["1", "0"]);
}
