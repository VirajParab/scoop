use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::library::path_search_tokens;
use super::Db;
use crate::error::ScoopResult;
use crate::paths::screenshots_dir;

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
    pub library_item_id: Option<String>,
    pub linked_library_title: Option<String>,
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

fn map_note_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<Note> {
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
        library_item_id: r.get(9)?,
        linked_library_title: r.get(10)?,
        created_at: r.get(11)?,
    })
}

const NOTE_SELECT: &str = "
    SELECT n.id, n.title, n.content, n.summary, n.ocr_text, n.tags,
           COALESCE(n.screenshot_path, li.screenshot_path),
           n.content_type, n.is_smart, n.library_item_id, li.title, n.created_at
    FROM notes n
    LEFT JOIN library_items li ON li.id = n.library_item_id
";

impl Db {
    pub fn get_note(&self, id: &str) -> ScoopResult<Option<Note>> {
        let conn = self.conn.lock().expect("db");
        let mut stmt = conn.prepare(&format!("{NOTE_SELECT} WHERE n.id = ?1"))?;
        let mut rows = stmt.query_map(params![id], map_note_row)?;
        Ok(rows.next().transpose()?)
    }

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

        let library_item_id = input.library_item_id.clone();

        // Prefer sharing the linked library screenshot (lineage, no duplicate file).
        let mut screenshot_path: Option<String> = None;
        if let Some(ref lid) = library_item_id {
            screenshot_path = conn
                .query_row(
                    "SELECT screenshot_path FROM library_items WHERE id = ?1",
                    params![lid],
                    |r| r.get(0),
                )
                .ok()
                .flatten();
        }

        if screenshot_path.is_none() {
            if let Some(src) = input.capture_path.as_ref() {
                if std::path::Path::new(src).exists() {
                    let dest = screenshots_dir()?.join(format!("note-{id}.png"));
                    std::fs::copy(src, &dest)?;
                    screenshot_path = Some(dest.to_string_lossy().to_string());
                }
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
                library_item_id,
                input.content_type,
                now,
                is_smart
            ],
        )?;

        let fts_tags = format!(
            "{} {}",
            tags.join(" "),
            path_search_tokens(screenshot_path.as_deref())
        )
        .trim()
        .to_string();

        conn.execute(
            "INSERT INTO notes_fts (title, content, ocr_text, tags, summary, note_id)
             VALUES (?1,?2,?3,?4,?5,?6)",
            params![
                title,
                content,
                ocr.clone().unwrap_or_default(),
                fts_tags,
                input.summary.clone().unwrap_or_default(),
                id
            ],
        )?;

        // Bidirectional lineage: library → note
        if let Some(ref lid) = library_item_id {
            conn.execute(
                "UPDATE library_items SET note_id = ?1, updated_at = ?2 WHERE id = ?3",
                params![id, now, lid],
            )?;
        }

        let linked_library_title = library_item_id.as_ref().and_then(|lid| {
            conn.query_row(
                "SELECT title FROM library_items WHERE id = ?1",
                params![lid],
                |r| r.get(0),
            )
            .ok()
        });

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
            library_item_id,
            linked_library_title,
            created_at: now,
        })
    }

    pub fn list_notes(&self) -> ScoopResult<Vec<Note>> {
        let conn = self.conn.lock().expect("db");
        let mut stmt =
            conn.prepare(&format!("{NOTE_SELECT} ORDER BY n.created_at DESC LIMIT 200"))?;
        let rows = stmt
            .query_map([], map_note_row)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn search_notes(&self, query: &str) -> ScoopResult<Vec<Note>> {
        let conn = self.conn.lock().expect("db");
        let q = query.trim();
        if q.is_empty() {
            return self.list_notes();
        }
        let mut stmt = conn.prepare(&format!(
            "{NOTE_SELECT}
             JOIN notes_fts f ON f.note_id = n.id
             WHERE notes_fts MATCH ?1
             ORDER BY rank
             LIMIT 100"
        ))?;
        let rows = stmt
            .query_map(params![q], map_note_row)?
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

        conn.execute(
            "UPDATE library_items SET note_id = NULL WHERE note_id = ?1",
            params![id],
        )?;
        conn.execute("DELETE FROM notes_fts WHERE note_id = ?1", params![id])?;
        conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;

        if let Some(p) = path {
            let still_used: bool = conn
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM library_items WHERE screenshot_path = ?1)
                     OR EXISTS(SELECT 1 FROM notes WHERE screenshot_path = ?1)",
                    params![p],
                    |r| r.get(0),
                )
                .unwrap_or(true);
            if !still_used {
                let _ = std::fs::remove_file(p);
            }
        }
        Ok(())
    }
}
