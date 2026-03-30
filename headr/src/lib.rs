use clap::Parser;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

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
        value_name = "LINES",
        help = "Number of lines to display",
        default_value_t = 10,
        allow_hyphen_values = true,
        conflicts_with = "num_bytes"
    )]
    num_lines: isize,
    #[arg(
        short = 'c',
        long = "bytes",
        value_name = "BYTES",
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
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut reader) => {
                if num_files > 1 {
                    println!("{}==> {} <==", if file_num > 0 { "\n" } else { "" }, filename);
                }

                if let Some(num_bytes) = config.num_bytes {
                    let bytes: Result<Vec<_>, _> = reader.bytes().take(num_bytes).collect();
                    print!("{}", String::from_utf8_lossy(&bytes?));
                } else if config.num_lines < 0 {
                    print_all_but_last_lines(&mut reader, config.num_lines.unsigned_abs())?;
                } else {
                    print_lines(&mut reader, config.num_lines as usize)?;
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

fn print_lines(reader: &mut dyn BufRead, num_lines: usize) -> MyResult<()> {
    let mut line_buf = String::new();
    for _ in 0..num_lines {
        let bytes = reader.read_line(&mut line_buf)?;
        if bytes == 0 {
            break;
        }
        print!("{}", line_buf);
        line_buf.clear();
    }
    Ok(())
}

fn print_all_but_last_lines(reader: &mut dyn BufRead, num_lines: usize) -> MyResult<()> {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf: VecDeque<String> = VecDeque::with_capacity(num_lines);

    let mut line_buf = String::new();
    while reader.read_line(&mut line_buf)? > 0 {
        if buf.len() == num_lines {
            let oldest = buf.pop_front().unwrap();
            write!(out, "{}", oldest)?;
        }
        buf.push_back(line_buf.clone());
        line_buf.clear();
    }
    Ok(())
}