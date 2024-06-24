use anyhow::Result;
use std::time::Instant;
use structopt::StructOpt;

mod config;
mod file_processing;
mod output;

use config::{determine_output_file, load_config, merge_ignore_patterns, print_verbose_info, Opt};
use file_processing::process_files;
use output::{print_skipped_files, print_table};

const DEFAULT_OUTPUT_PREFIX: &str = "combiner_";

fn main() -> Result<()> {
    let start_time = Instant::now();
    let mut opt = Opt::from_args();

    // Load configuration
    let config = load_config(&mut opt)?;
    let mut ignore_patterns = merge_ignore_patterns(&opt.ignore_patterns, &config.ignore_patterns);

    // Ensure 'target' is in ignore patterns
    if !ignore_patterns.contains(&"target".to_string()) {
        ignore_patterns.push("target".to_string());
    }

    // Determine output file
    let output_file = determine_output_file(&mut opt, &config)?;

    // Add output and config files to ignore patterns
    ignore_patterns.push(output_file.to_string_lossy().into_owned());
    if let Some(config_file) = &opt.config_file {
        ignore_patterns.push(config_file.to_string_lossy().into_owned());
    }

    // Print verbose information if enabled
    print_verbose_info(&opt, &output_file, &ignore_patterns, &config);

    // Process files
    let (files_processed, total_tokens, file_stats, skipped_files) =
        process_files(&opt, &output_file, &ignore_patterns, &config)?;

    let processing_time = start_time.elapsed();

    // Calculate files failed and ignored
    let files_failed = skipped_files.len();
    let total_files = files_processed + files_failed + ignore_patterns.len();
    let files_ignored = total_files - files_processed - files_failed;

    // Print results
    print_table(
        files_processed,
        total_tokens,
        &output_file,
        &file_stats,
        processing_time,
        config
            .tokenization_method
            .as_ref()
            .unwrap_or(&opt.tokenization_method),
        files_failed,
        files_ignored,
    );

    // Print skipped files
    print_skipped_files(&skipped_files);

    Ok(())
}

