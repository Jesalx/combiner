use anyhow::{Context, Result};
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tiktoken_rs::p50k_base;
use walkdir::WalkDir;

use crate::config::Config;

pub fn process_files(
    opt: &crate::config::Opt,
    output_file: &Path,
    ignore_patterns: &[String],
    config: &Config,
) -> Result<(
    usize,
    usize,
    Vec<(String, usize, u64)>,
    Vec<(String, String)>,
)> {
    let output = Arc::new(Mutex::new(BufWriter::new(File::create(output_file)?)));
    let bpe = Arc::new(p50k_base()?);

    let file_stats = Arc::new(Mutex::new(Vec::new()));
    let skipped_files = Arc::new(Mutex::new(Vec::new()));

    let files: Vec<_> = WalkDir::new(&opt.input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    let files_processed = files
        .par_iter()
        .filter(|entry| {
            let path = entry.path();
            if entry.file_type().is_file() {
                if is_text_file(path)
                    && !should_ignore(path, ignore_patterns)
                    && should_include(path, &config.include_patterns)
                {
                    true
                } else {
                    if opt.verbose {
                        print_skip_reason(path, ignore_patterns, &config.include_patterns);
                    }
                    false
                }
            } else {
                false
            }
        })
        .count();

    let total_tokens: usize = files
        .par_iter()
        .filter(|entry| {
            let path = entry.path();
            path.is_file()
                && is_text_file(path)
                && !should_ignore(path, ignore_patterns)
                && should_include(path, &config.include_patterns)
        })
        .map(|entry| {
            let path = entry.path();
            if opt.verbose {
                println!("Processing file: {:?}", path);
            }
            (
                path.to_string_lossy().into_owned(),
                process_file(path, &output, &bpe),
            )
        })
        .filter_map(|(path, result)| match result {
            Ok((file_tokens, file_size)) => {
                file_stats
                    .lock()
                    .unwrap()
                    .push((path.clone(), file_tokens, file_size));
                Some(file_tokens)
            }
            Err(e) => {
                skipped_files
                    .lock()
                    .unwrap()
                    .push((path.clone(), e.to_string()));
                if opt.verbose {
                    println!("Skipped file due to error: {:?} - {}", path, e);
                }
                None
            }
        })
        .sum();

    Ok((
        files_processed,
        total_tokens,
        Arc::try_unwrap(file_stats).unwrap().into_inner().unwrap(),
        Arc::try_unwrap(skipped_files)
            .unwrap()
            .into_inner()
            .unwrap(),
    ))
}

fn is_text_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            matches!(
                ext,
                "txt"
                    | "md"
                    | "rs"
                    | "toml"
                    | "json"
                    | "yaml"
                    | "yml"
                    | "py"
                    | "js"
                    | "ts"
                    | "html"
                    | "css"
                    | "sh"
                    | "bash"
                    | "xml"
                    | "svg"
                    | "cpp"
                    | "c"
                    | "h"
                    | "hpp"
            )
        })
        .unwrap_or(false)
}

fn should_ignore(path: &Path, ignore_patterns: &[String]) -> bool {
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    ignore_patterns
        .iter()
        .any(|pattern| path.to_str().map(|s| s.contains(pattern)).unwrap_or(false))
        || file_name.starts_with(crate::DEFAULT_OUTPUT_PREFIX)
}

fn should_include(path: &Path, include_patterns: &Option<Vec<String>>) -> bool {
    include_patterns
        .as_ref()
        .map(|patterns| {
            patterns.is_empty()
                || patterns
                    .iter()
                    .any(|pattern| path.to_str().map(|s| s.contains(pattern)).unwrap_or(false))
        })
        .unwrap_or(true)
}

fn process_file(
    path: &Path,
    output: &Arc<Mutex<BufWriter<File>>>,
    bpe: &Arc<tiktoken_rs::CoreBPE>,
) -> Result<(usize, u64)> {
    let mut output = output.lock().unwrap();
    write!(output, "File: {:?}\n", path)?;
    writeln!(output, "{}", "-".repeat(80))?;

    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read file: {:?}", path))?;

    write!(output, "{}", content)?;
    writeln!(output, "{}", "-".repeat(80))?;

    let tokens = bpe.encode_ordinary(&content);
    let file_size = fs::metadata(path)?.len();
    Ok((tokens.len(), file_size))
}

pub fn print_skip_reason(
    path: &Path,
    ignore_patterns: &[String],
    include_patterns: &Option<Vec<String>>,
) {
    if !path.is_file() {
        println!("Skipping non-file: {:?}", path);
    } else if !is_text_file(path) {
        println!("Skipping non-text file: {:?}", path);
    } else if should_ignore(path, ignore_patterns) {
        println!("Skipping ignored file: {:?}", path);
    } else if !should_include(path, include_patterns) {
        println!("Skipping non-included file: {:?}", path);
    }
}
