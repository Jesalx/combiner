use prettytable::{row, Table};
use std::time::Duration;

/// Represents the statistics collected during the file combining process.
#[derive(Debug)]
pub struct Statistics {
    pub files_processed: usize,
    pub files_skipped: usize,
    pub directories_visited: usize,
    pub total_tokens: usize,
    pub max_tokens: usize,
    pub max_tokens_file: String,
    pub processing_time: Duration,
    pub output_file: String,
}

/// Prints the statistics of the file combining process in a formatted table.
///
/// # Arguments
///
/// * `stats` - A reference to the `Statistics` struct containing the data to print.
pub fn print_statistics(stats: &Statistics) {
    let mut table = Table::new();
    table.add_row(row!["Statistic", "Value"]);
    table.add_row(row!["Output File", &stats.output_file]);
    table.add_row(row!["Files Processed", stats.files_processed]);
    table.add_row(row!["Files Skipped", stats.files_skipped]);
    table.add_row(row!["Directories Visited", stats.directories_visited]);
    table.add_row(row!["Total Tokens", stats.total_tokens]);
    table.add_row(row!["Max Tokens", stats.max_tokens]);
    table.add_row(row!["File with Max Tokens", &stats.max_tokens_file]);
    table.add_row(row![
        "Processing Time",
        format!("{:.2?}", stats.processing_time)
    ]);
    table.printstd();
}

