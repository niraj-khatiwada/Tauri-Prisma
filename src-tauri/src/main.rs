// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lib::{domain::DbState, prisma};
// use lib::tauri_commands::{__cmd__get_users, __specta__fn__get_users, get_users};
use std::sync::Arc;
use tauri_plugin_log::{Target as LogTarget, TargetKind as LogTargetKind};

#[tauri::command]
#[specta::specta]
async fn get_users(db: DbState<'_>) -> Result<Vec<prisma::user::Data>, String> {
    db.user()
        .find_many(vec![])
        .exec()
        .await
        .map_err(|err| err.to_string())
}

#[cfg(debug_assertions)]
const LOG_TARGETS: [LogTarget; 1] = [LogTarget::new(LogTargetKind::Stdout)];

#[cfg(not(debug_assertions))]
const LOG_TARGETS: [LogTarget; 0] = [];

#[tokio::main]
async fn main() {
    let db = match prisma::PrismaClient::_builder().build().await {
        Ok(db_client) => db_client,
        Err(err) => panic!("[database] {}", err),
    };

    let invoke_handler = {
        let builder =
            tauri_specta::ts::builder().commands(tauri_specta::collect_commands![get_users]);

        #[cfg(debug_assertions)]
        let builder = builder.path("../src/types/bindings.ts");

        builder.build().unwrap()
    };

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets(LOG_TARGETS)
                .build(),
        )
        .invoke_handler(invoke_handler)
        .manage(Arc::new(db))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
