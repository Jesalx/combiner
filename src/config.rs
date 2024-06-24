use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

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
    pub ignore_patterns: Option<Vec<String>>,
    pub include_patterns: Option<Vec<String>>,
    pub output_file: Option<String>,
    pub tokenization_method: Option<TokenizationMethod>,
}

pub fn load_config(opt: &mut Opt) -> Result<Config> {
    if opt.config_file.is_none() {
        let default_config = opt.input_dir.join(DEFAULT_CONFIG_FILE);
        if default_config.exists() {
            opt.config_file = Some(default_config);
        }
    }

    match &opt.config_file {
        Some(path) => {
            let config_str = fs::read_to_string(path)
                .with_context(|| format!("Failed to read config file: {:?}", path))?;
            let mut config: Config = toml::from_str(&config_str)?;

            // If tokenization_method is not specified in the config file, use the CLI option
            if config.tokenization_method.is_none() {
                config.tokenization_method = Some(opt.tokenization_method.clone());
            }

            Ok(config)
        }
        None => Ok(Config {
            ignore_patterns: None,
            include_patterns: None,
            output_file: None,
            tokenization_method: Some(opt.tokenization_method.clone()),
        }),
    }
}

pub fn merge_ignore_patterns(
    cli_patterns: &[String],
    config_patterns: &Option<Vec<String>>,
) -> Vec<String> {
    let mut patterns = cli_patterns.to_vec();
    if let Some(config_patterns) = config_patterns {
        patterns.extend(config_patterns.iter().cloned());
    }
    patterns
}

pub fn determine_output_file(opt: &mut Opt, config: &Config) -> Result<PathBuf> {
    if opt.output_file.is_none() {
        opt.output_file = config.output_file.as_ref().map(PathBuf::from).or_else(|| {
            let datetime = chrono::Local::now().format("%Y%m%d_%H%M%S");
            Some(PathBuf::from(format!(
                "{}{}.txt",
                crate::DEFAULT_OUTPUT_PREFIX,
                datetime
            )))
        });
    }
    Ok(opt.output_file.as_ref().unwrap().to_path_buf())
}

pub fn print_verbose_info(
    opt: &Opt,
    output_file: &Path,
    ignore_patterns: &[String],
    config: &Config,
) {
    if opt.verbose {
        println!("Input directory: {:?}", opt.input_dir);
        println!("Output file: {:?}", output_file);
        println!("Config file: {:?}", opt.config_file);
        println!("Ignore patterns: {:?}", ignore_patterns);
        println!(
            "Tokenization method: {}",
            config
                .tokenization_method
                .as_ref()
                .unwrap_or(&opt.tokenization_method)
                .to_string()
        );
        if let Some(include_patterns) = &config.include_patterns {
            println!("Include patterns: {:?}", include_patterns);
        }
    }
}
