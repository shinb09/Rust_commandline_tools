use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(
    name = "uniqr",
    version = "0.1.0",
    author = "shinb09",
    about = "Rust uniq"
)]
pub struct Args {
    #[arg(value_name = "IN_FILE", help = "Input file", default_value = "-")]
    pub in_file: String,
    #[arg(
        short = 'o',
        long = "outfile",
        value_name = "OUT_FILE",
        help = "Output file"
    )]
    pub out_file: Option<String>,
    #[arg(short = 'c', long = "count", help = "Show counts")]
    pub count: bool,
}

pub fn get_args() -> MyResult<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> MyResult<()> {
    let mut reader = open(&args.in_file)?;
    let mut writer = open_writer(args.out_file.as_deref())?;
    let mut line = String::new();
    let mut prev_line = String::new();
    let mut count = 0usize;

    while reader.read_line(&mut line)? != 0 {
        if prev_line.trim_end() != line.trim_end() {
            if count > 0 {
                write_group(&mut writer, &prev_line, count, args.count)?;
            }
            prev_line = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    if count > 0 {
        write_group(&mut writer, &prev_line, count, args.count)?;
    }
    Ok(())
}

fn write_group(writer: &mut dyn Write, line: &str, count: usize, show_count: bool) -> MyResult<()> {
    if show_count {
        write!(writer, "{:>7} ", count)?;
    }
    if line.ends_with('\n') {
        write!(writer, "{}", line)?;
    } else {
        writeln!(writer, "{}", line)?;
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    Ok(match filename {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{filename}: {e}"))?,
        )),
    })
}

fn open_writer(filename: Option<&str>) -> MyResult<Box<dyn Write>> {
    Ok(match filename {
        Some(path) => Box::new(BufWriter::new(File::create(path)?)),
        None => Box::new(BufWriter::new(io::stdout())),
    })
}
