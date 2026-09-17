use chrono::{Duration, Utc};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Db;
use crate::error::ScoopResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryItem {
    pub id: String,
    pub action: String,
    pub input_summary: Option<String>,
    pub output_summary: Option<String>,
    pub content_type: Option<String>,
    pub created_at: String,
}

impl Db {
    pub fn add_history(
        &self,
        action: &str,
        input: Option<&str>,
        output: Option<&str>,
        content_type: Option<&str>,
    ) -> ScoopResult<()> {
        let conn = self.conn.lock().expect("db");
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let input_summary = input.map(|s| s.chars().take(240).collect::<String>());
        let output_summary = output.map(|s| s.chars().take(240).collect::<String>());
        conn.execute(
            "INSERT INTO history (id, action, input_summary, output_summary, content_type, created_at)
             VALUES (?1,?2,?3,?4,?5,?6)",
            params![id, action, input_summary, output_summary, content_type, now],
        )?;
        Ok(())
    }

    pub fn list_history(&self, limit: i64) -> ScoopResult<Vec<HistoryItem>> {
        let conn = self.conn.lock().expect("db");
        let mut stmt = conn.prepare(
            "SELECT id, action, input_summary, output_summary, content_type, created_at
             FROM history ORDER BY created_at DESC LIMIT ?1",
        )?;
        let rows = stmt
            .query_map(params![limit], |r| {
                Ok(HistoryItem {
                    id: r.get(0)?,
                    action: r.get(1)?,
                    input_summary: r.get(2)?,
                    output_summary: r.get(3)?,
                    content_type: r.get(4)?,
                    created_at: r.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn delete_history(&self, id: &str) -> ScoopResult<()> {
        let conn = self.conn.lock().expect("db");
        conn.execute("DELETE FROM history WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn clear_history(&self) -> ScoopResult<()> {
        let conn = self.conn.lock().expect("db");
        conn.execute("DELETE FROM history", [])?;
        Ok(())
    }

    pub fn sweep_history(&self) -> ScoopResult<u64> {
        let days: i64 = self
            .get_setting("history_retention_days")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);
        let cutoff = (Utc::now() - Duration::days(days)).to_rfc3339();
        let conn = self.conn.lock().expect("db");
        let n = conn.execute("DELETE FROM history WHERE created_at < ?1", params![cutoff])?;
        Ok(n as u64)
    }
}
