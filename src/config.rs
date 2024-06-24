use anyhow::{Context, Result};
use serde::Deserialize;
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
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ignore_patterns: Option<Vec<String>>,
    pub include_patterns: Option<Vec<String>>,
    pub output_file: Option<String>,
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
            Ok(toml::from_str(&config_str)?)
        }
        None => Ok(Config {
            ignore_patterns: None,
            include_patterns: None,
            output_file: None,
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
        if let Some(include_patterns) = &config.include_patterns {
            println!("Include patterns: {:?}", include_patterns);
        }
    }
}
