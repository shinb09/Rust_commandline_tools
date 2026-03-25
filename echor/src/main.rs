use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "echor", version = "0.1.0", author = "shinb09", about = "Rust echo")]
struct Args {
    #[arg(value_name = "TEXT", help = "Input text", required = true, num_args(1..))]
    text: Vec<String>,
    #[arg(short = 'n', long = "no-newline", help = "Do not print newline")]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    print!("{}{}", args.text.join(" "), if args.omit_newline { "" } else { "\n" });
}
