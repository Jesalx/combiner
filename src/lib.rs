use anyhow::{Context, Result};
use ignore::Walk;
use prettytable::{row, Table};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tiktoken_rs::cl100k_base;

pub struct Statistics {
    files_processed: usize,
    directories_visited: usize,
    total_tokens: usize,
    max_tokens: usize,
    max_tokens_file: String,
    processing_time: Duration,
    output_file: String,
}

pub fn combine_files(directory: &str, output: &str) -> Result<Statistics> {
    let start_time = Instant::now();

    let dir_path = Path::new(directory);
    let output_path = if dir_path.is_relative() {
        PathBuf::from(".").join(output)
    } else {
        PathBuf::from(output)
    };

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)
        .context("Failed to create output file")?;

    let bpe = cl100k_base().unwrap();
    let mut stats = Statistics {
        files_processed: 0,
        directories_visited: 1, // Start with 1 to count the root directory
        total_tokens: 0,
        max_tokens: 0,
        max_tokens_file: String::new(),
        processing_time: Duration::default(),
        output_file: output_path.display().to_string(),
    };

    for entry in Walk::new(directory) {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() && path != dir_path {
            stats.directories_visited += 1;
        } else if path.is_file() && path != output_path {
            stats.files_processed += 1;
            let mut file = File::open(path).context("Failed to open input file")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .context("Failed to read input file")?;

            let tokens = bpe.encode_with_special_tokens(&contents);
            let token_count = tokens.len();
            stats.total_tokens += token_count;

            if token_count > stats.max_tokens {
                stats.max_tokens = token_count;
                stats.max_tokens_file = path.display().to_string();
            }

            writeln!(output_file, "--- File: {} ---", path.display())
                .context("Failed to write file name to output")?;
            write!(output_file, "{}", contents)
                .context("Failed to write file contents to output")?;
            writeln!(output_file).context("Failed to write newline to output")?;
        }
    }

    stats.processing_time = start_time.elapsed();

    Ok(stats)
}

pub fn print_statistics(stats: &Statistics) {
    let mut table = Table::new();
    table.add_row(row!["Statistic", "Value"]);
    table.add_row(row!["Output File", &stats.output_file]);
    table.add_row(row!["Files Processed", stats.files_processed]);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_combine_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();

        // Create test files
        fs::write(dir_path.join("file1.txt"), "Content of file 1")?;
        fs::write(dir_path.join("file2.txt"), "Content of file 2")?;

        // Create a subdirectory with a file
        fs::create_dir(dir_path.join("subdir"))?;
        fs::write(
            dir_path.join("subdir").join("file3.txt"),
            "Content of file 3",
        )?;

        // Combine files
        let output_file = dir_path.join("output.txt");
        let stats = combine_files(dir_path.to_str().unwrap(), output_file.to_str().unwrap())?;

        // Read the combined output
        let combined_content = fs::read_to_string(&output_file)?;

        // Check if all files are included in the output
        assert!(combined_content.contains("--- File:"));
        assert!(combined_content.contains("Content of file 1"));
        assert!(combined_content.contains("Content of file 2"));
        assert!(combined_content.contains("Content of file 3"));

        // Check statistics
        assert_eq!(stats.files_processed, 3);
        assert_eq!(stats.directories_visited, 2); // Root directory + 1 subdirectory
        assert!(stats.total_tokens > 0);
        assert!(stats.processing_time > Duration::default());
        assert_eq!(stats.output_file, output_file.display().to_string());

        Ok(())
    }
}

