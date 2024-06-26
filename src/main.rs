mod file_processing;
mod tokenization;

use anyhow::Result;
use clap::Parser;
use file_processing::load_directory_contents;
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

fn main() -> Result<()> {
    let mut args = Args::parse();

    // Add ".git" to the ignore list
    args.ignore.push(String::from(".git"));

    let tokenizer = tokenization::Tokenizer::new(args.tokenization_method)?;

    let contents = load_directory_contents(&args.input_dir, args.ignore.clone())?;
    println!("Valid files: {}", contents.valid_files.len());
    println!("Invalid files: {}", contents.invalid_files.len());
    for file in contents.valid_files {
        println!("{}", file.content)
    }
    for file in contents.invalid_files {
        println!("{}", file.path.display());
    }

    Ok(())
}
