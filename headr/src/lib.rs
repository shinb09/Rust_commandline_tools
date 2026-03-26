use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(
    name = "headr",
    version = "0.1.0",
    author = "shinb09",
    about = "Rust head"
)]
pub struct Config {
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    files: Vec<String>,
    #[arg(
        short = 'n',
        long = "lines",
        help = "Number of lines to display",
        default_value_t = 10,
        conflicts_with = "num_bytes"
    )]
    num_lines: usize,
    #[arg(
        short = 'c',
        long = "bytes",
        help = "Number of bytes to display",
    )]
    num_bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    // for filename in &config.files {
    //     let filename = filename.as_str();
    //     println!("==> {} <==", filename);
    // }
    Ok(())
}
