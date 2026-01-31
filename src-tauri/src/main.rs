// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use anymaku_lib::websocket;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            websocket::start_server_connection,
            websocket::set_overlay_ignore_mouse
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}