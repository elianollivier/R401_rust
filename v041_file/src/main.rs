use anyhow::Result;
use clap::Parser;
use v041_file::configuration::Configuration;
use v041_file::app_builder::run_app;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = Configuration::parse();
    run_app(configuration).await
}
