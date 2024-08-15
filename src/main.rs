use combiner::{combine_files, print_statistics, CombinerConfig};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "combiner", about = "Combines files in a directory")]
struct Opt {
    /// Input directory to process
    #[structopt(short, long, default_value = ".")]
    directory: String,

    /// Output file path/name
    #[structopt(short, long, default_value = "combined_output.txt")]
    output: String,

    /// Tokenizer to use
    #[structopt(
        short,
        long,
        default_value = "p50k_base",
        possible_values = &["o200k_base", "cl100k_base", "p50k_base", "p50k_edit", "r50k_base"]
    )]
    tokenizer: String,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let config = CombinerConfig::new(opt.directory, opt.output, opt.tokenizer);
    let stats = combine_files(&config)?;
    print_statistics(&stats);
    Ok(())
}
