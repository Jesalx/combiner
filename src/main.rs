use combiner::{combine_files, print_statistics};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "combiner", about = "Combines files in a directory")]
struct Opt {
    #[structopt(short, long, default_value = ".")]
    directory: String,

    #[structopt(short, long, default_value = "combined_output.txt")]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let stats = combine_files(&opt.directory, &opt.output)?;
    print_statistics(&stats);
    Ok(())
}
