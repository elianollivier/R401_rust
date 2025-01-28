use anyhow::Result;
use clap::Parser;
use v020_configuration::configuration::Configuration; // ajustez le chemin selon votre module

#[tokio::main]
async fn main() -> Result<()> {
    let mut configuration = Configuration::parse();

    configuration.candidates.push("blanc".to_string());
    configuration.candidates.push("nul".to_string());

    println!("Candidats : {:?}", configuration.candidates);

    Ok(())
}
