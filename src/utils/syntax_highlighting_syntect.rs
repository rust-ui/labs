#[cfg(feature = "ssr")]
use std::sync::OnceLock;

#[cfg(feature = "ssr")]
use syntect::easy::HighlightLines;
#[cfg(feature = "ssr")]
use syntect::highlighting::{Style, ThemeSet};
#[cfg(feature = "ssr")]
use syntect::parsing::SyntaxSet;
#[cfg(feature = "ssr")]
use syntect::util::LinesWithEndings;

#[cfg(feature = "ssr")]
static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
#[cfg(feature = "ssr")]
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

#[cfg(feature = "ssr")]
fn get_syntax_set() -> &'static SyntaxSet {
    SYNTAX_SET.get_or_init(|| SyntaxSet::load_defaults_newlines())
}

#[cfg(feature = "ssr")]
fn get_theme_set() -> &'static ThemeSet {
    THEME_SET.get_or_init(ThemeSet::load_defaults)
}

#[cfg(not(feature = "ssr"))]
use syntect::easy::HighlightLines;
#[cfg(not(feature = "ssr"))]
use syntect::highlighting::{Style, ThemeSet};
#[cfg(not(feature = "ssr"))]
use syntect::parsing::SyntaxSet;
#[cfg(not(feature = "ssr"))]
use syntect::util::LinesWithEndings;
#[cfg(not(feature = "ssr"))]
use std::sync::OnceLock;

#[cfg(not(feature = "ssr"))]
static WASM_SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
#[cfg(not(feature = "ssr"))]
static WASM_THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

pub fn detect_language_from_filename(filename: &str) -> Option<&str> {
    let extension = filename.split('.').last()?;
    match extension.to_lowercase().as_str() {
        "rs" => Some("rust"),
        "js" | "mjs" => Some("javascript"),
        "ts" => Some("typescript"),
        "json" => Some("json"),
        "html" => Some("html"),
        "css" => Some("css"),
        "scss" | "sass" => Some("scss"),
        "py" => Some("python"),
        "go" => Some("go"),
        "java" => Some("java"),
        "cpp" | "cc" | "cxx" => Some("cpp"),
        "c" => Some("c"),
        "h" | "hpp" => Some("c"),
        "xml" => Some("xml"),
        "yaml" | "yml" => Some("yaml"),
        "toml" => Some("toml"),
        "md" => Some("markdown"),
        "sh" | "bash" => Some("bash"),
        "sql" => Some("sql"),
        _ => None,
    }
}

#[cfg(feature = "ssr")]
pub fn highlight_code(code: &str, language: Option<&str>, filename: Option<&str>) -> String {
    let syntax_set = get_syntax_set();
    let theme_set = get_theme_set();

    // Try to get theme - fallback to base16-ocean if not available
    let theme = theme_set
        .themes
        .get("base16-ocean.dark")
        .or_else(|| theme_set.themes.get("InspiredGitHub"))
        .or_else(|| theme_set.themes.values().next())
        .expect("No themes available");

    // Determine language
    let lang = language
        .or_else(|| filename.and_then(detect_language_from_filename))
        .unwrap_or("plain");

    // Try to find syntax definition
    let syntax = syntax_set
        .find_syntax_by_name(lang)
        .or_else(|| syntax_set.find_syntax_by_extension(lang))
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut html_output = String::new();

    for line in LinesWithEndings::from(code) {
        let ranges = highlighter
            .highlight_line(line, syntax_set)
            .unwrap_or_default();

        for (style, text) in ranges {
            let css_class = style_to_css_class(&style);
            if css_class.is_empty() {
                html_output.push_str(&html_escape::encode_text(text));
            } else {
                html_output.push_str(&format!(
                    "<span class=\"{}\">{}</span>",
                    css_class,
                    html_escape::encode_text(text)
                ));
            }
        }
    }

    html_output
}

#[cfg(not(feature = "ssr"))]
fn get_wasm_syntax_set() -> &'static SyntaxSet {
    WASM_SYNTAX_SET.get_or_init(|| SyntaxSet::load_defaults_newlines())
}

#[cfg(not(feature = "ssr"))]
fn get_wasm_theme_set() -> &'static ThemeSet {
    WASM_THEME_SET.get_or_init(ThemeSet::load_defaults)
}

#[cfg(not(feature = "ssr"))]
pub fn highlight_code(code: &str, language: Option<&str>, filename: Option<&str>) -> String {
    let syntax_set = get_wasm_syntax_set();
    let theme_set = get_wasm_theme_set();

    // Try to get theme - fallback to base16-ocean if not available
    let theme = theme_set
        .themes
        .get("base16-ocean.dark")
        .or_else(|| theme_set.themes.get("InspiredGitHub"))
        .or_else(|| theme_set.themes.values().next())
        .expect("No themes available");

    // Determine language
    let lang = language
        .or_else(|| filename.and_then(detect_language_from_filename))
        .unwrap_or("plain");

    // Try to find syntax definition using built-in syntect language detection
    let syntax = syntax_set
        .find_syntax_by_name(lang)
        .or_else(|| syntax_set.find_syntax_by_extension(lang))
        .or_else(|| {
            // Try mapping our language names to syntect's expectations
            match lang {
                "javascript" | "js" => syntax_set.find_syntax_by_name("JavaScript"),
                "rust" => syntax_set.find_syntax_by_name("Rust"),
                "toml" => syntax_set.find_syntax_by_name("TOML"),
                _ => None,
            }
        })
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut html_output = String::new();

    for line in LinesWithEndings::from(code) {
        let ranges = highlighter
            .highlight_line(line, syntax_set)
            .unwrap_or_default();

        for (style, text) in ranges {
            let css_class = wasm_style_to_css_class(&style);
            if css_class.is_empty() {
                html_output.push_str(&html_escape::encode_text(text));
            } else {
                html_output.push_str(&format!(
                    "<span class=\"{}\">{}</span>",
                    css_class,
                    html_escape::encode_text(text)
                ));
            }
        }
    }

    html_output
}

#[cfg(not(feature = "ssr"))]
fn wasm_style_to_css_class(style: &Style) -> String {
    let mut classes = Vec::new();

    // Map foreground colors to semantic classes
    let fg = style.foreground;
    let color_class = match (fg.r, fg.g, fg.b) {
        // Keywords (usually blue/purple)
        (r, g, b) if b > 200 && r < 100 && g < 150 => "syntax-keyword",
        // Strings (usually green)
        (r, g, b) if g > 180 && r < 150 && b < 150 => "syntax-string",
        // Comments (usually gray)
        (r, g, b) if r > 100 && g > 100 && b > 100 && r.abs_diff(g) < 30 && g.abs_diff(b) < 30 => {
            "syntax-comment"
        }
        // Numbers (usually orange/red)
        (r, g, b) if r > 200 && g > 100 && b < 100 => "syntax-number",
        // Functions (usually yellow/orange)
        (r, g, b) if r > 200 && g > 150 && b < 150 => "syntax-function",
        // Types (usually cyan/light blue)
        (r, g, b) if r < 150 && g > 150 && b > 150 => "syntax-type",
        _ => "",
    };

    if !color_class.is_empty() {
        classes.push(color_class);
    }

    classes.join(" ")
}


#[cfg(feature = "ssr")]
fn style_to_css_class(style: &Style) -> String {
    let mut classes = Vec::new();

    // Map foreground colors to semantic classes
    let fg = style.foreground;
    let color_class = match (fg.r, fg.g, fg.b) {
        // Keywords (usually blue/purple)
        (r, g, b) if b > 200 && r < 100 && g < 150 => "syntax-keyword",
        // Strings (usually green)
        (r, g, b) if g > 180 && r < 150 && b < 150 => "syntax-string",
        // Comments (usually gray)
        (r, g, b) if r > 100 && g > 100 && b > 100 && r.abs_diff(g) < 30 && g.abs_diff(b) < 30 => {
            "syntax-comment"
        }
        // Numbers (usually orange/red)
        (r, g, b) if r > 200 && g > 100 && b < 100 => "syntax-number",
        // Functions (usually yellow/orange)
        (r, g, b) if r > 200 && g > 150 && b < 150 => "syntax-function",
        // Types (usually cyan/light blue)
        (r, g, b) if r < 150 && g > 150 && b > 150 => "syntax-type",
        _ => "",
    };

    if !color_class.is_empty() {
        classes.push(color_class);
    }

    if style
        .font_style
        .contains(syntect::highlighting::FontStyle::BOLD)
    {
        classes.push("syntax-bold");
    }
    if style
        .font_style
        .contains(syntect::highlighting::FontStyle::ITALIC)
    {
        classes.push("syntax-italic");
    }
    if style
        .font_style
        .contains(syntect::highlighting::FontStyle::UNDERLINE)
    {
        classes.push("syntax-underline");
    }

    classes.join(" ")
}
