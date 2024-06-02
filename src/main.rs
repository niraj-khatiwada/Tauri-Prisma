// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::{Target as LogTarget, TargetKind as LogTargetKind};

#[cfg(debug_assertions)]
const LOG_TARGETS: [LogTarget; 1] = [LogTarget::new(LogTargetKind::Stdout)];

#[cfg(not(debug_assertions))]
const LOG_TARGETS: [LogTarget; 0] = [];

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets(LOG_TARGETS)
                .build(),
        )
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
