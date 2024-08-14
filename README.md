# Combiner

Combiner is a Rust-based command-line tool that processes text files in a given directory, combining their contents into a single output file. This tool is particularly useful for providing context to Large Language Models (LLMs) about the files in a project, streamlining the process of getting debugging advice or a project overview.

## Features

- Recursively scans directories for text files
- Token counting using the tiktoken-rs library
- Detailed output statistics

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
combiner -d <directory> -o <output>
```

For more options:

```
combiner --help
```

### Command-line Options

- `-d, --directory <directory>`: Input directory to process (default: current directory)
- `-o, --output <output>`: Output file path/name

## Output

The program generates a single output file containing the contents of all processed text files. Each file's content is preceded by its file path and separated by a line of dashes.

The program also prints a summary table showing:

- Number of files processed
- Total number of tokens
- Output file path
- Processing time
- Top file by token count
