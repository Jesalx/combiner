mod tokenization;

use clap::Parser;
use std::path::PathBuf;
use tokenization::TokenizationMethod;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "PATTERN")]
    ignore: Vec<String>,

    #[arg(short = 'd', long, value_name = "DIR", default_value = ".")]
    input_dir: PathBuf,

    #[arg(short, long, value_name = "FILE", default_value = "combiner.txt")]
    output_file: PathBuf,

    #[arg(short, long, default_value_t = TokenizationMethod::Code)]
    tokenization_method: TokenizationMethod,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
