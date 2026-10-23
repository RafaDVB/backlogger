use crate::db::settings;
use crate::error::{AppError, AppResult};
use crate::models::Settings;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> AppResult<Settings> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    settings::all(&conn)
}

#[tauri::command]
pub fn set_rawg_api_key(state: State<'_, AppState>, key: String) -> AppResult<Settings> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    settings::set(&conn, settings::RAWG_API_KEY, &key)?;
    settings::all(&conn)
}