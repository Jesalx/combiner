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

impl Statistics {
    /// Creates a new Statistics instance with default values.
    pub fn new(output_file: String) -> Self {
        Self {
            files_processed: 0,
            files_skipped: 0,
            directories_visited: 1, // Start with 1 to count the root directory
            total_tokens: 0,
            max_tokens: 0,
            max_tokens_file: String::new(),
            processing_time: Duration::default(),
            output_file,
        }
    }

    /// Increments the count of processed files.
    pub fn increment_processed_files(&mut self) {
        self.files_processed += 1;
    }

    /// Updates the statistics with information from a processed file.
    pub fn update_token_stats(&mut self, tokens: usize, file_path: String) {
        self.total_tokens += tokens;
        if tokens > self.max_tokens {
            self.max_tokens = tokens;
            self.max_tokens_file = file_path;
        }
    }

    /// Increments the count of skipped files.
    pub fn increment_skipped_files(&mut self) {
        self.files_skipped += 1;
    }

    /// Increments the count of visited directories.
    pub fn increment_directories_visited(&mut self) {
        self.directories_visited += 1;
    }

    /// Sets the processing time.
    pub fn set_processing_time(&mut self, duration: Duration) {
        self.processing_time = duration;
    }

    /// Prints the statistics in a formatted table.
    pub fn print(&self) {
        let mut table = Table::new();
        table.add_row(row!["Statistic", "Value"]);
        table.add_row(row!["Output File", &self.output_file]);
        table.add_row(row!["Files Processed", self.files_processed]);
        table.add_row(row!["Files Skipped", self.files_skipped]);
        table.add_row(row!["Directories Visited", self.directories_visited]);
        table.add_row(row!["Total Tokens", self.total_tokens]);
        table.add_row(row!["Max Tokens", self.max_tokens]);
        table.add_row(row!["File with Max Tokens", &self.max_tokens_file]);
        table.add_row(row![
            "Processing Time",
            format!("{:.2?}", self.processing_time)
        ]);
        table.printstd();
    }
}
