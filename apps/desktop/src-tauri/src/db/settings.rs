use rusqlite::params;
use serde::{Deserialize, Serialize};

use super::Db;
use crate::error::{ScoopError, ScoopResult};

const SERVICE: &str = "scoop";
const USER: &str = "api_key";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub hotkey: String,
    pub search_provider: String,
    pub ai_provider: String,
    pub ai_model: String,
    pub history_retention_days: String,
    pub save_copies_to_library: bool,
    pub cloud_processing: bool,
    pub has_api_key: bool,
}

impl Db {
    pub fn get_setting(&self, key: &str) -> ScoopResult<String> {
        let conn = self.conn.lock().expect("db");
        conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |r| r.get(0),
        )
        .map_err(|_| ScoopError::msg(format!("Missing setting: {key}")))
    }

    pub fn set_setting(&self, key: &str, value: &str) -> ScoopResult<()> {
        let conn = self.conn.lock().expect("db");
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_all_settings(&self) -> ScoopResult<AppSettings> {
        Ok(AppSettings {
            hotkey: self
                .get_setting("hotkey")
                .unwrap_or_else(|_| "Super+Shift+Space".into()),
            search_provider: self
                .get_setting("search_provider")
                .unwrap_or_else(|_| "duckduckgo".into()),
            ai_provider: self
                .get_setting("ai_provider")
                .unwrap_or_else(|_| "openai".into()),
            ai_model: self
                .get_setting("ai_model")
                .unwrap_or_else(|_| "gpt-4o-mini".into()),
            history_retention_days: self
                .get_setting("history_retention_days")
                .unwrap_or_else(|_| "30".into()),
            save_copies_to_library: self
                .get_setting("save_copies_to_library")
                .unwrap_or_else(|_| "false".into())
                == "true",
            cloud_processing: self
                .get_setting("cloud_processing")
                .unwrap_or_else(|_| "true".into())
                == "true",
            has_api_key: read_api_key().ok().filter(|s| !s.is_empty()).is_some(),
        })
    }

    pub fn update_settings(&self, settings: &AppSettings) -> ScoopResult<()> {
        self.set_setting("hotkey", &settings.hotkey)?;
        self.set_setting("search_provider", &settings.search_provider)?;
        self.set_setting("ai_provider", &settings.ai_provider)?;
        self.set_setting("ai_model", &settings.ai_model)?;
        self.set_setting("history_retention_days", &settings.history_retention_days)?;
        self.set_setting(
            "save_copies_to_library",
            if settings.save_copies_to_library {
                "true"
            } else {
                "false"
            },
        )?;
        self.set_setting(
            "cloud_processing",
            if settings.cloud_processing {
                "true"
            } else {
                "false"
            },
        )?;
        Ok(())
    }
}

pub fn read_api_key() -> ScoopResult<String> {
    let entry = keyring::Entry::new(SERVICE, USER).map_err(|e| ScoopError::msg(e.to_string()))?;
    entry
        .get_password()
        .map_err(|e| ScoopError::msg(format!("No API key: {e}")))
}

pub fn write_api_key(key: &str) -> ScoopResult<()> {
    let entry = keyring::Entry::new(SERVICE, USER).map_err(|e| ScoopError::msg(e.to_string()))?;
    if key.is_empty() {
        let _ = entry.delete_credential();
        return Ok(());
    }
    entry
        .set_password(key)
        .map_err(|e| ScoopError::msg(e.to_string()))?;
    Ok(())
}
