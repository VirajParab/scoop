use std::path::Path;

use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Db;
use crate::error::{ScoopError, ScoopResult};
use crate::paths::screenshots_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub is_system: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryItem {
    pub id: String,
    pub collection_id: String,
    pub collection_name: Option<String>,
    pub title: String,
    pub item_type: String,
    pub clip_text: Option<String>,
    pub ocr_text: Option<String>,
    pub tags: Vec<String>,
    pub screenshot_path: Option<String>,
    pub content_type: Option<String>,
    pub note_id: Option<String>,
    pub linked_note_title: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveLibraryInput {
    pub title: Option<String>,
    pub collection_name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub clip_text: Option<String>,
    pub ocr_text: Option<String>,
    pub capture_path: Option<String>,
    pub content_type: Option<String>,
    /// When false, skip persisting the screenshot (text-only save). Default true.
    pub include_screenshot: Option<bool>,
    /// Optional existing note to link (lineage).
    pub note_id: Option<String>,
}

pub fn path_search_tokens(path: Option<&str>) -> String {
    let Some(path) = path else {
        return String::new();
    };
    let p = Path::new(path);
    let mut parts = Vec::new();
    if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
        parts.push(name.to_string());
    }
    if let Some(stem) = p.file_stem().and_then(|n| n.to_str()) {
        parts.push(stem.to_string());
    }
    parts.push("screenshot".into());
    parts.push("image".into());
    parts.join(" ")
}

fn map_library_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<LibraryItem> {
    let tags_json: String = r.get(7)?;
    let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
    Ok(LibraryItem {
        id: r.get(0)?,
        collection_id: r.get(1)?,
        collection_name: r.get(2)?,
        title: r.get(3)?,
        item_type: r.get(4)?,
        clip_text: r.get(5)?,
        ocr_text: r.get(6)?,
        tags,
        screenshot_path: r.get(8)?,
        content_type: r.get(9)?,
        note_id: r.get(10)?,
        linked_note_title: r.get(11)?,
        created_at: r.get(12)?,
    })
}

const LIBRARY_SELECT: &str = "
    SELECT li.id, li.collection_id, c.name, li.title, li.item_type, li.clip_text, li.ocr_text,
           li.tags, li.screenshot_path, li.content_type, li.note_id, n.title, li.created_at
    FROM library_items li
    JOIN collections c ON c.id = li.collection_id
    LEFT JOIN notes n ON n.id = li.note_id
";

impl Db {
    pub fn list_collections(&self) -> ScoopResult<Vec<Collection>> {
        let conn = self.conn.lock().expect("db");
        let mut stmt = conn.prepare(
            "SELECT id, name, is_system FROM collections ORDER BY sort_order, name",
        )?;
        let rows = stmt
            .query_map([], |r| {
                Ok(Collection {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    is_system: r.get::<_, i64>(2)? == 1,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn create_collection(&self, name: &str) -> ScoopResult<Collection> {
        let conn = self.conn.lock().expect("db");
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO collections (id, name, is_system, sort_order, created_at, updated_at)
             VALUES (?1, ?2, 0, 100, ?3, ?3)",
            params![id, name, now],
        )?;
        Ok(Collection {
            id,
            name: name.to_string(),
            is_system: false,
        })
    }

    fn resolve_collection_id(conn: &rusqlite::Connection, name: Option<&str>) -> ScoopResult<String> {
        let target = name.unwrap_or("Inbox");
        let id: String = conn.query_row(
            "SELECT id FROM collections WHERE name = ?1",
            params![target],
            |r| r.get(0),
        )?;
        Ok(id)
    }

    pub fn get_library_item(&self, id: &str) -> ScoopResult<Option<LibraryItem>> {
        let conn = self.conn.lock().expect("db");
        let mut stmt = conn.prepare(&format!("{LIBRARY_SELECT} WHERE li.id = ?1"))?;
        let mut rows = stmt.query_map(params![id], map_library_row)?;
        Ok(rows.next().transpose()?)
    }

    pub fn save_library_item(&self, input: SaveLibraryInput) -> ScoopResult<LibraryItem> {
        let conn = self.conn.lock().expect("db");
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let collection_id =
            Self::resolve_collection_id(&conn, input.collection_name.as_deref())?;
        let collection_name: String = conn.query_row(
            "SELECT name FROM collections WHERE id = ?1",
            params![collection_id],
            |r| r.get(0),
        )?;

        let clip = input.clip_text.clone().or_else(|| input.ocr_text.clone());
        let title = input
            .title
            .filter(|t| !t.trim().is_empty())
            .unwrap_or_else(|| {
                clip.as_deref()
                    .and_then(|c| c.lines().next())
                    .map(|l| l.chars().take(60).collect())
                    .filter(|s: &String| !s.is_empty())
                    .unwrap_or_else(|| format!("Capture {now}"))
            });

        let mut screenshot_path = None;
        let mut item_type = "text".to_string();
        if let Some(src) = input.capture_path.as_ref() {
            let src_path = Path::new(src);
            if !src_path.exists() {
                return Err(ScoopError::msg(format!(
                    "Screenshot not found at {src}"
                )));
            }
            let dest = screenshots_dir()?.join(format!("{id}.png"));
            std::fs::copy(src_path, &dest).map_err(|e| {
                ScoopError::msg(format!("Failed to save screenshot: {e}"))
            })?;
            if !dest.exists() || dest.metadata().map(|m| m.len()).unwrap_or(0) == 0 {
                return Err(ScoopError::msg("Screenshot copy produced an empty file"));
            }
            screenshot_path = Some(dest.to_string_lossy().to_string());
            item_type = if clip.as_ref().map(|c| !c.is_empty()).unwrap_or(false) {
                "mixed".into()
            } else {
                "screenshot".into()
            };
        }

        let tags = input.tags.unwrap_or_default();
        let tags_json = serde_json::to_string(&tags)?;
        let note_id = input.note_id.clone();

        conn.execute(
            "INSERT INTO library_items
             (id, collection_id, title, item_type, clip_text, ocr_text, tags, screenshot_path, content_type, note_id, created_at, updated_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?11)",
            params![
                id,
                collection_id,
                title,
                item_type,
                clip,
                input.ocr_text,
                tags_json,
                screenshot_path,
                input.content_type,
                note_id,
                now
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
            "INSERT INTO library_fts (title, clip_text, ocr_text, tags, collection_name, item_id)
             VALUES (?1,?2,?3,?4,?5,?6)",
            params![
                title,
                clip.clone().unwrap_or_default(),
                input.ocr_text.clone().unwrap_or_default(),
                fts_tags,
                collection_name,
                id
            ],
        )?;

        // Bidirectional lineage: note → library
        if let Some(ref nid) = note_id {
            conn.execute(
                "UPDATE notes SET library_item_id = ?1, updated_at = ?2 WHERE id = ?3",
                params![id, now, nid],
            )?;
        }

        let linked_note_title = note_id.as_ref().and_then(|nid| {
            conn.query_row(
                "SELECT title FROM notes WHERE id = ?1",
                params![nid],
                |r| r.get(0),
            )
            .ok()
        });

        Ok(LibraryItem {
            id,
            collection_id,
            collection_name: Some(collection_name),
            title,
            item_type,
            clip_text: clip,
            ocr_text: input.ocr_text,
            tags,
            screenshot_path,
            content_type: input.content_type,
            note_id,
            linked_note_title,
            created_at: now,
        })
    }

    #[allow(dead_code)]
    pub fn link_library_to_note(&self, library_id: &str, note_id: &str) -> ScoopResult<()> {
        let conn = self.conn.lock().expect("db");
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE library_items SET note_id = ?1, updated_at = ?2 WHERE id = ?3",
            params![note_id, now, library_id],
        )?;
        conn.execute(
            "UPDATE notes SET library_item_id = ?1, updated_at = ?2 WHERE id = ?3",
            params![library_id, now, note_id],
        )?;
        // Prefer sharing the library screenshot on the note when note has none.
        let lib_shot: Option<String> = conn
            .query_row(
                "SELECT screenshot_path FROM library_items WHERE id = ?1",
                params![library_id],
                |r| r.get(0),
            )
            .ok()
            .flatten();
        if let Some(path) = lib_shot {
            conn.execute(
                "UPDATE notes SET screenshot_path = COALESCE(screenshot_path, ?1) WHERE id = ?2",
                params![path, note_id],
            )?;
        }
        Ok(())
    }

    pub fn list_library_items(
        &self,
        collection_id: Option<String>,
    ) -> ScoopResult<Vec<LibraryItem>> {
        let conn = self.conn.lock().expect("db");
        let mut sql = LIBRARY_SELECT.to_string();
        if collection_id.is_some() {
            sql.push_str(" WHERE li.collection_id = ?1");
        }
        sql.push_str(" ORDER BY li.created_at DESC LIMIT 200");

        let mut stmt = conn.prepare(&sql)?;
        let rows = if let Some(cid) = collection_id {
            stmt.query_map(params![cid], map_library_row)?
                .collect::<Result<Vec<_>, _>>()?
        } else {
            stmt.query_map([], map_library_row)?
                .collect::<Result<Vec<_>, _>>()?
        };
        Ok(rows)
    }

    pub fn search_library(&self, query: &str) -> ScoopResult<Vec<LibraryItem>> {
        let conn = self.conn.lock().expect("db");
        let q = query.trim();
        if q.is_empty() {
            return self.list_library_items(None);
        }
        let mut stmt = conn.prepare(&format!(
            "{LIBRARY_SELECT}
             JOIN library_fts f ON f.item_id = li.id
             WHERE library_fts MATCH ?1
             ORDER BY rank
             LIMIT 100"
        ))?;
        let rows = stmt
            .query_map(params![q], map_library_row)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn delete_library_item(&self, id: &str) -> ScoopResult<()> {
        let conn = self.conn.lock().expect("db");
        let path: Option<String> = conn
            .query_row(
                "SELECT screenshot_path FROM library_items WHERE id = ?1",
                params![id],
                |r| r.get(0),
            )
            .ok()
            .flatten();
        // Unlink notes but keep note content; clear shared path only if it points at this file.
        conn.execute(
            "UPDATE notes SET library_item_id = NULL WHERE library_item_id = ?1",
            params![id],
        )?;
        conn.execute("DELETE FROM library_fts WHERE item_id = ?1", params![id])?;
        conn.execute("DELETE FROM library_items WHERE id = ?1", params![id])?;
        if let Some(p) = path {
            let still_used: bool = conn
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM notes WHERE screenshot_path = ?1)",
                    params![p],
                    |r| r.get(0),
                )
                .unwrap_or(false);
            if !still_used {
                let _ = std::fs::remove_file(p);
            }
        }
        Ok(())
    }
}
