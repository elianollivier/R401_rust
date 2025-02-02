use anyhow::Result;
use clap::Parser;
use v030_domain::configuration::Configuration;
use v030_domain::app_builder::run_app;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = Configuration::parse();
    run_app(configuration).await
}
