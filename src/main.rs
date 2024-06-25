use std::time::Instant;
use structopt::StructOpt;

mod config;
mod error;
mod file_processing;
mod output;

use config::{determine_output_file, load_config, merge_ignore_patterns, print_verbose_info, Opt};
use error::CombinerError;
use file_processing::{process_files, ProcessingResult};
use output::{print_skipped_files, print_table};

const DEFAULT_OUTPUT_PREFIX: &str = "combiner_";

fn main() -> Result<(), CombinerError> {
    env_logger::init();
    let start_time = Instant::now();
    let opt = Opt::from_args();

    // Load configuration
    let config = load_config(&opt)?;
    let ignore_patterns = merge_ignore_patterns(&opt.ignore_patterns, &config.ignore_patterns);

    // Determine output file
    let output_file = determine_output_file(&opt, &config)?;

    // Print verbose information if enabled
    if opt.verbose {
        print_verbose_info(&opt, &output_file, &ignore_patterns, &config);
    }

    // Process files
    let ProcessingResult {
        files_processed,
        total_tokens,
        file_stats,
        skipped_files,
    } = process_files(&opt, &output_file, &ignore_patterns, &config)?;

    let processing_time = start_time.elapsed();

    // Print results
    print_table(
        files_processed,
        total_tokens,
        &output_file,
        &file_stats,
        processing_time,
        &config.tokenization_method,
        skipped_files.len(),
        ignore_patterns.len(),
    );

    // Print skipped files
    print_skipped_files(&skipped_files);

    Ok(())
}
