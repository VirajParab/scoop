pub mod openai;

use serde::{Deserialize, Serialize};

use crate::error::{ScoopError, ScoopResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AskAiRequest {
    pub question: String,
    pub ocr_text: String,
    pub content_type: String,
    pub capture_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AskAiResponse {
    pub answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartNoteDraft {
    pub title: String,
    pub summary: String,
    pub content: String,
    pub tags: Vec<String>,
}

pub trait AiProvider: Send + Sync {
    #[allow(dead_code)]
    fn id(&self) -> &str;
    fn ask(&self, req: &AskAiRequest) -> ScoopResult<AskAiResponse>;
    fn structure_note(&self, ocr_text: &str, content_type: &str) -> ScoopResult<SmartNoteDraft>;
    fn rewrite_search_query(&self, text: &str) -> ScoopResult<String>;
}

pub fn provider_from_settings(
    provider: &str,
    model: &str,
    api_key: &str,
) -> ScoopResult<Box<dyn AiProvider>> {
    match provider {
        "openai" | "" => Ok(Box::new(openai::OpenAiProvider::new(
            api_key.to_string(),
            model.to_string(),
        ))),
        other => Err(ScoopError::msg(format!(
            "PROVIDER_UNCONFIGURED: unknown provider {other}"
        ))),
    }
}
