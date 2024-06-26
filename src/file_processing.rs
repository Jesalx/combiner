use anyhow::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::tokenization::Tokenizer;

#[derive(Debug)]
pub struct FileContents {
    pub path: PathBuf,
    pub content: String,
    token_count: OnceLock<usize>,
}

impl FileContents {
    pub fn new(path: PathBuf, content: String) -> Self {
        Self {
            path,
            content,
            token_count: OnceLock::new(),
        }
    }

    pub fn count_tokens(&self, tokenizer: &Tokenizer) -> usize {
        self.token_count
            .get_or_init(|| tokenizer.count_tokens(&self.content))
            .clone()
    }
}

#[derive(Debug)]
pub struct InvalidFile {
    pub path: PathBuf,
    pub reason: String,
}

#[derive(Debug)]
pub struct DirectoryContents {
    pub valid_files: Vec<FileContents>,
    pub invalid_files: Vec<InvalidFile>,
}

pub fn load_directory_contents(
    dir: &Path,
    ignore_patterns: Vec<String>,
) -> Result<DirectoryContents> {
    let mut valid_files = Vec::new();
    let mut invalid_files = Vec::new();

    let walker = WalkBuilder::new(dir)
        .hidden(false)
        .ignore(true)
        .git_ignore(true)
        .filter_entry(move |entry| {
            let path = entry.path();
            !ignore_patterns
                .iter()
                .any(|pattern| path.to_str().map(|p| p.contains(pattern)).unwrap_or(false))
        })
        .build();

    for entry in walker.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            match std::fs::read_to_string(path) {
                Ok(content) => valid_files.push(FileContents::new(path.to_path_buf(), content)),
                Err(e) => invalid_files.push(InvalidFile {
                    path: path.to_path_buf(),
                    reason: format!("Failed to read file: {}", e),
                }),
            }
        }
    }

    Ok(DirectoryContents {
        valid_files,
        invalid_files,
    })
}
