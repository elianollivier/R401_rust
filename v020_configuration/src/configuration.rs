use clap::Parser;

#[derive(Parser,Debug)]
pub struct Configuration {
    #[arg(short = 'c', long = "candidates")]
    pub candidates: Vec<String>,
}