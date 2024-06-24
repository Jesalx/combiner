use prettytable::{row, Table};
use std::path::Path;
use std::time::Duration;

pub const TOP_FILES_TO_SHOW: usize = 10;

pub fn print_table(
    files_processed: usize,
    total_tokens: usize,
    output_file: &Path,
    file_stats: &[(String, usize, u64)],
    processing_time: Duration,
) {
    let mut table = Table::new();
    table.add_row(row!["Statistic", "Value"]);
    table.add_row(row!["Files Processed", files_processed]);
    table.add_row(row!["Total Tokens", total_tokens]);
    table.add_row(row!["Output File", output_file.to_string_lossy()]);
    table.add_row(row!["Processing Time", format!("{:.2?}", processing_time)]);
    table.printstd();

    let mut details_table = Table::new();
    details_table.add_row(row!["File", "Tokens", "Size (bytes)"]);

    // Sort file_stats by token count (descending) and take top N
    let mut sorted_stats = file_stats.to_vec();
    sorted_stats.sort_by(|a, b| b.1.cmp(&a.1));
    for (file, tokens, size) in sorted_stats.iter().take(TOP_FILES_TO_SHOW) {
        details_table.add_row(row![file, tokens, size]);
    }
    println!("\nTop {} Files by Token Count:", details_table.len());
    details_table.printstd();
}

pub fn print_skipped_files(skipped_files: &[(String, String)]) {
    println!("\nSkipped Files:");
    let mut skipped_table = Table::new();
    skipped_table.add_row(row!["File", "Reason"]);
    for (file, reason) in skipped_files {
        skipped_table.add_row(row![file, reason]);
    }
    skipped_table.printstd();
}
