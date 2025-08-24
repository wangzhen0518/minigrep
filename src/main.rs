#![allow(dead_code)]

use std::{error::Error, fs};

use clap::Parser;
use colored::Colorize;

#[derive(Debug, Parser)]
struct Config {
    /// The file to search for the pattern
    ///
    /// This should be a valid file path that exists and is readable.
    /// The program will search through this file for occurrences of the pattern.
    filename: String,

    /// The pattern to search for in the file
    ///
    /// This is the text string that the program will look for in the file.
    /// The search can be case-sensitive or case-insensitive based on the --insensitive flag.
    #[arg(long, short)]
    pattern: String,

    /// Enable case-insensitive search
    ///
    /// When this flag is set, the search will ignore case differences.
    /// For example, "Hello" will match "hello", "HELLO", etc.
    #[arg(long, short)]
    insensitive: bool,
}

impl Config {
    fn myparse(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 3 {
            Err(format!(
                "useage: minigrep <FILENAME> <PATTERN>, but the number of args is {}",
                args.len() - 1
            )
            .into())
        } else {
            let filename = args[1].to_string();
            let insensitive = if args.len() >= 4 {
                let insensitive_arg = args[3].to_lowercase();
                insensitive_arg == "--insensitive" || insensitive_arg == "-i"
            } else {
                false
            };
            let pattern = if insensitive {
                args[2].to_lowercase()
            } else {
                args[2].to_string()
            };
            Ok(Config {
                filename,
                pattern,
                insensitive,
            })
        }
    }

    fn build() -> Config {
        let mut config = Config::parse();
        if config.insensitive {
            config.pattern = config.pattern.to_lowercase();
        }
        config
    }
}

#[derive(Debug)]
struct Match {
    start: usize,
    end: usize,
    line: usize,
    pattern: String,
    content: String,
    highlight_content: String,
}

fn find_match(config: &Config) -> Vec<Match> {
    let mut matches = vec![];
    let file_content = fs::read_to_string(&config.filename).expect("Failed to read file");
    for (line, content) in file_content.lines().enumerate() {
        let new_content = if config.insensitive {
            content.to_lowercase()
        } else {
            content.to_string()
        };
        if let Some(start) = new_content.find(&config.pattern) {
            let end = start + config.pattern.len();
            let highlight_content = format!(
                "{}{}{}",
                &content[..start],
                &content[start..end].red(),
                &content[end..]
            );
            matches.push(Match {
                start,
                end,
                line,
                pattern: config.pattern.clone(),
                content: content.to_string(),
                highlight_content,
            });
        }
    }

    matches
}

fn main() {
    let config = Config::build();
    let matches = find_match(&config);
    for match_res in matches {
        println!(
            "{}:{}:{}-{}: {}",
            config.filename,
            match_res.line + 1,
            match_res.start + 1,
            match_res.end + 1,
            match_res.highlight_content
        );
    }
}
