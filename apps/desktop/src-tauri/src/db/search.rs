use serde::Serialize;

use super::Db;
use crate::error::ScoopResult;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchHit {
    pub kind: String,
    pub id: String,
    pub title: String,
    pub snippet: String,
    pub tags: Vec<String>,
    pub screenshot_path: Option<String>,
    pub linked_kind: Option<String>,
    pub linked_id: Option<String>,
    pub linked_title: Option<String>,
    pub created_at: String,
}

impl Db {
    /// Search library + notes, then surface linked siblings for easy lineage navigation.
    pub fn search_all(&self, query: &str) -> ScoopResult<Vec<SearchHit>> {
        let q = query.trim();
        let mut hits: Vec<SearchHit> = Vec::new();
        let mut seen = std::collections::HashSet::<String>::new();

        let library = if q.is_empty() {
            self.list_library_items(None)?
        } else {
            self.search_library(q)?
        };
        for item in library {
            let key = format!("library:{}", item.id);
            if !seen.insert(key) {
                continue;
            }
            let snippet = item
                .clip_text
                .clone()
                .or(item.ocr_text.clone())
                .unwrap_or_default();
            hits.push(SearchHit {
                kind: "library".into(),
                id: item.id.clone(),
                title: item.title.clone(),
                snippet: snippet.chars().take(160).collect(),
                tags: item.tags.clone(),
                screenshot_path: item.screenshot_path.clone(),
                linked_kind: item.note_id.as_ref().map(|_| "note".into()),
                linked_id: item.note_id.clone(),
                linked_title: item.linked_note_title.clone(),
                created_at: item.created_at.clone(),
            });

            // Also surface the linked note so one query finds both sides of the lineage.
            if let Some(nid) = item.note_id {
                let nkey = format!("note:{nid}");
                if seen.insert(nkey) {
                    if let Some(note) = self.get_note(&nid)? {
                        hits.push(SearchHit {
                            kind: "note".into(),
                            id: note.id,
                            title: note.title,
                            snippet: note
                                .summary
                                .or(Some(note.content.clone()))
                                .unwrap_or_default()
                                .chars()
                                .take(160)
                                .collect(),
                            tags: note.tags,
                            screenshot_path: note.screenshot_path,
                            linked_kind: Some("library".into()),
                            linked_id: Some(item.id.clone()),
                            linked_title: Some(item.title.clone()),
                            created_at: note.created_at,
                        });
                    }
                }
            }
        }

        let notes = if q.is_empty() {
            self.list_notes()?
        } else {
            self.search_notes(q)?
        };
        for note in notes {
            let key = format!("note:{}", note.id);
            if !seen.insert(key) {
                continue;
            }
            hits.push(SearchHit {
                kind: "note".into(),
                id: note.id.clone(),
                title: note.title.clone(),
                snippet: note
                    .summary
                    .clone()
                    .or(Some(note.content.clone()))
                    .unwrap_or_default()
                    .chars()
                    .take(160)
                    .collect(),
                tags: note.tags.clone(),
                screenshot_path: note.screenshot_path.clone(),
                linked_kind: note.library_item_id.as_ref().map(|_| "library".into()),
                linked_id: note.library_item_id.clone(),
                linked_title: note.linked_library_title.clone(),
                created_at: note.created_at.clone(),
            });

            if let Some(lid) = note.library_item_id {
                let lkey = format!("library:{lid}");
                if seen.insert(lkey) {
                    if let Some(item) = self.get_library_item(&lid)? {
                        hits.push(SearchHit {
                            kind: "library".into(),
                            id: item.id,
                            title: item.title,
                            snippet: item
                                .clip_text
                                .or(item.ocr_text)
                                .unwrap_or_default()
                                .chars()
                                .take(160)
                                .collect(),
                            tags: item.tags,
                            screenshot_path: item.screenshot_path,
                            linked_kind: Some("note".into()),
                            linked_id: Some(note.id.clone()),
                            linked_title: Some(note.title.clone()),
                            created_at: item.created_at,
                        });
                    }
                }
            }
        }

        hits.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        hits.truncate(100);
        Ok(hits)
    }
}
