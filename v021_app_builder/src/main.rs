use anyhow::Result;
use clap::Parser;
use configuration::Configuration;
use app_builder::run_app;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = Configuration::parse();
    run_app(configuration).await
}
