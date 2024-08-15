use combiner::{combine_files, print_statistics};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "combiner", about = "Combines files in a directory")]
struct Opt {
    #[structopt(short, long, default_value = ".")]
    directory: String,

    #[structopt(short, long, default_value = "combined_output.txt")]
    output: String,

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
    let stats = combine_files(&opt.directory, &opt.output, &opt.tokenizer)?;
    print_statistics(&stats);
    Ok(())
}
