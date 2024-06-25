use log::info;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use crate::error::CombinerError;

const DEFAULT_CONFIG_FILE: &str = "combiner.toml";

#[derive(Debug, StructOpt)]
#[structopt(name = "combiner", about = "Combines text files in a directory")]
pub struct Opt {
    /// Input directory to process
    #[structopt(short = "d", long, parse(from_os_str), default_value = ".")]
    pub input_dir: PathBuf,

    /// Output file path
    #[structopt(short, long, parse(from_os_str))]
    pub output_file: Option<PathBuf>,

    /// Patterns to ignore (in addition to those in config)
    #[structopt(short = "g", long)]
    pub ignore_patterns: Vec<String>,

    /// Path to config file
    #[structopt(short, long, parse(from_os_str))]
    pub config_file: Option<PathBuf>,

    /// Enable verbose output
    #[structopt(short, long)]
    pub verbose: bool,

    /// Tokenization method
    #[structopt(
        long,
        parse(try_from_str = parse_tokenization_method),
        possible_values = &TokenizationMethod::variants(),
        case_insensitive = true,
        default_value = "code"
    )]
    pub tokenization_method: TokenizationMethod,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TokenizationMethod {
    #[serde(alias = "gpt4o")]
    O200kBase,
    #[serde(alias = "gpt4")]
    Cl100kBase,
    #[serde(alias = "code")]
    P50kBase,
    P50kEdit,
    #[serde(alias = "gpt2")]
    R50kBase,
}

impl Default for TokenizationMethod {
    fn default() -> Self {
        TokenizationMethod::P50kBase
    }
}

impl TokenizationMethod {
    pub fn variants() -> [&'static str; 9] {
        [
            "gpt4o",
            "o200k_base",
            "gpt4",
            "cl100k_base",
            "code",
            "p50k_base",
            "p50k_edit",
            "gpt2",
            "r50k_base",
        ]
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "gpt4o" | "o200k_base" => Ok(TokenizationMethod::O200kBase),
            "gpt4" | "cl100k_base" => Ok(TokenizationMethod::Cl100kBase),
            "code" | "p50k_base" => Ok(TokenizationMethod::P50kBase),
            "p50k_edit" => Ok(TokenizationMethod::P50kEdit),
            "gpt2" | "r50k_base" => Ok(TokenizationMethod::R50kBase),
            _ => Err(format!("Invalid tokenization method: {}", s)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TokenizationMethod::O200kBase => "gpt4o".to_string(),
            TokenizationMethod::Cl100kBase => "gpt4".to_string(),
            TokenizationMethod::P50kBase => "code".to_string(),
            TokenizationMethod::P50kEdit => "p50k_edit".to_string(),
            TokenizationMethod::R50kBase => "gpt2".to_string(),
        }
    }
}

fn parse_tokenization_method(s: &str) -> Result<TokenizationMethod, String> {
    TokenizationMethod::from_str(s)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub ignore_patterns: Vec<String>,
    #[serde(default)]
    pub include_patterns: Vec<String>,
    pub output_file: Option<PathBuf>,
    #[serde(default)]
    pub tokenization_method: TokenizationMethod,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ignore_patterns: vec![],
            include_patterns: vec![],
            output_file: None,
            tokenization_method: TokenizationMethod::default(),
        }
    }
}

pub fn load_config(opt: &Opt) -> Result<Config, CombinerError> {
    let config_path = opt
        .config_file
        .clone()
        .unwrap_or_else(|| opt.input_dir.join(DEFAULT_CONFIG_FILE));

    if config_path.exists() {
        let config_str = fs::read_to_string(&config_path)
            .map_err(|e| CombinerError::Config(format!("Failed to read config file: {}", e)))?;
        let mut config: Config = toml::from_str(&config_str)
            .map_err(|e| CombinerError::Config(format!("Failed to parse config file: {}", e)))?;

        // If tokenization_method is not specified in the config file, use the CLI option
        if config.tokenization_method == TokenizationMethod::P50kBase {
            config.tokenization_method = opt.tokenization_method.clone();
        }

        Ok(config)
    } else {
        Ok(Config {
            tokenization_method: opt.tokenization_method.clone(),
            ..Default::default()
        })
    }
}

pub fn merge_ignore_patterns(cli_patterns: &[String], config_patterns: &[String]) -> Vec<String> {
    let mut patterns = cli_patterns.to_vec();
    patterns.extend(config_patterns.iter().cloned());
    patterns
}

pub fn determine_output_file(opt: &Opt, config: &Config) -> Result<PathBuf, CombinerError> {
    opt.output_file
        .clone()
        .or_else(|| config.output_file.clone())
        .or_else(|| {
            let datetime = chrono::Local::now().format("%Y%m%d_%H%M%S");
            Some(PathBuf::from(format!(
                "{}{}.txt",
                crate::DEFAULT_OUTPUT_PREFIX,
                datetime
            )))
        })
        .ok_or_else(|| CombinerError::Config("No output file specified".to_string()))
}

pub fn print_verbose_info(
    opt: &Opt,
    output_file: &Path,
    ignore_patterns: &[String],
    config: &Config,
) {
    info!("Input directory: {:?}", opt.input_dir);
    info!("Output file: {:?}", output_file);
    info!("Config file: {:?}", opt.config_file);
    info!("Ignore patterns: {:?}", ignore_patterns);
    info!(
        "Tokenization method: {}",
        config.tokenization_method.to_string()
    );
    info!("Include patterns: {:?}", config.include_patterns);
}

