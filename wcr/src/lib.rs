use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(name = "wcr", version = "0.1.0", author = "shinb09", about = "Rust wc")]
struct Args {
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    files: Vec<String>,
    #[arg(short = 'l', long = "lines", help = "Show line count")]
    lines: bool,
    #[arg(short = 'w', long = "words", help = "Show word count")]
    words: bool,
    #[arg(
        short = 'c',
        long = "bytes",
        help = "Show byte count",
        conflicts_with = "chars"
    )]
    bytes: bool,
    #[arg(
        short = 'm',
        long = "chars",
        help = "Show character count",
        conflicts_with = "bytes"
    )]
    chars: bool,
}

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub lines: bool,
    pub words: bool,
    pub bytes: bool,
    pub chars: bool,
}

impl Config {
    fn from_args(args: Args) -> Self {
        let mut config = Config {
            files: args.files,
            lines: args.lines,
            words: args.words,
            bytes: args.bytes,
            chars: args.chars,
        };

        if [args.lines, args.words, args.bytes, args.chars]
            .iter()
            .all(|v| v == &false)
        {
            config.lines = true;
            config.words = true;
            config.bytes = true;
        }
        config
    }
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

impl FileInfo {
    pub fn count(file: impl BufRead) -> MyResult<Self> {
        let mut file = file;
        let mut num_lines = 0;
        let mut num_words = 0;
        let mut num_bytes = 0;
        let mut num_chars = 0;

        let mut line = String::new();
        while file.read_line(&mut line)? != 0 {
            num_lines += 1;
            num_words += line.split_whitespace().count();
            num_bytes += line.len();
            num_chars += line.chars().count();
            line.clear();
        }

        Ok(Self {
            num_lines,
            num_words,
            num_bytes,
            num_chars,
        })
    }
}

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn get_args() -> MyResult<Config> {
    let args = Args::parse();
    Ok(Config::from_args(args))
}

pub fn run(config: Config) -> MyResult<()> {
    for file in &config.files {
        match open(file) {
            Err(e) => eprintln!("{}: {}", file, e),
            Ok(mut _reader) => match FileInfo::count(_reader) {
                Err(e) => eprintln!("{}: {}", file, e),
                Ok(info) => {
                    println!(
                        "{:>8}{:>8}{:>8}{:>8}\t{}",
                        info.num_lines, info.num_words, info.num_bytes, info.num_chars, file
                    )
                }
            },
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[cfg(test)]
mod tests {
    use super::FileInfo;
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = FileInfo::count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
