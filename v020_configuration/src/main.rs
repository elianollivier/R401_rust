use anyhow::Result;
use clap::Parser;
mod configuration;
use configuration::Configuration;


#[tokio::main]
async fn main() -> Result<()> {
    let mut configuration = Configuration::parse();

    configuration.candidates.push("blanc".to_string());
    configuration.candidates.push("nul".to_string());

    println!("Candidats : {:?}", configuration.candidates);

    Ok(())
}
