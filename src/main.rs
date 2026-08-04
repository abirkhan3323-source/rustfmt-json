// rustfmt-json: fast JSON formatter, validator, and minifier.
// Usage: rustfmt-json [--minify] [--validate] <file...>

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process;

struct Config {
    minify: bool,
    validate_only: bool,
    files: Vec<PathBuf>,
}

fn main() {
    let config = parse_args();

    if config.files.is_empty() {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read stdin");
        process_json(&input, "<stdin>", &config);
        return;
    }

    let mut exit_code = 0;
    for file in &config.files {
        match fs::read_to_string(file) {
            Ok(content) => {
                if !process_json(&content, &file.display().to_string(), &config) {
                    exit_code = 1;
                }
            }
            Err(e) => {
                eprintln!("Error reading {}: {}", file.display(), e);
                exit_code = 1;
            }
        }
    }
    process::exit(exit_code);
}

fn process_json(input: &str, source: &str, config: &Config) -> bool {
    let value: serde_json::Value = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: Invalid JSON: {}", source, e);
            return false;
        }
    };

    if config.validate_only {
        println!("{}: Valid JSON", source);
        return true;
    }

    let output = if config.minify {
        serde_json::to_string(&value).unwrap_or_default()
    } else {
        serde_json::to_string_pretty(&value).unwrap_or_default()
    };

    if config.files.len() <= 1 {
        println!("{}", output);
    } else {
        println!("// {}\n{}", source, output);
    }
    true
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut config = Config {
        minify: false,
        validate_only: false,
        files: Vec::new(),
    };

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--minify" | "-m" => config.minify = true,
            "--validate" | "-v" => config.validate_only = true,
            "--help" | "-h" => {
                println!("rustfmt-json — format, validate, and minify JSON\n");
                println!("Usage: rustfmt-json [OPTIONS] [FILE...]\n");
                println!("Options:");
                println!("  -m, --minify    Output minified JSON");
                println!("  -v, --validate  Only validate, don't output");
                println!("  -h, --help      Show this help");
                process::exit(0);
            }
            arg if !arg.starts_with('-') => {
                config.files.push(PathBuf::from(arg));
            }
            unknown => {
                eprintln!("Unknown option: {}", unknown);
                process::exit(1);
            }
        }
        i += 1;
    }

    config
}
