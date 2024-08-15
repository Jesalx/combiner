use anyhow::{Context, Result};
use ignore::Walk;
use prettytable::{row, Table};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tiktoken_rs::{cl100k_base, o200k_base, p50k_base, p50k_edit, r50k_base};

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

/// Returns the appropriate BPE based on the tokenizer name.
///
/// # Arguments
/// * `tokenizer` - A string slice that holds the name of the tokenizer.
///
/// # Returns
/// Returns a `CoreBPE` instance for the specified tokenizer.
pub fn get_bpe(tokenizer: &str) -> tiktoken_rs::CoreBPE {
    match tokenizer {
        "o200k_base" => o200k_base().expect("Failed to load o200k_base tokenizer"),
        "cl100k_base" => cl100k_base().expect("Failed to load cl100k_base tokenizer"),
        "p50k_base" => p50k_base().expect("Failed to load p50k_base tokenizer"),
        "p50k_edit" => p50k_edit().expect("Failed to load p50k_edit tokenizer"),
        "r50k_base" => r50k_base().expect("Failed to load r50k_base tokenizer"),
        _ => cl100k_base().expect("Failed to load default cl100k_base tokenizer"),
    }
}

/// Combines files from the specified directory into a single output file.
///
/// # Arguments
///
/// * `directory` - A string slice that holds the path to the directory to process.
/// * `output` - A string slice that holds the path to the output file.
/// * `tokenizer` - A string slice that specifies the tokenizer to use.
///
/// # Returns
///
/// Returns a `Result` containing the `Statistics` of the operation if successful.
pub fn combine_files(directory: &str, output: &str, tokenizer: &str) -> Result<Statistics> {
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

    let bpe = get_bpe(tokenizer);
    let mut stats = Statistics {
        files_processed: 0,
        files_skipped: 0,
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
            match process_file(path, &mut output_file, &bpe) {
                Ok((token_count, file_content)) => {
                    stats.total_tokens += token_count;
                    if token_count > stats.max_tokens {
                        stats.max_tokens = token_count;
                        stats.max_tokens_file = path.display().to_string();
                    }

                    writeln!(output_file, "--- File: {} ---", path.display())
                        .context("Failed to write file name to output")?;
                    write!(output_file, "{}", file_content)
                        .context("Failed to write file contents to output")?;
                    writeln!(output_file).context("Failed to write newline to output")?;
                    stats.files_processed += 1;
                }
                Err(e) => {
                    eprintln!("Skipped file {}: {}", path.display(), e);
                    stats.files_skipped += 1;
                }
            }
        }
    }

    stats.processing_time = start_time.elapsed();
    Ok(stats)
}

/// Processes a single file, reading its contents and counting tokens.
///
/// # Arguments
///
/// * `path` - A reference to the path of the file to process.
/// * `_output_file` - A mutable reference to the output file (unused in this function).
/// * `bpe` - A reference to the CoreBPE tokenizer.
///
/// # Returns
///
/// Returns a `Result` containing a tuple of the token count and file content if successful.
fn process_file(
    path: &Path,
    _output_file: &mut File,
    bpe: &tiktoken_rs::CoreBPE,
) -> Result<(usize, String)> {
    let mut file = File::open(path).context("Failed to open input file")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Failed to read input file as UTF-8")?;

    let tokens = bpe.encode_with_special_tokens(&contents);
    let token_count = tokens.len();

    Ok((token_count, contents))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_combine_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();
        let tokenizer = "cl100k_base";

        // Create test files
        fs::write(dir_path.join("file1.txt"), "Content of file 1")?;
        fs::write(dir_path.join("file2.txt"), "Content of file 2")?;

        // Create a subdirectory with a file
        fs::create_dir(dir_path.join("subdir"))?;
        fs::write(
            dir_path.join("subdir").join("file3.txt"),
            "Content of file 3",
        )?;

        // Create a file with invalid UTF-8
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];
        fs::write(dir_path.join("invalid_utf8.bin"), invalid_utf8)?;

        // Combine files
        let output_file = dir_path.join("output.txt");
        let stats = combine_files(
            dir_path.to_str().unwrap(),
            output_file.to_str().unwrap(),
            tokenizer,
        )?;

        // Read the combined output
        let combined_content = fs::read_to_string(&output_file)?;

        // Check if all valid files are included in the output
        assert!(combined_content.contains("--- File:"));
        assert!(combined_content.contains("Content of file 1"));
        assert!(combined_content.contains("Content of file 2"));
        assert!(combined_content.contains("Content of file 3"));

        // Check statistics
        assert_eq!(stats.files_processed, 3);
        assert_eq!(stats.files_skipped, 1);
        assert_eq!(stats.directories_visited, 2); // Root directory + 1 subdirectory
        assert!(stats.total_tokens > 0);
        assert!(stats.processing_time > Duration::default());
        assert_eq!(stats.output_file, output_file.display().to_string());

        Ok(())
    }
}
