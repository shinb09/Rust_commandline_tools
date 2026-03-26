use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(name = "catr", version = "0.1.0", author = "shinb09", about = "Rust cat")]
pub struct Config {
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    files : Vec<String>,
    #[arg(short = 'n', long = "number", help = "Number lines", conflicts_with = "number_nonblank_lines")]
    number_lines : bool,
    #[arg(short = 'b', long = "number-nonblank", help = "Number non-blank lines")]
    number_nonblank_lines : bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    let mut line_num = 0;
    for filename in &config.files {
        match open(&filename) {
            Err(e) => eprintln!("Error opening {}: {}", filename, e),
            Ok(file_reader) => {
                for line in file_reader.lines() {
                    let line = line?;
                    match (config.number_lines, config.number_nonblank_lines) {
                        (true, _) => {
                            line_num += 1;
                            println!("{:>6}\t{}", line_num, line);
                        }
                        (_, true) if !line.is_empty() => {
                            line_num += 1;
                            println!("{:>6}\t{}", line_num, line);
                        }
                        (_, true) => println!(),
                        _ => println!("{}", line),
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}