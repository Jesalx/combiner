use tiktoken_rs::{cl100k_base, o200k_base, p50k_base, p50k_edit, r50k_base};

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
