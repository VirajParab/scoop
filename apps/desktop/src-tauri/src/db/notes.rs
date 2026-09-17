use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Db;
use crate::error::ScoopResult;
use crate::paths::media_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub ocr_text: Option<String>,
    pub tags: Vec<String>,
    pub screenshot_path: Option<String>,
    pub content_type: Option<String>,
    pub is_smart: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveNoteInput {
    pub title: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub ocr_text: Option<String>,
    pub tags: Option<Vec<String>>,
    pub capture_path: Option<String>,
    pub content_type: Option<String>,
    pub is_smart: Option<bool>,
    pub library_item_id: Option<String>,
}

impl Db {
    pub fn save_note(&self, input: SaveNoteInput) -> ScoopResult<Note> {
        let conn = self.conn.lock().expect("db");
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let content = input.content.clone().unwrap_or_default();
        let ocr = input.ocr_text.clone();
        let title = input
            .title
            .filter(|t| !t.trim().is_empty())
            .unwrap_or_else(|| {
                ocr.as_deref()
                    .or(Some(content.as_str()))
                    .and_then(|c| c.lines().next())
                    .map(|l| l.chars().take(60).collect())
                    .filter(|s: &String| !s.is_empty())
                    .unwrap_or_else(|| format!("Note {now}"))
            });

        let mut screenshot_path = None;
        if let Some(src) = input.capture_path.as_ref() {
            if std::path::Path::new(src).exists() {
                let dest = media_dir()?.join(format!("note-{id}.png"));
                std::fs::copy(src, &dest)?;
                screenshot_path = Some(dest.to_string_lossy().to_string());
            }
        }

        let tags = input.tags.unwrap_or_default();
        let tags_json = serde_json::to_string(&tags)?;
        let is_smart = input.is_smart.unwrap_or(false) as i64;

        conn.execute(
            "INSERT INTO notes
             (id, title, content, summary, ocr_text, tags, screenshot_path, library_item_id, content_type, created_at, updated_at, is_smart)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?10,?11)",
            params![
                id,
                title,
                content,
                input.summary,
                ocr,
                tags_json,
                screenshot_path,
                input.library_item_id,
                input.content_type,
                now,
                is_smart
            ],
        )?;

        conn.execute(
            "INSERT INTO notes_fts (title, content, ocr_text, tags, summary, note_id)
             VALUES (?1,?2,?3,?4,?5,?6)",
            params![
                title,
                content,
                ocr.clone().unwrap_or_default(),
                tags.join(" "),
                input.summary.clone().unwrap_or_default(),
                id
            ],
        )?;

        Ok(Note {
            id,
            title,
            content,
            summary: input.summary,
            ocr_text: ocr,
            tags,
            screenshot_path,
            content_type: input.content_type,
            is_smart: is_smart == 1,
            created_at: now,
        })
    }

    pub fn list_notes(&self) -> ScoopResult<Vec<Note>> {
        let conn = self.conn.lock().expect("db");
        let mut stmt = conn.prepare(
            "SELECT id, title, content, summary, ocr_text, tags, screenshot_path, content_type, is_smart, created_at
             FROM notes ORDER BY created_at DESC LIMIT 200",
        )?;
        let rows = stmt
            .query_map([], |r| {
                let tags_json: String = r.get(5)?;
                let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
                Ok(Note {
                    id: r.get(0)?,
                    title: r.get(1)?,
                    content: r.get(2)?,
                    summary: r.get(3)?,
                    ocr_text: r.get(4)?,
                    tags,
                    screenshot_path: r.get(6)?,
                    content_type: r.get(7)?,
                    is_smart: r.get::<_, i64>(8)? == 1,
                    created_at: r.get(9)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn search_notes(&self, query: &str) -> ScoopResult<Vec<Note>> {
        let conn = self.conn.lock().expect("db");
        let q = query.trim();
        if q.is_empty() {
            return self.list_notes();
        }
        let mut stmt = conn.prepare(
            "SELECT n.id, n.title, n.content, n.summary, n.ocr_text, n.tags, n.screenshot_path, n.content_type, n.is_smart, n.created_at
             FROM notes_fts f
             JOIN notes n ON n.id = f.note_id
             WHERE notes_fts MATCH ?1
             ORDER BY rank
             LIMIT 100",
        )?;
        let rows = stmt
            .query_map(params![q], |r| {
                let tags_json: String = r.get(5)?;
                let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
                Ok(Note {
                    id: r.get(0)?,
                    title: r.get(1)?,
                    content: r.get(2)?,
                    summary: r.get(3)?,
                    ocr_text: r.get(4)?,
                    tags,
                    screenshot_path: r.get(6)?,
                    content_type: r.get(7)?,
                    is_smart: r.get::<_, i64>(8)? == 1,
                    created_at: r.get(9)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn delete_note(&self, id: &str) -> ScoopResult<()> {
        let conn = self.conn.lock().expect("db");
        let path: Option<String> = conn
            .query_row(
                "SELECT screenshot_path FROM notes WHERE id = ?1",
                params![id],
                |r| r.get(0),
            )
            .ok()
            .flatten();
        conn.execute("DELETE FROM notes_fts WHERE note_id = ?1", params![id])?;
        conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
        if let Some(p) = path {
            let _ = std::fs::remove_file(p);
        }
        Ok(())
    }
}
