use crate::constants::{
    LintResult, CONSOLE_LOG, FILE_LINE, NECESSARY_CONSOLE_LOGGING, RE_LINT_GO, RE_LINT_TS,
    RE_MATCH_COLOR, RE_TSX_THEME_FILE, SVG_ATTRIBUTE_NAMES, TODO_IGNORE_SEARCH, TODO_SEARCH,
    WELCOME,
};
use crate::draw_table::{DrawTable, GoTable, TsTable};
use crate::utils::{convert_to_camel_case, get_extension};
use colored::Colorize;
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::process::Command;

pub struct CheckFile {}

impl CheckFile {
    pub fn run(file_paths: Vec<String>) -> Result<(), Box<dyn Error>> {
        println!("{}", WELCOME.green().bold());
        let mut check_errors: u16 = 0;
        let diff_add_files: Vec<String> = Self::git_add_files();

        for file_path in file_paths {
            let reader = fs::read_to_string(&file_path)?;
            // support extension name .js .ts .tsx .go
            match get_extension(&file_path) {
                Ok(Some(ext)) => match &ext[..] {
                    "js" | "ts" | "tsx" => {
                        let match_color_result = Self::match_tsx_color(&reader, &mut check_errors)?;

                        let lint_ts_result = Self::lint_ts(&file_path, &mut check_errors);
                        let match_svg_attribute_result =
                            Self::match_svg_attribute(&reader, &file_path, &mut check_errors);
                        let match_todo_result = Self::match_todo(&reader, &mut check_errors);
                        let match_image_alt = Self::match_image_alt(&reader, &mut check_errors);
                        let match_a_rel = Self::match_a_rel(&reader, &mut check_errors);

                        let check_file_lines = Self::check_file_lines(
                            &file_path,
                            &mut check_errors,
                            &diff_add_files,
                            &reader,
                        );

                        let match_console_log_result = Self::match_console_log(&reader, &mut check_errors);

                        let ts_table = TsTable {
                            lint_check: lint_ts_result,
                            svg_check: match_svg_attribute_result,
                            todo_check: match_todo_result,
                            color_check: match_color_result,
                            image_alt_check: match_image_alt,
                            a_rel_check: match_a_rel,
                            file_line_check: check_file_lines,
                            console_log_check: match_console_log_result,
                        };

                        DrawTable::draw_ts_table(&file_path, ts_table);
                    }
                    "go" => {
                        let lint_go_result = Self::lint_go(&file_path);
                        let match_todo_result = Self::match_todo(&reader, &mut check_errors);
                        let check_file_lines = Self::check_file_lines(
                            &file_path,
                            &mut check_errors,
                            &diff_add_files,
                            &reader,
                        );

                        let go_table = GoTable {
                            lint_check: lint_go_result,
                            todo_check: match_todo_result,
                            file_line_check: check_file_lines,
                        };

                        DrawTable::draw_go_table(&file_path, go_table);
                    }
                    _ => {
                        std::process::exit(0);
                    }
                },
                Ok(None) => {}
                Err(err) => {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                }
            }
        }

        if check_errors > 0 {
            println!("All errors total {}", check_errors.to_string().red().bold());
            std::process::exit(1);
        } else {
            println!(
                "All errors total {}",
                check_errors.to_string().green().bold()
            );
        }

        Ok(())
    }

    fn match_todo<'a>(contents: &'a str, check_errors: &mut u16) -> LintResult {
        let mut result = Vec::new();
        for (line_number, line) in contents.lines().enumerate() {
            let line = line.trim_start();
            if line.starts_with("//") && line.contains(TODO_SEARCH) {
                if !line.contains(TODO_IGNORE_SEARCH) {
                    let r = format!("line {} has TODO {}", line_number, line);
                    result.push(r);
                }
            }
        }

        *check_errors += result.len() as u16;

        LintResult {
            errors: result.len(),
            result,
        }
    }

    fn match_console_log<'a>(contents: &'a str, check_errors: &mut u16) -> LintResult {
        let mut match_necessary = 0;
        let mut match_console = 0;
        let mut result = Vec::new();
        for line in contents.lines() {
            let line = line.trim_start();
            if line.starts_with("//") && line.contains(NECESSARY_CONSOLE_LOGGING) {
                match_necessary += 1;
            }
            if line.contains(CONSOLE_LOG) {
                match_console += 1;
            }
        }

        let difference = match_console - match_necessary;
        if difference > 0 {
            *check_errors += difference;
            let r = format!("file has {} console.log", difference);
            result.push(r);
        }

        LintResult {
            errors: difference as usize,
            result,
        }
    }

    fn match_a_rel<'a>(contents: &'a str, check_errors: &mut u16) -> LintResult {
        let mut result = Vec::new();
        let re = Regex::new(r#"href=[^>]*>"#).unwrap();
        for cap in re.find_iter(&contents) {
            let line = &contents[cap.start()..cap.end()];
            if !line.contains("rel=") {
                let r = format!("a tag need set <rel> value: {}", line);
                result.push(r);
            }
        }

        *check_errors += result.len() as u16;

        LintResult {
            errors: result.len(),
            result,
        }
    }

    fn match_image_alt<'a>(contents: &'a str, check_errors: &mut u16) -> LintResult {
        let mut result = Vec::new();
        let re = Regex::new(r#"<Image[^>]*>"#).unwrap();
        for cap in re.find_iter(&contents) {
            let line = &contents[cap.start()..cap.end()];
            if !line.contains("alt=") {
                let r = format!("img tag need set <alt> value: {}", line);
                result.push(r);
            }
        }

        *check_errors += result.len() as u16;

        LintResult {
            errors: result.len(),
            result,
        }
    }

    fn match_tsx_color(
        contents: &str,
        check_errors: &mut u16,
    ) -> Result<LintResult, Box<dyn Error>> {
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
        let mut unique_result: HashSet<_> = result.drain(..).collect();
        result.extend(unique_result.drain());

        *check_errors += result.len() as u16;

        Ok(LintResult {
            errors: result.len(),
            result,
        })
    }

    fn lint_ts(file_path: &str, check_errors: &mut u16) -> LintResult {
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

        *check_errors += result.len() as u16;

        LintResult {
            errors: result.len(),
            result,
        }
    }

    fn git_add_files() -> Vec<String> {
        let mut result = Vec::new();
        let output = Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .output()
            .expect("Failed to run golangci-lint");

        let stdout: String = String::from_utf8_lossy(&output.stdout).into_owned();

        for line in stdout.lines() {
            if line.starts_with("A") {
                result.push(line.to_string());
            }
        }

        result
    }

    fn check_file_lines(
        file_path: &str,
        check_errors: &mut u16,
        diff_add_files: &Vec<String>,
        contents: &str,
    ) -> LintResult {
        let mut result = Vec::new();
        let len = contents.lines().count();

        let f = format!("A  {}", &file_path.to_string());
        if diff_add_files.contains(&f) && len > FILE_LINE as usize {
            let r = format!("File cannot be larger than {} lines", FILE_LINE);
            result.push(r);
        }

        *check_errors += result.len() as u16;

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

    fn match_svg_attribute(contents: &str, file_path: &str, check_errors: &mut u16) -> LintResult {
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

        *check_errors += result.len() as u16;

        LintResult {
            errors: result.len(),
            result,
        }
    }
}
