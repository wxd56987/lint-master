use colored::Colorize;
use comfy_table::*;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct Config {
    pub file_paths: Vec<String>,
}

const TODO_SEARCH: &'static str = "TODO";
const TODO_IGNORE_SEARCH: &'static str = "IGNORE";
const CONGRATULATE: &'static str = "✨ Congratulate all passed 🎉🎉🎉";
const WELCOME: &'static str = r#"
    __     ____ _   __ ______   __  ___ ___    _____ ______ ______ ____ 
   / /    /  _// | / //_  __/  /  |/  //   |  / ___//_  __// ____// __ \
  / /     / / /  |/ /  / /    / /|_/ // /| |  \__ \  / /  / __/  / /_/ /
 / /___ _/ / / /|  /  / /    / /  / // ___ | ___/ / / /  / /___ / _  _/ 
/_____//___//_/ |_/  /_/    /_/  /_//_/  |_|/____/ /_/  /_____//_/ |_|  
                                                                        
"#;

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
    println!("{}", WELCOME.green().bold());
    let file_paths = config.file_paths;
    let mut has_error = false;

    for file_path in file_paths {
        let reader = fs::read_to_string(&file_path)?;
        // support extension name .js .ts .tsx .go
        match get_extension(&file_path) {
            Ok(Some(ext)) => match &ext[..] {
                "js" | "ts" | "tsx" => {
                    let file_path_theme: &str = "apps/identity-hub/config/theme.ts";
                    let reader_color = fs::read_to_string(file_path_theme)?;
                    let match_color_result = match_tsx_color(&reader_color, &reader);

                    let lint_ts_result = lint_ts(&file_path);
                    let match_svg_attribute_result = match_svg_attribute(&reader, &file_path);
                    let match_todo_result = match_todo(&reader);

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

                    draw_ts_table(&file_path, ts_table);

                    if errors > 0 {
                        has_error = true
                    } else {
                        std::process::exit(0);
                    }
                }
                "go" => {
                    let lint_go_result = lint_go(&file_path);
                    let match_todo_result = match_todo(&reader);

                    let go_table = GoTable {
                        lint_check: lint_go_result,
                        todo_check: match_todo_result,
                    };

                    let errors = go_table.lint_check.errors + go_table.todo_check.errors;

                    draw_go_table(&file_path, go_table);

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

struct LintTodo {
    errors: usize,
    result: Vec<String>,
}

fn match_todo<'a>(contents: &'a str) -> LintTodo {
    let mut result = Vec::new();
    for (line_number, line) in contents.lines().enumerate() {
        if line.contains(TODO_SEARCH) {
            if !line.contains(TODO_IGNORE_SEARCH) {
                let r = format!("line {} has TODO {}", line_number, line);
                result.push(r)
            }
        }
    }
    LintTodo {
        errors: result.len(),
        result,
    }
}

struct LintTSXColor {
    errors: usize,
    result: Vec<String>,
}

fn match_tsx_color(color_contents: &str, contents: &str) -> LintTSXColor {
    let mut result = Vec::new();
    let re_a = Regex::new(r"#[0-9a-fA-F]{6}").unwrap();
    let mut colors: Vec<String> = Vec::new();
    for cap in re_a.captures_iter(&contents) {
        let color_with_equal = &cap[0];
        colors.push(color_with_equal.to_string());
    }

    let re_b = Regex::new(r"#[0-9a-fA-F]{6}").unwrap();
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

    LintTSXColor {
        errors: result.len(),
        result,
    }
}
struct LintSVG {
    errors: usize,
    result: Vec<String>,
}

fn match_svg_attribute(contents: &str, file_path: &str) -> LintSVG {
    let mut result = Vec::new();
    if file_path.ends_with(".tsx") {
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

    LintSVG {
        errors: result.len(),
        result,
    }
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

struct LintTs {
    errors: usize,
    result: Vec<String>,
}

fn lint_ts(file_path: &str) -> LintTs {
    let output = Command::new("eslint")
        .arg(file_path)
        .output()
        .expect("Failed to run ESLint");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();

    let mut result = Vec::new();
    let re = Regex::new(r"^\s*\d+:\d+\s+error.*").unwrap();

    for line in stdout.lines() {
        if re.is_match(line) {
            result.push(line.trim().to_string());
        }
    }

    LintTs {
        errors: result.len(),
        result,
    }
}

struct LintGo {
    errors: usize,
    result: Vec<String>,
}

fn lint_go(file_path: &str) -> LintGo {
    let output = Command::new("golangci-lint")
        .args(&["run", &file_path])
        .output()
        .expect("Failed to run golangci-lint");

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();

    let re = Regex::new(r".*?:(\d+:\d+:\s.*?(?:\n\s+.*?)+)").unwrap();
    let result: Vec<String> = re
        .captures_iter(&stdout)
        .map(|cap| cap.get(1).unwrap().as_str().to_string())
        .collect();

    LintGo {
        errors: result.len(),
        result,
    }
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

struct TsTable {
    lint_check: LintTs,
    svg_check: LintSVG,
    todo_check: LintTodo,
    color_check: LintTSXColor,
}

fn draw_ts_table<'a>(file_path: &str, lint: TsTable) {
    let mut table = Table::new();

    let TsTable {
        lint_check,
        svg_check,
        todo_check,
        color_check,
    } = lint;

    let mut lines_lint: Vec<String> = lint_check
        .result
        .iter()
        .map(|check| overflow_text(check).join("\n"))
        .collect();

    let mut lines_svg: Vec<String> = svg_check
        .result
        .iter()
        .map(|check| overflow_text(check).join("\n"))
        .collect();
    let mut lines_todo: Vec<String> = todo_check
        .result
        .iter()
        .map(|check| overflow_text(check).join("\n"))
        .collect();

    let mut lines_color: Vec<String> = color_check
        .result
        .iter()
        .map(|check| overflow_text(check).join("\n"))
        .collect();

    for s in lines_lint.iter_mut() {
        *s = format!("🤔 {}", s);
    }

    for s in lines_svg.iter_mut() {
        *s = format!("🤔 {}", s);
    }

    for s in lines_todo.iter_mut() {
        *s = format!("🤔 {}", s);
    }

    for s in lines_color.iter_mut() {
        *s = format!("🤔 {}", s);
    }

    let cell_lint_r = if lines_lint.len() > 0 {
        lines_lint.join("\n")
    } else {
        CONGRATULATE.to_string()
    };

    let cell_svg_r = if lines_svg.len() > 0 {
        lines_svg.join("\n")
    } else {
        CONGRATULATE.to_string()
    };

    let cell_todo_r = if lines_todo.len() > 0 {
        lines_todo.join("\n")
    } else {
        CONGRATULATE.to_string()
    };

    let cell_color_r = if lines_color.len() > 0 {
        lines_color.join("\n")
    } else {
        CONGRATULATE.to_string()
    };

    let cell_lint = Cell::new(cell_lint_r);
    let cell_svg = Cell::new(cell_svg_r);
    let cell_todo = Cell::new(cell_todo_r);
    let cell_color = Cell::new(cell_color_r);

    let file_name = format!("📃 {}", get_file_name(file_path));

    table
        .set_header(vec![
            Cell::new(file_name).fg(Color::Green),
            Cell::new("🧐 CHECK").fg(Color::Green),
            Cell::new("🎃 MATCHED").fg(Color::Green),
            Cell::new("🐝 STATUS").fg(Color::Green),
        ])
        .add_row(vec![
            Cell::new("🍓 ESLINT").fg(Color::Yellow),
            cell_lint,
            Cell::new(lint_check.errors.to_string()).fg(status_color(lint_check.errors)),
            Cell::new(status_emoji(lint_check.errors)),
        ])
        .add_row(vec![
            Cell::new("🥝 SVG").fg(Color::Yellow),
            cell_svg,
            Cell::new(svg_check.errors.to_string()).fg(status_color(svg_check.errors)),
            Cell::new(status_emoji(svg_check.errors)),
        ])
        .add_row(vec![
            Cell::new("🍋 TODO").fg(Color::Yellow),
            cell_todo,
            Cell::new(todo_check.errors.to_string()).fg(status_color(todo_check.errors)),
            Cell::new(status_emoji(todo_check.errors)),
        ])
        .add_row(vec![
            Cell::new("🎨 COLOR").fg(Color::Yellow),
            cell_color,
            Cell::new(color_check.errors.to_string()).fg(status_color(color_check.errors)),
            Cell::new(status_emoji(color_check.errors)),
        ]);

    println!("{table}");
}

fn status_emoji<'a>(flag: usize) -> &'a str {
    if flag > 0 {
        "🔴 "
    } else {
        "✅ "
    }
}

fn status_color(flag: usize) -> Color {
    if flag > 0 {
        Color::Red
    } else {
        Color::Green
    }
}

struct GoTable {
    lint_check: LintGo,
    todo_check: LintTodo,
}

fn draw_go_table<'a>(file_path: &str, lint: GoTable) {
    let mut table = Table::new();

    let GoTable {
        lint_check,
        todo_check,
    } = lint;

    let mut lines_lint: Vec<String> = lint_check
        .result
        .iter()
        .map(|check| overflow_text(check).join("\n"))
        .collect();

    let mut lines_todo: Vec<String> = todo_check
        .result
        .iter()
        .map(|check| overflow_text(check).join("\n"))
        .collect();

    for s in lines_lint.iter_mut() {
        *s = format!("🤔 {}", s);
    }

    for s in lines_todo.iter_mut() {
        *s = format!("🤔 {}", s);
    }

    let cell_lint_r = if lines_lint.len() > 0 {
        lines_lint.join("\n")
    } else {
        CONGRATULATE.to_string()
    };

    let cell_todo_r = if lines_todo.len() > 0 {
        lines_todo.join("\n")
    } else {
        CONGRATULATE.to_string()
    };

    let cell_lint = Cell::new(cell_lint_r);
    let cell_todo = Cell::new(cell_todo_r);

    let file_name = format!("📃 {}", get_file_name(file_path));

    table
        .set_header(vec![
            Cell::new(file_name).fg(Color::Green),
            Cell::new("🧐 CHECK").fg(Color::Green),
            Cell::new("🎃 MATCHED").fg(Color::Green),
            Cell::new("🐝 STATUS").fg(Color::Green),
        ])
        .add_row(vec![
            Cell::new("🍓 GOLANGCI_LINT").fg(Color::Yellow),
            cell_lint,
            Cell::new(lint_check.errors.to_string()).fg(status_color(lint_check.errors)),
            Cell::new(status_emoji(lint_check.errors)),
        ])
        .add_row(vec![
            Cell::new("🍋 TODO").fg(Color::Yellow),
            cell_todo,
            Cell::new(todo_check.errors.to_string()).fg(status_color(todo_check.errors)),
            Cell::new(status_emoji(todo_check.errors)),
        ]);

    println!("{table}");
}

fn overflow_text(long_text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let max_width = 50;
    for word in long_text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_width {
            lines.push(current_line.clone());
            current_line.clear();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

fn get_file_name(file_path: &str) -> &str {
    let file_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();
    file_name
}
