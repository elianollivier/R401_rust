use anyhow::Result;
use clap::Parser;
use v050_use_cases::configuration::Configuration;
use v050_use_cases::app_builder::run_app;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = Configuration::parse();
    run_app(configuration).await
}
