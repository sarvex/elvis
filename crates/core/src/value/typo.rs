/// Font Style
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum FontStyle {
    /// Italic Font
    Italic,
    /// Nomal Font
    Normal,
}

impl ToString for FontStyle {
    fn to_string(&self) -> String {
        match self {
            FontStyle::Italic => "italic",
            FontStyle::Normal => "normal",
        }
        .to_string()
    }
}

impl From<&str> for FontStyle {
    fn from(s: &str) -> FontStyle {
        match s.to_lowercase().as_str() {
            "italic" => FontStyle::Italic,
            _ => FontStyle::Normal,
        }
    }
}

/// Font Family
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum FontFamily {
    /// Helvetica Neue Font
    Mix(Box<FontFamily>, Box<FontFamily>),
    /// Helvetica Font
    Helvetica,
    /// Neue Font
    Neue,
    /// Arial Font
    Arial,
}

impl ToString for FontFamily {
    fn to_string(&self) -> String {
        match self {
            FontFamily::Mix(a, b) => format!("\"{} {}\"", a.to_string(), b.to_string()),
            FontFamily::Helvetica => "Helvetica".to_string(),
            FontFamily::Neue => "Neue".to_string(),
            FontFamily::Arial => "Arial".to_string(),
        }
    }
}

impl From<&str> for FontFamily {
    fn from(s: &str) -> FontFamily {
        match s.to_lowercase().as_str() {
            "Helvetica" => FontFamily::Helvetica,
            "Neue" => FontFamily::Neue,
            _ => FontFamily::Arial,
        }
    }
}
