use crate::EntryType::*;
use clap::{Parser, ValueEnum};
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Eq, PartialEq, ValueEnum)]
pub enum EntryType {
    #[value(name = "d")]
    Dir,
    #[value(name = "f")]
    File,
    #[value(name = "l")]
    Link,
}

#[derive(Parser, Debug)]
#[command(
    name = "findr",
    version = "0.1.0",
    author = "shinb09",
    about = "Rust find"
)]
pub struct Args {
    #[arg(value_name = "PATH", help = "Path to list", default_value = ".", num_args = 1..)]
    pub path: Vec<String>,
    #[arg(short = 't', long = "type", value_name = "TYPE", help = "Entry type to list", num_args = 1..)]
    pub entry_types: Vec<EntryType>,
    #[arg(short = 'n', long = "name", value_name = "PATTERN", help = "Regex pattern to filter entries", num_args = 1..)]
    pub names: Vec<Regex>,
}

pub fn get_args() -> MyResult<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> MyResult<()> {
    println!("{:#?}", args);
    Ok(())
}
