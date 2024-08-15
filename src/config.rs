/// Configuration for the file combiner
#[derive(Debug, Clone)]
pub struct CombinerConfig {
    pub directory: String,
    pub output: String,
    pub tokenizer: String,
}

impl CombinerConfig {
    pub fn new(directory: String, output: String, tokenizer: String) -> Self {
        Self {
            directory,
            output,
            tokenizer,
        }
    }
}
