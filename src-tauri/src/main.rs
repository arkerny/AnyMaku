// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anymaku_lib::{websocket, window};
use std::sync::Mutex;

fn main() {
    tauri::Builder::default()
        .manage(websocket::ConnectionState {
            token: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            websocket::start_server_connection,
            websocket::stop_server_connection,
            window::set_overlay_ignore_mouse
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}