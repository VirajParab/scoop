use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::capture::DesktopCapture;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectionSession {
    pub capture_path: Option<String>,
    /// Small PNG preview as a data URL for the toolbar (set on confirm).
    pub preview_data_url: Option<String>,
    pub ocr_text: String,
    /// Empty when OCR succeeded; otherwise a user-facing reason.
    pub ocr_error: Option<String>,
    pub content_type: String,
    pub actions: Vec<String>,
    pub region: Option<Region>,
    /// Lineage: library item saved from this selection (if any).
    pub library_item_id: Option<String>,
    /// Lineage: note saved from this selection (if any).
    pub note_id: Option<String>,
    /// True when the selection screenshot was copied to the clipboard.
    pub clipboard_image_copied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionStartedPayload {
    pub backdrop_path: String,
}

#[derive(Default)]
pub struct AppState {
    pub session: Mutex<SelectionSession>,
    pub desktop_capture: Mutex<Option<DesktopCapture>>,
}

impl AppState {
    pub fn clear_session(&self) {
        let mut s = self.session.lock().expect("session lock");
        if let Some(path) = s.capture_path.take() {
            let _ = std::fs::remove_file(&path);
        }
        *s = SelectionSession::default();

        let mut desk = self.desktop_capture.lock().expect("desktop lock");
        if let Some(cap) = desk.take() {
            let _ = std::fs::remove_file(&cap.path);
        }
    }
}
