use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    // Generate Rust bindings for CTF Exchange contract
    Abigen::new("CtfExchange", "abis/ctf_exchange.json")?
        .generate()?
        .write_to_file("src/abi/ctf_exchange.rs")?;

    // Generate Rust bindings for Neg Risk Exchange contract  
    Abigen::new("NegRiskExchange", "abis/neg_risk_exchange.json")?
        .generate()?
        .write_to_file("src/abi/neg_risk_exchange.rs")?;

    Ok(())
}
