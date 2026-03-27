use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    num_lines: usize,
    num_bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("shinb09")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .multiple(true)
        )
        .arg(
            Arg::with_name("num_lines")
                .value_name("LINES")
                .short("n")
                .long("lines")
                .help("Number of lines to display")
                .default_value("10")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("num_bytes")
                .value_name("BYTES")
                .short("c")
                .long("bytes")
                .help("Number of bytes to display")
                .takes_value(true)
        )
        .get_matches();

    if matches.occurrences_of("num_lines") > 0 && matches.occurrences_of("num_bytes") > 0 {
        return Err("Cannot specify both '--lines <LINES>' and '--bytes <BYTES>'".into());
    }

    let lines = matches
        .value_of("num_lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("invalid value '{}' for '--lines <LINES>'", e))?;

    let bytes = matches
        .value_of("num_bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("invalid value '{}' for '--bytes <BYTES>'", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        num_lines: lines.unwrap(),
        num_bytes: bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(_) => println!("Opened: {}", filename),
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(num) if num > 0 => Ok(num),
        _ => Err(val.into()),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

#[test]
fn test_parse_positive_int() {
    // 3は正の整数なのでOK
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // 数字出ない文字列はエラー
    let res = parse_positive_int("abc");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "abc".to_string());

    // 0の場合もエラー
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());

    // 負の数もエラー
    let res = parse_positive_int("-5");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "-5".to_string());
}