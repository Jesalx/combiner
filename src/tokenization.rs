use anyhow::{anyhow, Result};
use clap::ValueEnum;
use std::fmt;
use std::sync::Arc;
use tiktoken_rs::{cl100k_base, o200k_base, p50k_base, p50k_edit, r50k_base, CoreBPE};

#[derive(Debug, Clone, ValueEnum)]
pub enum TokenizationMethod {
    Code,
    GPT4o,
    GPT4,
    GPT3,
    O200kBase,
    CL100kBase,
    P50kBase,
    P50kEdit,
    R50kBase,
}

impl fmt::Display for TokenizationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenizationMethod::Code => write!(f, "code"),
            TokenizationMethod::GPT4o => write!(f, "gpt4o"),
            TokenizationMethod::GPT4 => write!(f, "gpt4"),
            TokenizationMethod::GPT3 => write!(f, "gpt3"),
            TokenizationMethod::O200kBase => write!(f, "O200kBase"),
            TokenizationMethod::CL100kBase => write!(f, "CL100kBase"),
            TokenizationMethod::P50kBase => write!(f, "P50kBase"),
            TokenizationMethod::P50kEdit => write!(f, "P50kEdit"),
            TokenizationMethod::R50kBase => write!(f, "R50kBase"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tokenizer {
    bpe: Arc<CoreBPE>,
}

impl Tokenizer {
    pub fn new(method: TokenizationMethod) -> Result<Self> {
        let bpe = match method {
            TokenizationMethod::GPT4o => o200k_base()?,
            TokenizationMethod::GPT4 | TokenizationMethod::CL100kBase => cl100k_base()?,
            TokenizationMethod::Code | TokenizationMethod::P50kBase => p50k_base()?,
            TokenizationMethod::P50kEdit => p50k_edit()?,
            TokenizationMethod::GPT3 | TokenizationMethod::R50kBase => r50k_base()?,
            _ => return Err(anyhow!("Unsupported tokenization method: {}", method)),
        };
        Ok(Self { bpe: Arc::new(bpe) })
    }

    pub fn count_tokens(&self, text: &str) -> usize {
        self.bpe.encode_ordinary(text).len()
    }
}
