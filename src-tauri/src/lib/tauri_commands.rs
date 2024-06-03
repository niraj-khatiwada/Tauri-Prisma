use crate::{domain::DbState, prisma};

#[tauri::command]
pub async fn get_users(db: DbState<'_>) -> Result<Vec<prisma::user::Data>, String> {
    db.user()
        .find_many(vec![])
        .exec()
        .await
        .map_err(|err| err.to_string())
}
