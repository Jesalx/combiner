use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CombinerError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to process file: {path}")]
    FileProcessing {
        path: PathBuf,
        source: anyhow::Error,
    },

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Tokenization error: {0}")]
    Tokenization(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

