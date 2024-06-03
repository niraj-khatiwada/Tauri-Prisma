// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lib::prisma;
use lib::tauri_commands::{__cmd__get_users, get_users};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let db = match prisma::PrismaClient::_builder().build().await {
        Ok(db_client) => db_client,
        Err(err) => panic!("[database] {}", err),
    };

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_users])
        .manage(Arc::new(db))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
