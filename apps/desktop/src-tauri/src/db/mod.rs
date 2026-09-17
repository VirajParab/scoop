pub mod history;
pub mod library;
pub mod notes;
pub mod settings;

use std::sync::Mutex;

use rusqlite::Connection;

use crate::error::ScoopResult;
use crate::paths::db_path;

pub struct Db {
    pub conn: Mutex<Connection>,
}

impl Db {
    pub fn open() -> ScoopResult<Self> {
        let path = db_path()?;
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> ScoopResult<()> {
        let conn = self.conn.lock().expect("db lock");
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
              key TEXT PRIMARY KEY,
              value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS collections (
              id TEXT PRIMARY KEY,
              name TEXT NOT NULL UNIQUE,
              is_system INTEGER NOT NULL DEFAULT 0,
              sort_order INTEGER NOT NULL DEFAULT 0,
              created_at TEXT NOT NULL,
              updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS library_items (
              id TEXT PRIMARY KEY,
              collection_id TEXT NOT NULL REFERENCES collections(id),
              title TEXT NOT NULL,
              item_type TEXT NOT NULL,
              clip_text TEXT,
              ocr_text TEXT,
              tags TEXT NOT NULL DEFAULT '[]',
              screenshot_path TEXT,
              content_type TEXT,
              source_url TEXT,
              note_id TEXT,
              created_at TEXT NOT NULL,
              updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS notes (
              id TEXT PRIMARY KEY,
              title TEXT NOT NULL,
              content TEXT NOT NULL DEFAULT '',
              summary TEXT,
              key_points TEXT,
              important_facts TEXT,
              ocr_text TEXT,
              source_url TEXT,
              tags TEXT NOT NULL DEFAULT '[]',
              screenshot_path TEXT,
              library_item_id TEXT,
              content_type TEXT,
              created_at TEXT NOT NULL,
              updated_at TEXT NOT NULL,
              is_smart INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS history (
              id TEXT PRIMARY KEY,
              action TEXT NOT NULL,
              input_summary TEXT,
              output_summary TEXT,
              screenshot_path TEXT,
              content_type TEXT,
              created_at TEXT NOT NULL,
              meta_json TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_library_collection ON library_items(collection_id);
            CREATE INDEX IF NOT EXISTS idx_library_created ON library_items(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_notes_created ON notes(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_history_created ON history(created_at DESC);

            CREATE VIRTUAL TABLE IF NOT EXISTS library_fts USING fts5(
              title, clip_text, ocr_text, tags, collection_name, item_id UNINDEXED
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
              title, content, ocr_text, tags, summary, note_id UNINDEXED
            );
            "#,
        )?;

        // Seed Inbox
        let now = chrono::Utc::now().to_rfc3339();
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM collections WHERE name = 'Inbox')",
            [],
            |r| r.get(0),
        )?;
        if !exists {
            conn.execute(
                "INSERT INTO collections (id, name, is_system, sort_order, created_at, updated_at)
                 VALUES (?1, 'Inbox', 1, 0, ?2, ?2)",
                rusqlite::params![uuid::Uuid::new_v4().to_string(), now],
            )?;
        }

        // Default settings
        let defaults = [
            ("hotkey", "Super+Shift+Space"),
            ("search_provider", "duckduckgo"),
            ("ai_provider", "openai"),
            ("ai_model", "gpt-4o-mini"),
            ("history_retention_days", "30"),
            ("save_copies_to_library", "false"),
            ("cloud_processing", "true"),
        ];
        for (k, v) in defaults {
            conn.execute(
                "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
                rusqlite::params![k, v],
            )?;
        }

        Ok(())
    }
}
