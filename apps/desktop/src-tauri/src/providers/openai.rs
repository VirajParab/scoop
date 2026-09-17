use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::{json, Value};

use super::{AiProvider, AskAiRequest, AskAiResponse, SmartNoteDraft};
use crate::error::{ScoopError, ScoopResult};

pub struct OpenAiProvider {
    api_key: String,
    model: String,
    client: reqwest::blocking::Client,
}

impl OpenAiProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::blocking::Client::new(),
        }
    }

    fn chat(&self, messages: Value) -> ScoopResult<String> {
        if self.api_key.is_empty() {
            return Err(ScoopError::msg(
                "PROVIDER_UNCONFIGURED: set an API key in Settings",
            ));
        }
        let body = json!({
            "model": self.model,
            "messages": messages,
            "temperature": 0.2,
        });
        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            if status.as_u16() == 429 {
                return Err(ScoopError::msg(format!("PROVIDER_RATE_LIMIT: {text}")));
            }
            return Err(ScoopError::msg(format!("NETWORK_ERROR: {status} {text}")));
        }
        let v: Value = resp.json()?;
        let content = v["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        Ok(content)
    }
}

impl AiProvider for OpenAiProvider {
    fn id(&self) -> &str {
        "openai"
    }

    fn ask(&self, req: &AskAiRequest) -> ScoopResult<AskAiResponse> {
        let mut user_content = vec![json!({
            "type": "text",
            "text": format!(
                "Content type: {}\nOCR text:\n{}\n\nUser question: {}",
                req.content_type, req.ocr_text, req.question
            )
        })];

        if let Some(path) = &req.capture_path {
            if let Ok(bytes) = std::fs::read(path) {
                let b64 = STANDARD.encode(bytes);
                user_content.push(json!({
                    "type": "image_url",
                    "image_url": { "url": format!("data:image/png;base64,{b64}") }
                }));
            }
        }

        let messages = json!([
            {
                "role": "system",
                "content": "You are Scoop, an AI assistant for a Linux desktop selection tool. Be concise and practical."
            },
            { "role": "user", "content": user_content }
        ]);
        let answer = self.chat(messages)?;
        Ok(AskAiResponse { answer })
    }

    fn structure_note(&self, ocr_text: &str, content_type: &str) -> ScoopResult<SmartNoteDraft> {
        let messages = json!([
            {
                "role": "system",
                "content": "Return ONLY valid JSON with keys: title, summary, content, tags (array of strings)."
            },
            {
                "role": "user",
                "content": format!("Content type: {content_type}\nText:\n{ocr_text}")
            }
        ]);
        let raw = self.chat(messages)?;
        let cleaned = raw
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();
        let v: Value = serde_json::from_str(cleaned)
            .map_err(|e| ScoopError::msg(format!("Bad smart note JSON: {e}")))?;
        Ok(SmartNoteDraft {
            title: v["title"].as_str().unwrap_or("Untitled").to_string(),
            summary: v["summary"].as_str().unwrap_or("").to_string(),
            content: v["content"].as_str().unwrap_or(ocr_text).to_string(),
            tags: v["tags"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
        })
    }

    fn rewrite_search_query(&self, text: &str) -> ScoopResult<String> {
        let messages = json!([
            {
                "role": "system",
                "content": "Rewrite the selection into a single concise web search query. Return only the query text."
            },
            { "role": "user", "content": text }
        ]);
        Ok(self.chat(messages)?.trim().to_string())
    }
}
