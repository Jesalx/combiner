use crate::config::CombinerConfig;
use crate::statistics::Statistics;
use crate::tokenizer::get_bpe;
use anyhow::{Context, Result};
use ignore::Walk;
use rayon::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Struct to processing results of a file
#[derive(Debug)]
struct FileResult {
    path: PathBuf,
    content: String,
}

/// Combines files from the specified directory into a single output file.
///
/// # Arguments
///
/// * `config` - A reference to the `CombinerConfig` struct.
///
/// # Returns
///
/// Returns a `Result` containing the `Statistics` of the operation if successful.
pub fn combine_files(config: &CombinerConfig) -> Result<Statistics> {
    let start_time = Instant::now();

    let dir_path = Path::new(&config.directory);
    let output_path = if dir_path.is_relative() {
        PathBuf::from(".").join(&config.output)
    } else {
        PathBuf::from(&config.output)
    };

    let bpe = Arc::new(get_bpe(&config.tokenizer));
    let stats = Arc::new(Mutex::new(Statistics::new(
        output_path.display().to_string(),
    )));

    // Collect results in a vector
    let results: Vec<FileResult> = Walk::new(&config.directory)
        .par_bridge()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            if path.is_dir() && path != dir_path {
                let mut stats = stats.lock().unwrap();
                stats.increment_directories_visited();
                None
            } else if path.is_file() && path != output_path {
                match process_file(path, &bpe) {
                    Ok((token_count, content)) => {
                        let mut stats = stats.lock().unwrap();
                        stats.increment_processed_files();
                        stats.update_token_stats(token_count, path.display().to_string());
                        Some(FileResult {
                            path: path.to_path_buf(),
                            content,
                        })
                    }
                    Err(e) => {
                        eprintln!("Skipped file {}: {}", path.display(), e);
                        let mut stats = stats.lock().unwrap();
                        stats.increment_skipped_files();
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect();

    // Write results to the output file
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)
        .context("Failed to create output file")?;

    for result in results {
        writeln!(output_file, "--- File: {} ---", result.path.display())?;
        write!(output_file, "{}", result.content)?;
        writeln!(output_file)?;
    }

    let mut stats = Arc::try_unwrap(stats)
        .expect("Failed to unwrap Arc")
        .into_inner()
        .expect("Failed to unwrap Mutex");
    stats.processing_time = start_time.elapsed();

    Ok(stats)
}

/// Processes a single file, reading its contents and counting tokens.
///
/// # Arguments
///
/// * `path` - A reference to the path of the file to process.
/// * `bpe` - A reference to the CoreBPE tokenizer.
///
/// # Returns
///
/// Returns a `Result` containing a tuple of the token count and file content if successful.
fn process_file(path: &Path, bpe: &Arc<tiktoken_rs::CoreBPE>) -> Result<(usize, String)> {
    let mut file = File::open(path).context("Failed to open input file")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Failed to read input file as UTF-8")?;

    let tokens = bpe.encode_with_special_tokens(&contents);
    let token_count = tokens.len();

    Ok((token_count, contents))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::Duration;
    use tempfile::TempDir;

    #[test]
    fn test_combine_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();
        let tokenizer = "p50k_base";

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
        let config = CombinerConfig::new(
            dir_path.to_str().unwrap().to_string(),
            output_file.to_str().unwrap().to_string(),
            tokenizer.to_string(),
        );
        let stats = combine_files(&config)?;

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
        assert!(stats.total_tokens == 12); // Total tokens in above 3 strings is 12 for p50k_base
        assert!(stats.processing_time > Duration::default());
        assert_eq!(stats.output_file, output_file.display().to_string());

        Ok(())
    }
}
