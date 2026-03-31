use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Sum;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        let any_specified = args.lines || args.words || args.bytes || args.chars;
        Config {
            files: args.files,
            lines: if any_specified { args.lines } else { true },
            words: if any_specified { args.words } else { true },
            bytes: if any_specified { args.bytes } else { true },
            chars: args.chars,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FileInfo {
    pub num_lines: usize,
    pub num_words: usize,
    pub num_bytes: usize,
    pub num_chars: usize,
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

impl Sum for FileInfo {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            FileInfo {
                num_lines: 0,
                num_words: 0,
                num_bytes: 0,
                num_chars: 0,
            },
            |acc, info| FileInfo {
                num_lines: acc.num_lines + info.num_lines,
                num_words: acc.num_words + info.num_words,
                num_bytes: acc.num_bytes + info.num_bytes,
                num_chars: acc.num_chars + info.num_chars,
            },
        )
    }
}

pub fn get_args() -> MyResult<Config> {
    Ok(Args::parse().into())
}

pub fn run(config: Config) -> MyResult<()> {
    let file_infos: Vec<FileInfo> = config
        .files
        .iter()
        .filter_map(|file| {
            open(file)
                .and_then(|r| FileInfo::count(r))
                .map_err(|e| eprintln!("{file}: {e}"))
                .ok()
        })
        .collect();
    show_info(&file_infos, &config);
    if let Some(total) = calc_total(&file_infos) {
        show_info(
            &[total],
            &Config {
                files: vec!["total".to_string()],
                ..config
            },
        );
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn show_info(file_infos: &[FileInfo], config: &Config) {
    const MIN_WIDTH: usize = 4;

    type Extractor = fn(&FileInfo) -> usize;
    let col_width = |f: Extractor| {
        (file_infos
            .iter()
            .map(f)
            .max()
            .unwrap_or(0)
            .to_string()
            .len()
            + 1)
        .max(MIN_WIDTH)
    };

    let num_lines = col_width(|i| i.num_lines);
    let num_words = col_width(|i| i.num_words);
    let num_bytes = col_width(|i| i.num_bytes);
    let num_chars = col_width(|i| i.num_chars);

    for (info, file) in file_infos.iter().zip(&config.files) {
        let fields: &[(bool, usize, usize)] = &[
            (config.lines, info.num_lines, num_lines),
            (config.words, info.num_words, num_words),
            (config.bytes, info.num_bytes, num_bytes),
            (config.chars, info.num_chars, num_chars),
        ];
        let buf: String = fields
            .into_iter()
            .filter(|(enabled, ..)| *enabled)
            .map(|(_, val, width)| format!("{val:>width$}"))
            .collect();
        let file_name = if file == "-" { "" } else { file };
        println!("{buf} {file_name}");
    }
}

fn calc_total(file_infos: &[FileInfo]) -> Option<FileInfo> {
    if file_infos.len() > 1 {
        Some(file_infos.iter().cloned().sum())
    } else {
        None
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
