use prettytable::{row, Table};
use std::path::Path;
use std::time::Duration;

use crate::config::TokenizationMethod;

pub const TOP_FILES_TO_SHOW: usize = 10;

pub fn print_table(
    files_processed: usize,
    total_tokens: usize,
    output_file: &Path,
    file_stats: &[(String, usize, u64)],
    processing_time: Duration,
    tokenization_method: &TokenizationMethod,
    files_failed: usize,
    files_ignored: usize,
) {
    let mut table = Table::new();
    table.add_row(row!["Statistic", "Value"]);
    table.add_row(row!["Files Processed", files_processed]);
    table.add_row(row!["Files Failed", files_failed]);
    table.add_row(row!["Files Ignored", files_ignored]);
    table.add_row(row!["Total Tokens", total_tokens]);
    table.add_row(row!["Tokenization Method", tokenization_method.to_string()]);
    table.add_row(row!["Output File", output_file.to_string_lossy()]);
    table.add_row(row!["Processing Time", format!("{:.2?}", processing_time)]);

    let total_size: u64 = file_stats.iter().map(|(_, _, size)| size).sum();
    let avg_tokens = if files_processed > 0 {
        total_tokens as f64 / files_processed as f64
    } else {
        0.0
    };
    let avg_size = if files_processed > 0 {
        total_size as f64 / files_processed as f64
    } else {
        0.0
    };

    table.add_row(row![
        "Total File Size",
        format!("{:.2} MB", total_size as f64 / 1_048_576.0)
    ]);
    table.add_row(row![
        "Average Tokens per File",
        format!("{:.2}", avg_tokens)
    ]);
    table.add_row(row![
        "Average File Size",
        format!("{:.2} KB", avg_size / 1024.0)
    ]);

    table.printstd();

    let mut details_table = Table::new();
    details_table.add_row(row!["File", "Tokens", "Size (bytes)"]);

    // Sort file_stats by token count (descending) and take top N
    let mut sorted_stats = file_stats.to_vec();
    sorted_stats.sort_by(|a, b| b.1.cmp(&a.1));
    for (file, tokens, size) in sorted_stats.iter().take(TOP_FILES_TO_SHOW) {
        details_table.add_row(row![file, tokens, size]);
    }
    println!("\nTop {} Files by Token Count:", TOP_FILES_TO_SHOW);
    details_table.printstd();
}

pub fn print_skipped_files(skipped_files: &[(String, String)]) {
    if skipped_files.is_empty() {
        return;
    }

    println!("\nSkipped Files:");
    let mut skipped_table = Table::new();
    skipped_table.add_row(row!["File", "Reason"]);
    for (file, reason) in skipped_files {
        skipped_table.add_row(row![file, reason]);
    }
    skipped_table.printstd();
}

