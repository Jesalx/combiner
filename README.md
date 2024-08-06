# Combiner

Combiner is a Rust-based command-line tool that processes text files in a given directory, combining their contents into a single output file. This tool is particularly useful for providing context to Large Language Models (LLMs) about the files in a project, streamlining the process of getting debugging advice or a project overview.

## Features

- Recursively scans directories for text files
- Configurable file inclusion/exclusion patterns
- Parallel processing for improved performance
- Token counting using the tiktoken-rs library
- Detailed output statistics
- Support for both command-line arguments and configuration files

## Installation

### Prerequisites

- Rust programming language (<https://www.rust-lang.org/tools/install>)

### Building from source

1. Clone the repository:

   ```
   git clone https://github.com/jesalx/combiner.git
   cd combiner
   ```

2. Build the project:

   ```
   cargo build --release
   ```

3. The binary will be available at `target/release/combiner`

Alternatively, you can use install combiner using cargo:

```
cargo install combiner
```

## Usage

Basic usage:

```
combiner -d <input_directory> -o <output_file>
```

For more options:

```
combiner --help
```

### Command-line Options

- `-d, --input-dir <input_dir>`: Input directory to process (default: current directory)
- `-o, --output-file <output_file>`: Output file path
- `-g, --ignore-patterns <ignore_patterns>`: Patterns to ignore (in addition to those in config)
- `-c, --config-file <config_file>`: Path to config file
- `-v, --verbose`: Enable verbose output

### Configuration File

You can use a TOML configuration file to set default options. By default, the program looks for a `combiner.toml` file in the input directory. You can specify a different config file using the `-c` option.

Example `combiner.toml`:

```toml
ignore_patterns = ["*.log", "*.tmp"]
include_patterns = ["*.rs", "*.toml"]
output_file = "combined_output.txt"
```

## Output

The program generates a single output file containing the contents of all processed text files. Each file's content is preceded by its file path and separated by a line of dashes.

The program also prints a summary table showing:

- Number of files processed
- Total number of tokens
- Output file path
- Processing time
- Top files by token count
