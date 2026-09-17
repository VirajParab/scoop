use std::sync::Arc;

use tauri::{
    AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, Position, Size, State,
};
use tauri_plugin_opener::OpenerExt;

use crate::capture;
use crate::classify;
use crate::clipboard_ext;
use crate::db::library::SaveLibraryInput;
use crate::db::notes::SaveNoteInput;
use crate::db::settings::{self, AppSettings};
use crate::db::Db;
use crate::error::{ScoopError, ScoopResult};
use crate::intent;
use crate::math_engine;
use crate::ocr;
use crate::providers::{self, AskAiRequest};
use crate::search;
use crate::session::{AppState, Region, SelectionSession, SelectionStartedPayload};

pub struct SharedDb(pub Arc<Db>);

#[tauri::command]
pub fn get_session(state: State<AppState>) -> SelectionSession {
    state.session.lock().expect("session").clone()
}

#[tauri::command]
pub fn cancel_selection(state: State<AppState>, app: AppHandle) -> ScoopResult<()> {
    state.clear_session();
    if let Some(w) = app.get_webview_window("overlay") {
        let _ = w.hide();
    }
    if let Some(w) = app.get_webview_window("toolbar") {
        let _ = w.hide();
    }
    Ok(())
}

#[tauri::command]
pub fn start_selection(app: AppHandle) -> ScoopResult<()> {
    show_overlay(&app)
}

pub fn show_overlay(app: &AppHandle) -> ScoopResult<()> {
    let overlay = app
        .get_webview_window("overlay")
        .ok_or_else(|| ScoopError::msg("Overlay window missing"))?;

    // Hide first so we capture the real desktop, not the overlay.
    let _ = overlay.hide();
    // Give the compositor a beat to remove the overlay from the frame.
    std::thread::sleep(std::time::Duration::from_millis(40));

    // Cover the full virtual desktop (both monitors, including Y-offset layouts).
    // One freeze-frame + one window avoids “screen copy on the wrong monitor”.
    let desktop = capture::capture_virtual_desktop()?;
    let x = desktop.window_x;
    let y = desktop.window_y;
    let width = desktop.window_w;
    let height = desktop.window_h;
    let backdrop_path = desktop.path.to_string_lossy().to_string();

    {
        let state = app.state::<AppState>();
        let mut slot = state.desktop_capture.lock().expect("desktop lock");
        if let Some(old) = slot.take() {
            capture::cleanup_capture(&old.path.to_string_lossy());
        }
        *slot = Some(desktop);
    }

    let _ = overlay.set_fullscreen(false);
    let _ = overlay.set_decorations(false);
    let _ = overlay.set_always_on_top(true);
    let _ = overlay.set_skip_taskbar(true);
    let _ = overlay.set_size(Size::Physical(PhysicalSize { width, height }));
    let _ = overlay.set_position(Position::Physical(PhysicalPosition { x, y }));

    overlay.show().map_err(|e| ScoopError::msg(e.to_string()))?;
    let _ = overlay.set_focus();
    // Re-assert geometry after show — some WMs reset size/pos on map.
    let _ = overlay.set_size(Size::Physical(PhysicalSize { width, height }));
    let _ = overlay.set_position(Position::Physical(PhysicalPosition { x, y }));

    // Sync stored window origin to wherever the WM actually placed us.
    if let Ok(pos) = overlay.outer_position() {
        let state = app.state::<AppState>();
        let mut guard = state.desktop_capture.lock().expect("desktop lock");
        if let Some(ref mut desk) = *guard {
            desk.window_x = pos.x;
            desk.window_y = pos.y;
            if let Ok(size) = overlay.outer_size() {
                desk.window_w = size.width;
                desk.window_h = size.height;
            }
        }
    }

    let _ = app.emit(
        "selection-started",
        SelectionStartedPayload { backdrop_path },
    );

    Ok(())
}

#[tauri::command]
pub fn get_overlay_backdrop(state: State<AppState>) -> Option<String> {
    state
        .desktop_capture
        .lock()
        .expect("desktop lock")
        .as_ref()
        .map(|d| d.path.to_string_lossy().to_string())
}

/// Convert overlay-local CSS pixel rect → physical screen rect for capture.
fn client_rect_to_screen(app: &AppHandle, region: &Region) -> ScoopResult<Region> {
    let overlay = app
        .get_webview_window("overlay")
        .ok_or_else(|| ScoopError::msg("Overlay window missing"))?;
    let pos = overlay
        .outer_position()
        .map_err(|e| ScoopError::msg(format!("overlay pos: {e}")))?;
    let scale = overlay
        .scale_factor()
        .map_err(|e| ScoopError::msg(format!("scale: {e}")))?;

    Ok(Region {
        x: pos.x + (region.x as f64 * scale).round() as i32,
        y: pos.y + (region.y as f64 * scale).round() as i32,
        width: (region.width as f64 * scale).round().max(1.0) as u32,
        height: (region.height as f64 * scale).round().max(1.0) as u32,
    })
}

#[tauri::command]
pub fn confirm_selection(
    region: Region,
    state: State<AppState>,
    app: AppHandle,
) -> ScoopResult<SelectionSession> {
    if region.width < 4 || region.height < 4 {
        return Err(ScoopError::msg("Selection too small"));
    }

    let overlay = app
        .get_webview_window("overlay")
        .ok_or_else(|| ScoopError::msg("Overlay window missing"))?;
    let scale = overlay
        .scale_factor()
        .map_err(|e| ScoopError::msg(format!("scale: {e}")))?;
    let inner = overlay
        .inner_size()
        .map_err(|e| ScoopError::msg(format!("inner size: {e}")))?;
    let css_w = inner.width as f64 / scale;
    let css_h = inner.height as f64 / scale;

    // Crop in overlay-local CSS space against the freeze-frame the user saw.
    // Avoids screen-space mismatches that produced empty/corrupt PNGs.
    let path = {
        let desk = state.desktop_capture.lock().expect("desktop lock");
        if let Some(ref desktop) = *desk {
            capture::crop_from_desktop_client(desktop, &region, css_w, css_h)?
        } else {
            let screen = client_rect_to_screen(&app, &region)?;
            capture::capture_region(&screen)?
        }
    };

    let screen = client_rect_to_screen(&app, &region).unwrap_or(region.clone());

    let (ocr_text, ocr_error) = match ocr::recognize_image(&path) {
        Ok(text) => (text, None),
        Err(e) => (String::new(), Some(e.to_string())),
    };
    let content = classify::classify_text(&ocr_text);
    let actions = intent::rank_actions(content);

    // Pin crop to a stable session path so Save always finds the image.
    let held = capture::hold_session_capture(&path)?;
    let preview_data_url = match capture::read_data_url(&held) {
        Ok(url) => Some(url),
        Err(e) => {
            eprintln!("Scoop preview encode failed: {e}");
            None
        }
    };

    let session = SelectionSession {
        capture_path: Some(held.to_string_lossy().to_string()),
        preview_data_url,
        ocr_text,
        ocr_error,
        content_type: content.as_str().to_string(),
        actions,
        region: Some(screen.clone()),
    };

    {
        let mut s = state.session.lock().expect("session");
        if let Some(old) = s.capture_path.take() {
            capture::cleanup_capture(&old);
        }
        *s = session.clone();
    }

    // Drop frozen desktop backdrop after successful crop.
    {
        let mut desk = state.desktop_capture.lock().expect("desktop lock");
        if let Some(old) = desk.take() {
            capture::cleanup_capture(&old.path.to_string_lossy());
        }
    }

    if let Some(w) = app.get_webview_window("overlay") {
        let _ = w.hide();
        let _ = w.set_fullscreen(false);
    }

    // Never fail the whole selection just because toolbar positioning glitched —
    // still emit session so the UI can recover.
    if let Err(e) = position_toolbar(&app, &screen) {
        eprintln!("Scoop toolbar position failed: {e}");
        if let Some(toolbar) = app.get_webview_window("toolbar") {
            let _ = toolbar.show();
            let _ = toolbar.set_focus();
        }
    }
    let _ = app.emit("session-updated", &session);
    Ok(session)
}

fn position_toolbar(app: &AppHandle, region: &Region) -> ScoopResult<()> {
    let toolbar = app
        .get_webview_window("toolbar")
        .ok_or_else(|| ScoopError::msg("Toolbar window missing"))?;

    let tw = 640i32;
    let th = 420i32;
    let mut x = region.x;
    let mut y = region.y + region.height as i32 + 12;

    // Keep the toolbar on the monitor that contains the selection center.
    if let Ok(monitors) = app.available_monitors() {
        let cx = region.x + region.width as i32 / 2;
        let cy = region.y + region.height as i32 / 2;
        if let Some(mon) = monitors.iter().find(|m| {
            let p = m.position();
            let s = m.size();
            cx >= p.x
                && cy >= p.y
                && cx < p.x + s.width as i32
                && cy < p.y + s.height as i32
        }) {
            let p = mon.position();
            let s = mon.size();
            let max_x = p.x + s.width as i32 - tw;
            let max_y = p.y + s.height as i32 - th;
            if y > max_y {
                // Flip above the selection when there is no room below.
                y = region.y - th - 12;
            }
            x = x.clamp(p.x, max_x.max(p.x));
            y = y.clamp(p.y, max_y.max(p.y));
        }
    }

    toolbar
        .set_size(Size::Physical(PhysicalSize {
            width: tw as u32,
            height: th as u32,
        }))
        .map_err(|e| ScoopError::msg(e.to_string()))?;
    toolbar
        .set_position(Position::Physical(PhysicalPosition { x, y }))
        .map_err(|e| ScoopError::msg(e.to_string()))?;
    toolbar.show().map_err(|e| ScoopError::msg(e.to_string()))?;
    toolbar
        .set_focus()
        .map_err(|e| ScoopError::msg(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn update_ocr_text(text: String, state: State<AppState>) -> ScoopResult<SelectionSession> {
    let mut s = state.session.lock().expect("session");
    s.ocr_text = text;
    let content = classify::classify_text(&s.ocr_text);
    s.content_type = content.as_str().to_string();
    s.actions = intent::rank_actions(content);
    Ok(s.clone())
}

#[tauri::command]
pub fn dismiss_toolbar(state: State<AppState>, app: AppHandle) -> ScoopResult<()> {
    state.clear_session();
    if let Some(w) = app.get_webview_window("toolbar") {
        let _ = w.hide();
    }
    Ok(())
}

#[tauri::command]
pub fn action_copy(
    text: Option<String>,
    state: State<AppState>,
    shared: State<SharedDb>,
) -> ScoopResult<()> {
    let session = state.session.lock().expect("session").clone();
    let payload = text.unwrap_or(session.ocr_text.clone());
    clipboard_ext::copy_text(&payload)?;
    let _ = shared.0.add_history(
        "copy",
        Some(&payload),
        None,
        Some(&session.content_type),
    );

    if shared
        .0
        .get_setting("save_copies_to_library")
        .unwrap_or_default()
        == "true"
    {
        let _ = shared.0.save_library_item(SaveLibraryInput {
            title: None,
            collection_name: Some("Inbox".into()),
            tags: None,
            clip_text: Some(payload),
            ocr_text: Some(session.ocr_text),
            capture_path: None,
            content_type: Some(session.content_type),
            include_screenshot: Some(false),
        });
    }
    Ok(())
}

#[tauri::command]
pub fn action_calculate(
    text: Option<String>,
    state: State<AppState>,
    shared: State<SharedDb>,
) -> ScoopResult<math_engine::MathResult> {
    let session = state.session.lock().expect("session").clone();
    let expr = text.unwrap_or(session.ocr_text.clone());
    let result = math_engine::evaluate(&expr)?;
    let _ = shared.0.add_history(
        "calculate",
        Some(&result.expression),
        Some(&result.answer),
        Some("MATH"),
    );
    Ok(result)
}

#[tauri::command]
pub fn action_search(
    text: Option<String>,
    use_ai: bool,
    state: State<AppState>,
    shared: State<SharedDb>,
    app: AppHandle,
) -> ScoopResult<String> {
    let session = state.session.lock().expect("session").clone();
    let mut query = text.unwrap_or(session.ocr_text.clone());
    if use_ai {
        let settings = shared.0.get_all_settings()?;
        if !settings.cloud_processing {
            return Err(ScoopError::msg("Cloud processing disabled"));
        }
        let key = settings::read_api_key().unwrap_or_default();
        let provider =
            providers::provider_from_settings(&settings.ai_provider, &settings.ai_model, &key)?;
        query = provider.rewrite_search_query(&query)?;
    }
    let provider = shared
        .0
        .get_setting("search_provider")
        .unwrap_or_else(|_| "duckduckgo".into());
    let url = search::exact_search_url(&provider, &query)?;
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| ScoopError::msg(e.to_string()))?;
    let _ = shared
        .0
        .add_history("search", Some(&query), None, Some(&session.content_type));
    Ok(query)
}

#[tauri::command]
pub fn action_save_library(
    input: SaveLibraryInput,
    state: State<AppState>,
    shared: State<SharedDb>,
) -> ScoopResult<crate::db::library::LibraryItem> {
    let session = state.session.lock().expect("session").clone();
    let include_shot = input.include_screenshot.unwrap_or(true);
    let capture_path = if include_shot {
        input
            .capture_path
            .or(session.capture_path.clone())
            .filter(|p| !p.is_empty() && std::path::Path::new(p).exists())
    } else {
        None
    };

    if include_shot && capture_path.is_none() {
        return Err(ScoopError::msg(
            "Screenshot file is missing. Select the region again, then Save.",
        ));
    }

    let merged = SaveLibraryInput {
        title: input.title,
        collection_name: input.collection_name.or(Some("Inbox".into())),
        tags: input.tags,
        clip_text: input.clip_text.or(Some(session.ocr_text.clone())),
        ocr_text: input.ocr_text.or(Some(session.ocr_text.clone())),
        capture_path,
        content_type: input.content_type.or(Some(session.content_type.clone())),
        include_screenshot: Some(include_shot),
    };
    eprintln!(
        "Scoop save_library: include_shot={include_shot} path={:?} tags={:?}",
        merged.capture_path, merged.tags
    );
    let item = shared.0.save_library_item(merged)?;
    let _ = shared.0.add_history(
        "save_library",
        Some(&item.title),
        None,
        item.content_type.as_deref(),
    );
    Ok(item)
}

#[tauri::command]
pub fn action_save_note(
    input: SaveNoteInput,
    smart: bool,
    state: State<AppState>,
    shared: State<SharedDb>,
) -> ScoopResult<crate::db::notes::Note> {
    let session = state.session.lock().expect("session").clone();
    let mut merged = SaveNoteInput {
        title: input.title,
        content: input.content.or(Some(session.ocr_text.clone())),
        summary: input.summary,
        ocr_text: input.ocr_text.or(Some(session.ocr_text.clone())),
        tags: input.tags,
        capture_path: input.capture_path.or(session.capture_path.clone()),
        content_type: input.content_type.or(Some(session.content_type.clone())),
        is_smart: Some(smart),
        library_item_id: input.library_item_id,
    };

    if smart {
        let settings = shared.0.get_all_settings()?;
        if settings.cloud_processing {
            if let Ok(key) = settings::read_api_key() {
                if let Ok(provider) = providers::provider_from_settings(
                    &settings.ai_provider,
                    &settings.ai_model,
                    &key,
                ) {
                    if let Ok(draft) =
                        provider.structure_note(&session.ocr_text, &session.content_type)
                    {
                        merged.title = Some(draft.title);
                        merged.summary = Some(draft.summary);
                        merged.content = Some(draft.content);
                        merged.tags = Some(draft.tags);
                        merged.is_smart = Some(true);
                    }
                }
            }
        }
    }

    let note = shared.0.save_note(merged)?;
    let _ = shared.0.add_history(
        "save_note",
        Some(&note.title),
        None,
        note.content_type.as_deref(),
    );
    Ok(note)
}

#[tauri::command]
pub fn action_ask_ai(
    question: String,
    state: State<AppState>,
    shared: State<SharedDb>,
) -> ScoopResult<providers::AskAiResponse> {
    let session = state.session.lock().expect("session").clone();
    let settings = shared.0.get_all_settings()?;
    if !settings.cloud_processing {
        return Err(ScoopError::msg("Cloud processing disabled in Settings"));
    }
    let key = settings::read_api_key().unwrap_or_default();
    let provider =
        providers::provider_from_settings(&settings.ai_provider, &settings.ai_model, &key)?;
    let resp = provider.ask(&AskAiRequest {
        question: question.clone(),
        ocr_text: session.ocr_text.clone(),
        content_type: session.content_type.clone(),
        capture_path: session.capture_path.clone(),
    })?;
    let _ = shared.0.add_history(
        "ask_ai",
        Some(&question),
        Some(&resp.answer),
        Some(&session.content_type),
    );
    Ok(resp)
}

#[tauri::command]
pub fn get_settings(shared: State<SharedDb>) -> ScoopResult<AppSettings> {
    shared.0.get_all_settings()
}

#[tauri::command]
pub fn save_settings(
    settings: AppSettings,
    api_key: Option<String>,
    shared: State<SharedDb>,
) -> ScoopResult<()> {
    shared.0.update_settings(&settings)?;
    if let Some(key) = api_key {
        settings::write_api_key(&key)?;
    }
    Ok(())
}

#[tauri::command]
pub fn list_collections(
    shared: State<SharedDb>,
) -> ScoopResult<Vec<crate::db::library::Collection>> {
    shared.0.list_collections()
}

#[tauri::command]
pub fn create_collection(
    name: String,
    shared: State<SharedDb>,
) -> ScoopResult<crate::db::library::Collection> {
    shared.0.create_collection(&name)
}

#[tauri::command]
pub fn list_library(
    collection_id: Option<String>,
    shared: State<SharedDb>,
) -> ScoopResult<Vec<crate::db::library::LibraryItem>> {
    shared.0.list_library_items(collection_id)
}

#[tauri::command]
pub fn search_library(
    query: String,
    shared: State<SharedDb>,
) -> ScoopResult<Vec<crate::db::library::LibraryItem>> {
    shared.0.search_library(&query)
}

#[tauri::command]
pub fn delete_library_item(id: String, shared: State<SharedDb>) -> ScoopResult<()> {
    shared.0.delete_library_item(&id)
}

#[tauri::command]
pub fn list_notes(shared: State<SharedDb>) -> ScoopResult<Vec<crate::db::notes::Note>> {
    shared.0.list_notes()
}

#[tauri::command]
pub fn search_notes(
    query: String,
    shared: State<SharedDb>,
) -> ScoopResult<Vec<crate::db::notes::Note>> {
    shared.0.search_notes(&query)
}

#[tauri::command]
pub fn delete_note(id: String, shared: State<SharedDb>) -> ScoopResult<()> {
    shared.0.delete_note(&id)
}

#[tauri::command]
pub fn list_history(
    shared: State<SharedDb>,
) -> ScoopResult<Vec<crate::db::history::HistoryItem>> {
    shared.0.list_history(100)
}

#[tauri::command]
pub fn delete_history(id: String, shared: State<SharedDb>) -> ScoopResult<()> {
    shared.0.delete_history(&id)
}

#[tauri::command]
pub fn clear_history(shared: State<SharedDb>) -> ScoopResult<()> {
    shared.0.clear_history()
}

#[tauri::command]
pub fn read_capture_data_url(path: String) -> ScoopResult<String> {
    capture::read_data_url(std::path::Path::new(&path))
}

#[tauri::command]
pub fn open_main_window(app: AppHandle) -> ScoopResult<()> {
    if let Some(w) = app.get_webview_window("main") {
        w.show().map_err(|e| ScoopError::msg(e.to_string()))?;
        w.set_focus().map_err(|e| ScoopError::msg(e.to_string()))?;
    }
    Ok(())
}

pub fn register_hotkey(app: &AppHandle, shortcut: &str) -> ScoopResult<()> {
    use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

    let _ = app.global_shortcut().unregister_all();

    let sc: Shortcut = shortcut
        .parse()
        .map_err(|e| ScoopError::msg(format!("HOTKEY_CONFLICT: invalid shortcut: {e}")))?;

    let app_handle = app.clone();
    app.global_shortcut()
        .on_shortcut(sc, move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                let _ = show_overlay(&app_handle);
            }
        })
        .map_err(|e| ScoopError::msg(format!("HOTKEY_CONFLICT: {e}")))?;
    Ok(())
}

#[tauri::command]
pub fn rebind_hotkey(
    shortcut: String,
    app: AppHandle,
    shared: State<SharedDb>,
) -> ScoopResult<()> {
    register_hotkey(&app, &shortcut)?;
    shared.0.set_setting("hotkey", &shortcut)?;
    Ok(())
}

#[tauri::command]
pub fn ocr_available() -> bool {
    ocr::is_available()
}
