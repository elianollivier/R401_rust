use clap::Parser;

#[derive(Parser,Debug)]
pub struct Configuration {
    #[arg(short = 'c', long = "candidates", num_args = 1..)]
    pub candidates: Vec<String>,
}