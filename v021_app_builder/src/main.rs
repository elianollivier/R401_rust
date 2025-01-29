use anyhow::Result;
use clap::Parser;
use v021_app_builder::configuration::Configuration;
use v021_app_builder::app_builder::run_app;

#[tokio::main]
async fn main() -> Result<()> {
    let mut configuration = Configuration::parse();

    configuration.candidates.push("blanc".to_string());
    configuration.candidates.push("nul".to_string());
    
    run_app(configuration).await
}
