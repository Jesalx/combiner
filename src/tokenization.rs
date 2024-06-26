use clap::ValueEnum;
use std::fmt;

#[derive(Debug, Clone, ValueEnum)]
pub enum TokenizationMethod {
    Code,
    GPT4o,
    GPT4,
    GPT2,
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
            TokenizationMethod::GPT2 => write!(f, "gpt2"),
            TokenizationMethod::O200kBase => write!(f, "O200kBase"),
            TokenizationMethod::CL100kBase => write!(f, "CL100kBase"),
            TokenizationMethod::P50kBase => write!(f, "P50kBase"),
            TokenizationMethod::P50kEdit => write!(f, "P50kEdit"),
            TokenizationMethod::R50kBase => write!(f, "R50kBase"),
        }
    }
}
