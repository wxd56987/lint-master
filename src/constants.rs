pub const TODO_SEARCH: &'static str = "TODO";
pub const TODO_IGNORE_SEARCH: &'static str = "IGNORE";
pub const CONGRATULATE: &'static str = "✨ Congratulate all passed 🎉🎉🎉";
pub const WELCOME: &'static str = r#"
    __     ____ _   __ ______   __  ___ ___    _____ ______ ______ ____ 
   / /    /  _// | / //_  __/  /  |/  //   |  / ___//_  __// ____// __ \
  / /     / / /  |/ /  / /    / /|_/ // /| |  \__ \  / /  / __/  / /_/ /
 / /___ _/ / / /|  /  / /    / /  / // ___ | ___/ / / /  / /___ / _  _/ 
/_____//___//_/ |_/  /_/    /_/  /_//_/  |_|/____/ /_/  /_____//_/ |_|  
                                                                        
"#;

pub struct LintResult {
    pub errors: usize,
    pub result: Vec<String>,
}

pub const RE_LINT_TS: &'static str = r"^\s*\d+:\d+\s+error.*";
pub const RE_LINT_GO: &'static str = r".*?:(\d+:\d+:\s.*?(?:\n\s+.*?)+)";
pub const RE_MATCH_COLOR: &'static str = r"#[0-9a-fA-F]{6}";
pub const RE_TSX_THEME_FILE: &'static str = "apps/identity-hub/config/theme.ts";

pub const SVG_ATTRIBUTE_NAMES: [&str; 35] = [
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
