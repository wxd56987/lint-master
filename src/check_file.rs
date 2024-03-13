use crate::constants::{
    LintResult, RE_LINT_GO, RE_LINT_TS, RE_MATCH_COLOR, RE_TSX_THEME_FILE, SVG_ATTRIBUTE_NAMES,
    TODO_IGNORE_SEARCH, TODO_SEARCH, WELCOME,
};
use crate::draw_table::{DrawTable, GoTable, TsTable};
use crate::utils::{convert_to_camel_case, get_extension};
use colored::Colorize;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::process::Command;

pub struct CheckFile {}

impl CheckFile {
    pub fn run(file_paths: Vec<String>) -> Result<(), Box<dyn Error>> {
        println!("{}", WELCOME.green().bold());
        let mut has_error = false;

        for file_path in file_paths {
            let reader = fs::read_to_string(&file_path)?;
            // support extension name .js .ts .tsx .go
            match get_extension(&file_path) {
                Ok(Some(ext)) => match &ext[..] {
                    "js" | "ts" | "tsx" => {
                        let match_color_result = Self::match_tsx_color(&reader)?;

                        let lint_ts_result = Self::lint_ts(&file_path);
                        let match_svg_attribute_result =
                            Self::match_svg_attribute(&reader, &file_path);
                        let match_todo_result = Self::match_todo(&reader);

                        let ts_table = TsTable {
                            lint_check: lint_ts_result,
                            svg_check: match_svg_attribute_result,
                            todo_check: match_todo_result,
                            color_check: match_color_result,
                        };

                        let errors = ts_table.lint_check.errors
                            + ts_table.svg_check.errors
                            + ts_table.todo_check.errors
                            + ts_table.color_check.errors;

                        DrawTable::draw_ts_table(&file_path, ts_table);

                        if errors > 0 {
                            has_error = true
                        } else {
                            std::process::exit(0);
                        }
                    }
                    "go" => {
                        let lint_go_result = Self::lint_go(&file_path);
                        let match_todo_result = Self::match_todo(&reader);

                        let go_table = GoTable {
                            lint_check: lint_go_result,
                            todo_check: match_todo_result,
                        };

                        let errors = go_table.lint_check.errors + go_table.todo_check.errors;

                        DrawTable::draw_go_table(&file_path, go_table);

                        if errors > 0 {
                            has_error = true
                        } else {
                            std::process::exit(0);
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

    fn match_todo<'a>(contents: &'a str) -> LintResult {
        let mut result = Vec::new();
        for (line_number, line) in contents.lines().enumerate() {
            if line.contains(TODO_SEARCH) {
                if !line.contains(TODO_IGNORE_SEARCH) {
                    let r = format!("line {} has TODO {}", line_number, line);
                    result.push(r)
                }
            }
        }
        LintResult {
            errors: result.len(),
            result,
        }
    }

    fn match_tsx_color(contents: &str) -> Result<LintResult, Box<dyn Error>> {
        let color_contents = fs::read_to_string(&RE_TSX_THEME_FILE)?;
        let mut result = Vec::new();
        let re_s = Regex::new(r"xmlns").unwrap();

        if re_s.is_match(contents) {
            return Ok(LintResult {
                errors: result.len(),
                result,
            });
        }

        let re_a = Regex::new(&RE_MATCH_COLOR).unwrap();
        let mut colors: Vec<String> = Vec::new();
        for cap in re_a.captures_iter(&contents) {
            let color_with_equal = &cap[0];
            colors.push(color_with_equal.to_string());
        }

        let re_b = Regex::new(&RE_MATCH_COLOR).unwrap();
        for line in color_contents.lines() {
            for cap in re_b.captures_iter(line) {
                let color = &cap[0];
                if colors.contains(&color.to_string()) {
                    let r = format!(
                        "Color {} need replace theme.ts definition",
                        color.to_string()
                    );
                    result.push(r);
                }
            }
        }

        Ok(LintResult {
            errors: result.len(),
            result,
        })
    }

    fn lint_ts(file_path: &str) -> LintResult {
        let output = Command::new("eslint")
            .arg(file_path)
            .output()
            .expect("Failed to run ESLint");
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();

        let mut result = Vec::new();
        let re = Regex::new(&RE_LINT_TS).unwrap();

        for line in stdout.lines() {
            if re.is_match(line) {
                result.push(line.trim().to_string());
            }
        }

        LintResult {
            errors: result.len(),
            result,
        }
    }

    fn lint_go(file_path: &str) -> LintResult {
        let output = Command::new("golangci-lint")
            .args(&["run", &file_path])
            .output()
            .expect("Failed to run golangci-lint");

        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();

        let re = Regex::new(&RE_LINT_GO).unwrap();
        let result: Vec<String> = re
            .captures_iter(&stdout)
            .map(|cap| cap.get(1).unwrap().as_str().to_string())
            .collect();

        LintResult {
            errors: result.len(),
            result,
        }
    }

    fn match_svg_attribute(contents: &str, file_path: &str) -> LintResult {
        let mut result = Vec::new();
        if file_path.ends_with(".tsx") {
            for attribute_name in &SVG_ATTRIBUTE_NAMES {
                let re = Regex::new(&format!(r#"{}"#, attribute_name)).unwrap();
                let r: String = format!(
                    "need replace {} to {}",
                    attribute_name.to_string(),
                    convert_to_camel_case(attribute_name)
                );
                if re.is_match(&contents) {
                    result.push(r);
                }
            }
        }

        LintResult {
            errors: result.len(),
            result,
        }
    }
}
