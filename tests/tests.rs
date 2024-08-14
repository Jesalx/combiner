use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_cli_combine_files() -> Result<(), Box<dyn std::error::Error>> {
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

    // Run the combiner command
    let output_file = dir_path.join("output.txt");
    let mut cmd = Command::cargo_bin("combiner")?;
    cmd.arg("--directory")
        .arg(dir_path)
        .arg("--output")
        .arg(&output_file);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Output File"))
        .stdout(predicate::str::contains("Files Processed"))
        .stdout(predicate::str::contains("Directories Visited"))
        .stdout(predicate::str::contains("Total Tokens"))
        .stdout(predicate::str::contains("Max Tokens"))
        .stdout(predicate::str::contains("Processing Time"));

    // Read the combined output
    let combined_content = fs::read_to_string(output_file)?;

    // Check if all files are included in the output
    assert!(combined_content.contains("--- File:"));
    assert!(combined_content.contains("Content of file 1"));
    assert!(combined_content.contains("Content of file 2"));
    assert!(combined_content.contains("Content of file 3"));

    Ok(())
}

