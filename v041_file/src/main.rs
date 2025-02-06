use anyhow::Result;
use clap::Parser;
use v040_memory::configuration::Configuration;
use v040_memory::app_builder::run_app;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = Configuration::parse();
    run_app(configuration).await
}
