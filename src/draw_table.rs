use crate::constants::{LintResult, CONGRATULATE};
use crate::utils::{get_file_name, overflow_text, status_color, status_emoji};
use comfy_table::*;

trait CheckResult {
    fn result(&self) -> Vec<String>;
}

pub struct TsTable {
    pub lint_check: LintResult,
    pub svg_check: LintResult,
    pub todo_check: LintResult,
    pub color_check: LintResult,
}

pub struct GoTable {
    pub lint_check: LintResult,
    pub todo_check: LintResult,
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

    pub fn draw_go_table<'a>(file_path: &str, lint: GoTable) {
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
    
}
