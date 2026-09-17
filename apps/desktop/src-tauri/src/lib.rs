mod capture;
mod classify;
mod clipboard_ext;
mod commands;
mod db;
mod error;
mod intent;
mod math_engine;
mod ocr;
mod paths;
mod providers;
mod search;
mod session;

use std::sync::Arc;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

use commands::SharedDb;
use db::Db;
use session::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = Db::open().expect("open scoop database");
    let _ = db.sweep_history();
    let shared = SharedDb(Arc::new(db));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(AppState::default())
        .manage(shared)
        .invoke_handler(tauri::generate_handler![
            commands::get_session,
            commands::cancel_selection,
            commands::start_selection,
            commands::confirm_selection,
            commands::update_ocr_text,
            commands::apply_edited_capture,
            commands::dismiss_toolbar,
            commands::action_copy,
            commands::action_calculate,
            commands::action_search,
            commands::action_save_library,
            commands::action_save_note,
            commands::action_ask_ai,
            commands::get_settings,
            commands::save_settings,
            commands::list_collections,
            commands::create_collection,
            commands::list_library,
            commands::search_library,
            commands::search_all,
            commands::delete_library_item,
            commands::list_notes,
            commands::search_notes,
            commands::delete_note,
            commands::list_history,
            commands::delete_history,
            commands::clear_history,
            commands::read_capture_data_url,
            commands::open_main_window,
            commands::rebind_hotkey,
            commands::ocr_available,
            commands::get_overlay_backdrop,
        ])
        .setup(|app| {
            // Navigate overlay/toolbar to their routes via URL hash
            if let Some(overlay) = app.get_webview_window("overlay") {
                let _ = overlay.eval("window.location.hash = '#/overlay'");
            }
            if let Some(toolbar) = app.get_webview_window("toolbar") {
                let _ = toolbar.eval("window.location.hash = '#/toolbar'");
            }

            // Tray
            let show_i = MenuItem::with_id(app, "show", "Open Scoop", true, None::<&str>)?;
            let select_i =
                MenuItem::with_id(app, "select", "Select (Scoop)", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &select_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("Scoop")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    "show" => {
                        let _ = commands::open_main_window(app.clone());
                    }
                    "select" => {
                        let _ = commands::show_overlay(app);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        let _ = commands::open_main_window(app.clone());
                    }
                })
                .build(app)?;

            // Register hotkey from settings
            let hotkey = {
                let shared = app.state::<SharedDb>();
                shared
                    .0
                    .get_setting("hotkey")
                    .unwrap_or_else(|_| "Super+Shift+Space".into())
            };
            if let Err(e) = commands::register_hotkey(app.handle(), &hotkey) {
                eprintln!("Scoop hotkey registration failed: {e}");
                // Fallback try without Super if needed
                let _ = commands::register_hotkey(app.handle(), "Ctrl+Shift+Space");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Scoop");
}
