
use clap::Parser;
use clap::ValueEnum;


#[derive(Parser,Debug)]
pub struct Configuration {
    #[arg(short = 'c', long = "candidates", num_args = 1..)]
    pub candidates: Vec<String>,

    #[arg(long, value_enum, default_value_t = StorageType::Memory)]
    pub storage: StorageType,

    #[arg(short='l', long="language", value_enum, default_value_t = Language::Fr)]
    pub language: Language,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum StorageType {
    File,
    Memory,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum Language {
    En,
    Fr,
}