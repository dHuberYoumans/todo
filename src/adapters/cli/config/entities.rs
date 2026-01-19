use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
    pub style: Style,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub todo_db: String,
}

#[derive(Debug, Deserialize)]
pub enum TableStyle {
    Ascii,
    AsciiRounded,
    Modern,
    ModernRounded,
    Markdown,
}

impl From<String> for TableStyle {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "ascii" => TableStyle::Ascii,
            "ascii_rounded" | "ascii-rounded" => TableStyle::AsciiRounded,
            "modern" => TableStyle::Modern,
            "modern_rounded" | "modern-rounded" => TableStyle::ModernRounded,
            "markdown" => TableStyle::Markdown,
            _ => TableStyle::ModernRounded,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Style {
    pub id_length: usize,
    pub due_date_display_format: String,
    pub due_date_input_format: String,
    pub show_due: bool,
    pub show_tag: bool,
    pub sort_by: String,
    pub table: String,
}
