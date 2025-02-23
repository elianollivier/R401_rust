
use clap::Parser;
use clap::ValueEnum;


#[derive(Parser,Debug)]
pub struct Configuration {
    #[arg(short = 'c', long = "candidates", num_args = 1..)]
    pub candidates: Vec<String>,

    #[arg(long, value_enum, default_value_t = StorageType::Memory)]
    pub storage: StorageType,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum StorageType {
    File,
    Memory,
}