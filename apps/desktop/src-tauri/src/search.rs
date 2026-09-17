use crate::error::ScoopResult;

#[derive(Debug, Clone, Copy)]
pub enum SearchProvider {
    DuckDuckGo,
    Google,
    Bing,
}

impl SearchProvider {
    pub fn from_setting(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "google" => Self::Google,
            "bing" => Self::Bing,
            _ => Self::DuckDuckGo,
        }
    }

    pub fn build_url(self, query: &str) -> String {
        let q = urlencoding::encode(query);
        match self {
            Self::DuckDuckGo => format!("https://duckduckgo.com/?q={q}"),
            Self::Google => format!("https://www.google.com/search?q={q}"),
            Self::Bing => format!("https://www.bing.com/search?q={q}"),
        }
    }
}

pub fn exact_search_url(provider: &str, query: &str) -> ScoopResult<String> {
    Ok(SearchProvider::from_setting(provider).build_url(query))
}
