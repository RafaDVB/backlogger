use crate::db::settings;
use crate::error::{AppError, AppResult};
use crate::models::Settings;
use crate::state::AppState;
use tauri::State;

const MAX_UI_STATE_BYTES: usize = 100_000;

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

#[tauri::command]
pub fn get_ui_state(state: State<'_, AppState>) -> AppResult<Option<String>> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    settings::get(&conn, settings::UI_STATE)
}

#[tauri::command]
pub fn set_ui_state(state: State<'_, AppState>, json: String) -> AppResult<()> {
    if json.len() > MAX_UI_STATE_BYTES {
        return Err(AppError::Invalid("the saved view is too large".into()));
    }
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    settings::set(&conn, settings::UI_STATE, &json)
}
