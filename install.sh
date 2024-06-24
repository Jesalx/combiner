#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v rustc &>/dev/null; then
	echo -e "${RED}Rust is not installed. Please install Rust first.${NC}"
	echo "Visit https://www.rust-lang.org/tools/install for installation instructions."
	exit 1
fi

# Build the project
echo "Building combiner..."
cargo build --release

if [ $? -ne 0 ]; then
	echo -e "${RED}Build failed. Please check the error messages above.${NC}"
	exit 1
fi

# Create directory if it doesn't exist
sudo mkdir -p /usr/local/bin

# Copy the binary to /usr/local/bin
echo "Installing file-combiner..."
sudo cp target/release/combiner /usr/local/bin/

if [ $? -ne 0 ]; then
	echo -e "${RED}Installation failed. Please check if you have the necessary permissions.${NC}"
	exit 1
fi

# Make the binary executable
sudo chmod +x /usr/local/bin/combiner

echo -e "${GREEN}combiner has been successfully installed!${NC}"
echo "You can now use it by running 'combiner' from anywhere in your terminal."
