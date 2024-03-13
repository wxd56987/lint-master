use comfy_table::Color;
use std::path::Path;

pub fn overflow_text(long_text: &str) -> Vec<String> {
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

pub fn status_emoji<'a>(flag: usize) -> &'a str {
  if flag > 0 {
      "🔴 "
  } else {
      "✅ "
  }
}

pub fn status_color(flag: usize) -> Color {
  if flag > 0 {
      Color::Red
  } else {
      Color::Green
  }
}

pub fn get_file_name(file_path: &str) -> &str {
  let file_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();
  file_name
}

pub fn get_extension(file_path: &str) -> Result<Option<String>, &'static str> {
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

pub fn convert_to_camel_case(contents: &str) -> String {
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