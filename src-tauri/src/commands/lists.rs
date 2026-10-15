use crate::db::lists;
use crate::error::{AppError, AppResult};
use crate::models::Rating;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn list_ratings(state: State<'_, AppState>) -> AppResult<Vec<Rating>> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    lists::ratings(&conn)
}