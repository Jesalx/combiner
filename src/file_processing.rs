use log::{info, warn};
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tiktoken_rs::CoreBPE;
use walkdir::WalkDir;

use crate::config::{Config, TokenizationMethod};
use crate::error::CombinerError;

pub struct ProcessingResult {
    pub files_processed: usize,
    pub total_tokens: usize,
    pub file_stats: Vec<(String, usize, u64)>,
    pub skipped_files: Vec<(String, String)>,
}

pub fn process_files(
    opt: &crate::config::Opt,
    output_file: &Path,
    ignore_patterns: &[String],
    config: &Config,
) -> Result<ProcessingResult, CombinerError> {
    let output = Arc::new(Mutex::new(BufWriter::new(
        File::create(output_file).map_err(CombinerError::Io)?,
    )));
    let bpe = Arc::new(get_tokenizer(&config.tokenization_method)?);

    let files: Vec<_> = WalkDir::new(&opt.input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            let path = entry.path();
            entry.file_type().is_file()
                && is_text_file(path)
                && !should_ignore(path, ignore_patterns)
                && should_include(path, &config.include_patterns)
        })
        .collect();

    let (file_stats, skipped_files): (Vec<_>, Vec<_>) = files
        .par_iter()
        .map(|entry| {
            let path = entry.path();
            if opt.verbose {
                info!("Processing file: {:?}", path);
            }
            match process_file(path, &output, &bpe) {
                Ok((file_tokens, file_size)) => {
                    Ok((path.to_string_lossy().into_owned(), file_tokens, file_size))
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    warn!("Failed to process file: {:?} - Error: {}", path, error_msg);
                    Err((path.to_string_lossy().into_owned(), error_msg))
                }
            }
        })
        .partition(Result::is_ok);

    let file_stats: Vec<_> = file_stats.into_iter().map(Result::unwrap).collect();
    let skipped_files: Vec<_> = skipped_files.into_iter().map(Result::unwrap_err).collect();

    let files_processed = file_stats.len();
    let total_tokens: usize = file_stats.iter().map(|(_, tokens, _)| tokens).sum();

    Ok(ProcessingResult {
        files_processed,
        total_tokens,
        file_stats,
        skipped_files,
    })
}

fn get_tokenizer(method: &TokenizationMethod) -> Result<CoreBPE, CombinerError> {
    let result = match method {
        TokenizationMethod::O200kBase => tiktoken_rs::o200k_base(),
        TokenizationMethod::Cl100kBase => tiktoken_rs::cl100k_base(),
        TokenizationMethod::P50kBase => tiktoken_rs::p50k_base(),
        TokenizationMethod::P50kEdit => tiktoken_rs::p50k_edit(),
        TokenizationMethod::R50kBase => tiktoken_rs::r50k_base(),
    };
    result.map_err(|e| CombinerError::Tokenization(e.to_string()))
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

fn should_include(path: &Path, include_patterns: &[String]) -> bool {
    include_patterns.is_empty()
        || include_patterns
            .iter()
            .any(|pattern| path.to_str().map(|s| s.contains(pattern)).unwrap_or(false))
}

fn process_file(
    path: &Path,
    output: &Arc<Mutex<BufWriter<File>>>,
    bpe: &Arc<CoreBPE>,
) -> Result<(usize, u64), CombinerError> {
    let mut output = output
        .lock()
        .map_err(|_| CombinerError::Unknown("Failed to acquire lock".to_string()))?;
    writeln!(output, "File: {:?}", path).map_err(CombinerError::Io)?;
    writeln!(output, "{}", "-".repeat(80)).map_err(CombinerError::Io)?;

    let content = fs::read_to_string(path).map_err(|e| CombinerError::FileProcessing {
        path: path.to_path_buf(),
        source: e.into(),
    })?;

    write!(output, "{}", content).map_err(CombinerError::Io)?;
    writeln!(output, "{}", "-".repeat(80)).map_err(CombinerError::Io)?;

    let tokens = bpe.encode_ordinary(&content);
    let file_size = fs::metadata(path).map_err(CombinerError::Io)?.len();
    Ok((tokens.len(), file_size))
}

