use crate::constants::{LintResult, CONGRATULATE};
use crate::utils::{get_file_name, overflow_text, status_color, status_emoji};
use comfy_table::*;

pub struct TsTable {
    pub lint_check: LintResult,
    pub svg_check: LintResult,
    pub todo_check: LintResult,
    pub color_check: LintResult,
    pub image_alt_check: LintResult,
    pub a_rel_check: LintResult,
    pub file_line_check: LintResult,
    pub console_log_check: LintResult,
}

pub struct GoTable {
    pub lint_check: LintResult,
    pub todo_check: LintResult,
    pub file_line_check: LintResult,
}

pub struct DrawTable {}

impl DrawTable {
    pub fn draw_ts_table(file_path: &str, lint: TsTable) {
        let mut table = Table::new();

        let TsTable {
            lint_check,
            svg_check,
            todo_check,
            color_check,
            image_alt_check,
            a_rel_check,
            file_line_check,
            console_log_check,
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

        let mut lines_console_log: Vec<String> = console_log_check
            .result
            .iter()
            .map(|check| overflow_text(check).join("\n"))
            .collect();

        let mut lines_image_alt: Vec<String> = image_alt_check
            .result
            .iter()
            .map(|check| overflow_text(check).join("\n"))
            .collect();

        let mut lines_a_rel: Vec<String> = a_rel_check
            .result
            .iter()
            .map(|check| overflow_text(check).join("\n"))
            .collect();

        let mut lines_file_check: Vec<String> = file_line_check
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

        for s in lines_console_log.iter_mut() {
            *s = format!("🤔 {}", s);
        }

        for s in lines_image_alt.iter_mut() {
            *s = format!("🤔 {}", s);
        }

        for s in lines_a_rel.iter_mut() {
            *s = format!("🤔 {}", s);
        }

        for s in lines_file_check.iter_mut() {
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

        let cell_console_log_r = if lines_console_log.len() > 0 {
            lines_console_log.join("\n")
        } else {
            CONGRATULATE.to_string()
        };

        let cell_image_alt_r = if lines_image_alt.len() > 0 {
            lines_image_alt.join("\n")
        } else {
            CONGRATULATE.to_string()
        };

        let cell_a_rel_r = if lines_a_rel.len() > 0 {
            lines_a_rel.join("\n")
        } else {
            CONGRATULATE.to_string()
        };

        let cell_file_line_r = if lines_file_check.len() > 0 {
            lines_file_check.join("\n")
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
        let cell_console_log = Cell::new(cell_console_log_r);
        let cell_image_alt = Cell::new(cell_image_alt_r);
        let cell_a_rel = Cell::new(cell_a_rel_r);
        let cell_file_line = Cell::new(cell_file_line_r);
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
                Cell::new("🍉 CONSOLE_LOG").fg(Color::Yellow),
                cell_console_log,
                Cell::new(console_log_check.errors.to_string()).fg(status_color(console_log_check.errors)),
                Cell::new(status_emoji(console_log_check.errors)),
            ])
            .add_row(vec![
                Cell::new("🍎 IMAGE_ALT").fg(Color::Yellow),
                cell_image_alt,
                Cell::new(image_alt_check.errors.to_string())
                    .fg(status_color(image_alt_check.errors)),
                Cell::new(status_emoji(image_alt_check.errors)),
            ])
            .add_row(vec![
                Cell::new("🍍 A_REL").fg(Color::Yellow),
                cell_a_rel,
                Cell::new(a_rel_check.errors.to_string()).fg(status_color(a_rel_check.errors)),
                Cell::new(status_emoji(a_rel_check.errors)),
            ])
            .add_row(vec![
                Cell::new("🍌 FILE_LINES").fg(Color::Yellow),
                cell_file_line,
                Cell::new(file_line_check.errors.to_string())
                    .fg(status_color(file_line_check.errors)),
                Cell::new(status_emoji(file_line_check.errors)),
            ])
            .add_row(vec![
                Cell::new("🎨 COLOR").fg(Color::Yellow),
                cell_color,
                Cell::new(color_check.errors.to_string()).fg(status_color(color_check.errors)),
                Cell::new(status_emoji(color_check.errors)),
            ]);

        println!("{table}");
    }

    pub fn draw_go_table<'a>(file_path: &str, lint: GoTable) {
        let mut table = Table::new();

        let GoTable {
            lint_check,
            todo_check,
            file_line_check,
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

        let mut lines_file_check: Vec<String> = file_line_check
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

        for s in lines_file_check.iter_mut() {
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

        let cell_file_line_r = if lines_file_check.len() > 0 {
            lines_file_check.join("\n")
        } else {
            CONGRATULATE.to_string()
        };

        let cell_lint = Cell::new(cell_lint_r);
        let cell_todo = Cell::new(cell_todo_r);
        let cell_file_line = Cell::new(cell_file_line_r);

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
            ])
            .add_row(vec![
                Cell::new("🍌 FILE_LINES").fg(Color::Yellow),
                cell_file_line,
                Cell::new(file_line_check.errors.to_string())
                    .fg(status_color(file_line_check.errors)),
                Cell::new(status_emoji(file_line_check.errors)),
            ]);

        println!("{table}");
    }
}
