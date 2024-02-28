use colored::*;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
struct Line<'a> {
    line_number: usize,
    content: &'a str,
}

pub struct Config {
    pub file_paths: Vec<String>,
}

const TODO_SEARCH: &'static str = "TODO";

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            eprintln!("Not enough arguments");
            std::process::exit(1);
        }
        let file_paths = args[1..].to_vec();
        Ok(Config { file_paths })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_paths = config.file_paths;
    let mut has_error = false;

    for file_path in file_paths {
        let reader = fs::read_to_string(&file_path)?;
        // support extension name .js .ts .tsx .go
        match get_extension(&file_path) {
            Ok(Some(ext)) => match &ext[..] {
                "js" | "ts" | "tsx" => {
                    lint_ts(&file_path);
                    let mut errors = match_todo(&reader, &file_path);
                    errors += match_color_use(&reader, &file_path);
                    errors += match_svg_attribute(&reader, &file_path);
                    if errors > 0 {
                        has_error = true
                    }
                }
                "go" => {
                    let mut errors = lint_golang(&file_path);
                    errors += match_todo(&reader, &file_path);
                    if errors > 0 {
                        has_error = true
                    }
                }
                _ => {
                    eprintln!("File type not supported.");
                    std::process::exit(1);
                }
            },
            Ok(None) => {
                has_error = true;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                has_error = true;
            }
        }
    }

    if has_error {
        std::process::exit(1);
    }

    Ok(())
}

fn match_todo<'a>(contents: &'a str, file_path: &str) -> usize {
    let mut results = Vec::new();
    let mut errors = 0;
    for (line_number, line) in contents.lines().enumerate() {
        if line.contains(TODO_SEARCH) {
            let line_struct = Line {
                line_number: line_number + 1,
                content: line,
            };
            results.push(line_struct)
        }
    }

    if results.len() > 0 {
        errors = results.len();
        println!(
            "\n 🧐 {} {} search {} result is: {} 👇",
            "+---------".red().bold(),
            file_path.green().bold(),
            "TODO".italic().yellow().bold(),
            "---------+".red().bold()
        );
        for line in results {
            println!(
                "\n 👉 {} file has TODO, in {} line {}",
                file_path, line.line_number, line.content
            );
        }
    } else {
        println!(
            "\n 🎉 {} {} no {} find {} ✅",
            "+---------".blue().bold(),
            file_path.green().bold(),
            "TODO".italic().yellow().bold(),
            "---------+".blue().bold()
        );
    }

    errors
}

fn match_color_use(contents: &str, file_path: &str) -> usize {
    let mut errors = 0;
    if file_path.ends_with(".tsx") {
        let mut results = Vec::new();
        let re = Regex::new(r#"(?i)\s*#([0-9a-f]{6})\s*"#).unwrap();

        for (line_number, line) in contents.lines().enumerate() {
            if let Some(_) = re.find(line) {
                let line_struct = Line {
                    line_number: line_number + 1,
                    content: line,
                };
                results.push(line_struct)
            }
        }

        if results.len() > 0 {
            errors = results.len();
            println!(
                "\n 🧐 {} {} {} are not referenced {} 👇",
                "+---------".red().bold(),
                file_path.green().bold(),
                "Color".italic().yellow().bold(),
                "---------+".red().bold()
            );
            for line in results {
                println!(
                    "\n 👉 {} file has not referenced color, in {} line {}",
                    file_path, line.line_number, line.content
                );
            }
        } else {
            println!(
                "\n 🎉 {} {} no {} find not referenced {} ✅",
                "+---------".blue().bold(),
                file_path.green().bold(),
                "Color".italic().yellow().bold(),
                "---------+".blue().bold()
            );
        }
    }

    errors
}

fn match_svg_attribute(contents: &str, file_path: &str) -> usize {
    let mut errors = 0;
    if file_path.ends_with(".tsx") {
        let mut results = Vec::new();
        let attribute_names = vec![
            "fill-rule",
            "clip-rule",
            "fill-opacity",
            "stroke-opacity",
            "stop-color",
            "stop-opacity",
            "clip-path",
            "font-size",
            "font-weight",
            "text-anchor",
            "alignment-baseline",
            "baseline-shift",
            "word-spacing",
            "letter-spacing",
            "text-decoration",
            "font-style",
            "font-variant",
            "line-height",
            "writing-mode",
            "shape-rendering",
            "image-rendering",
            "color-interpolation",
            "color-interpolation-filters",
            "shape-rendering",
            "color-rendering",
            "flood-color",
            "flood-opacity",
            "lighting-color",
            "text-rendering",
            "stroke-dasharray",
            "stroke-dashoffset",
            "stroke-linecap",
            "stroke-linejoin",
            "stroke-miterlimit",
            "transform-origin",
        ];

        for attribute_name in &attribute_names {
            let re = Regex::new(&format!(r#"{}"#, attribute_name)).unwrap();
            if re.is_match(&contents) {
                results.push(attribute_name);
            }
        }

        if results.len() > 0 {
            errors = results.len();
            println!(
                "\n 🚗 {} {} {} attribute lint result is: {} 👇",
                "+---------".red().bold(),
                file_path.green().bold(),
                "SVG".italic().yellow().bold(),
                "---------+".red().bold()
            );
            for r in results {
                println!(
                    "\n 👉 {} attribute found. Use {} instead",
                    r,
                    convert_to_camel_case(r)
                );
            }
        } else {
            println!(
                "\n 🎉 {} {} {} lint ok {} ✅",
                "+---------".blue().bold(),
                file_path.green().bold(),
                "SVG".italic().yellow().bold(),
                "---------+".blue().bold()
            );
        }
    }
    errors
}

fn convert_to_camel_case(contents: &str) -> String {
    let mut result = String::new();
    let mut should_capitalize_next = false;

    for c in contents.chars() {
        if c == '-' {
            should_capitalize_next = true;
        } else {
            if should_capitalize_next {
                result.push(c.to_ascii_uppercase());
                should_capitalize_next = false;
            } else {
                result.push(c);
            }
        }
    }

    result
}

fn lint_ts(file_path: &str) -> usize {
    let output = Command::new("eslint")
        .arg(file_path)
        .output()
        .expect("Failed to run ESLint");

    if output.stdout.len() > 0 {
        println!(
            "\n 🚗 {} {} use {} lint result is: {} 👇",
            "+---------".red().bold(),
            file_path.green().bold(),
            "eslint".italic().yellow().bold(),
            "---------+".red().bold()
        );
        println!("\n 👉 {}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!(
            "\n 🎉 {} {} no {} find {} ✅",
            "+---------".blue().bold(),
            file_path.green().bold(),
            "lint".italic().yellow().bold(),
            "---------+".blue().bold()
        );
    }

    output.stdout.len()
}

fn lint_golang(file_path: &str) -> usize {
    let output = Command::new("golangci-lint")
        .args(&["run", &file_path])
        .output()
        .expect("Failed to run golangci-lint");

    if output.stdout.len() > 0 {
        println!(
            "\n 🚗 {} {} use {} lint result is: {} 👇",
            "+---------".blue().bold(),
            file_path.green().bold(),
            "golangci-lint".italic().yellow().bold(),
            "---------+".blue().bold(),
        );
        println!("\n 👉 {}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!(
            "\n 🎉 {} {} no {} find {} ✅",
            "+---------".blue().bold(),
            file_path.green().bold(),
            "lint".italic().yellow().bold(),
            "---------+".blue().bold()
        );
    }

    output.stdout.len()
}

fn get_extension(file_path: &str) -> Result<Option<String>, &'static str> {
    if let Some(extension) = Path::new(file_path).extension() {
        if let Some(ext_str) = extension.to_str() {
            let ext = ext_str.to_lowercase();
            if ext == "js" || ext == "ts" || ext == "tsx" || ext == "go" {
                return Ok(Some(ext));
            }
        }
    }
    Ok(None)
}
