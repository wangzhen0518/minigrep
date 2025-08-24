# MiniGrep - Rust Learning Project

A simple Rust practice project implementing basic text search functionality similar to grep.

## Project Overview

This is a foundational Rust learning project that implements the following features:
- Searching for specific text patterns in files
- Support for both case-sensitive and case-insensitive search
- Colored output to highlight matched text
- Display of match locations (filename, line number, character positions)

## Building and Running

### Prerequisites
Install https://www.rust-lang.org/tools/install

### Compile the Project
```shell
cargo build
```

### Run the Project
```shell
# Using cargo run (requires arguments)
cargo run -- <filename> <pattern> [--insensitive]

# Or compile first and run the binary directly
cargo build --release
./target/release/minigrep <filename> <pattern> [--insensitive]
```

### Usage Examples
```shell
# Search for specific text in a file
minigrep sample.txt hello

# Case-insensitive search
minigrep sample.txt HELLO -i

# Using long option names
minigrep sample.txt --pattern hello --insensitive
```
