use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    // V1 (legacy CLOB) — historical fills before v2 cutover (2026-04-28)
    Abigen::new("CtfExchange", "abis/ctf_exchange.json")?
        .generate()?
        .write_to_file("src/abi/ctf_exchange.rs")?;

    Abigen::new("NegRiskExchange", "abis/neg_risk_exchange.json")?
        .generate()?
        .write_to_file("src/abi/neg_risk_exchange.rs")?;

    // V2 — deployed at block 84902353
    Abigen::new("CtfExchangeV2", "abis/ctf_exchange_v2.json")?
        .generate()?
        .write_to_file("src/abi/ctf_exchange_v2.rs")?;

    Abigen::new("NegRiskExchangeV2", "abis/neg_risk_exchange_v2.json")?
        .generate()?
        .write_to_file("src/abi/neg_risk_exchange_v2.rs")?;

    Ok(())
}
