use std::sync::OnceLock;

use regex::Regex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Text,
    Code,
    Math,
    Table,
    Chart,
    Image,
    UiScreenshot,
    Product,
    Document,
    Unknown,
}

impl ContentType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Text => "TEXT",
            Self::Code => "CODE",
            Self::Math => "MATH",
            Self::Table => "TABLE",
            Self::Chart => "CHART",
            Self::Image => "IMAGE",
            Self::UiScreenshot => "UI_SCREENSHOT",
            Self::Product => "PRODUCT",
            Self::Document => "DOCUMENT",
            Self::Unknown => "UNKNOWN",
        }
    }
}

fn math_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        // Note: ₹ must not be written as \₹ (invalid escape in the regex crate).
        Regex::new(r"[√^×÷%₹$]|\d+\s*[+\-*/]\s*\d+|sqrt\s*\(|\d+%")
            .expect("math classification regex")
    })
}

fn code_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)(error:|exception|traceback|stack trace|fn |def |class |import |const |let |var |\{\s*$|;\s*$|localhost:\d+|::)",
        )
        .expect("code classification regex")
    })
}

fn table_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\|.+\|").expect("table classification regex"))
}

/// Local heuristics when vision is unavailable.
pub fn classify_text(ocr: &str) -> ContentType {
    let t = ocr.trim();
    if t.is_empty() {
        return ContentType::Unknown;
    }

    let math_hits = math_re().find_iter(t).count();
    let code_hits = code_re().find_iter(t).count();

    if math_hits >= 1 && t.len() < 120 && code_hits <= 2 {
        return ContentType::Math;
    }
    // Prefer math when expression-like and short
    if math_hits >= 1 && t.lines().count() <= 3 && code_hits == 0 {
        return ContentType::Math;
    }
    if code_hits >= 1 || (t.contains('{') && t.contains('}')) {
        return ContentType::Code;
    }
    if table_re().is_match(t) {
        return ContentType::Table;
    }
    if t.len() > 280 {
        return ContentType::Document;
    }
    ContentType::Text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_rupee_math() {
        assert_eq!(classify_text("₹2,40,000 × 8.5%"), ContentType::Math);
    }

    #[test]
    fn classifies_code_error() {
        assert_eq!(
            classify_text("ERROR: connection refused localhost:5432"),
            ContentType::Code
        );
    }
}
