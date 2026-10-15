use crate::db::lists;
use crate::error::{AppError, AppResult};
use crate::models::{Genre, Platform, Rating};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn list_ratings(state: State<'_, AppState>) -> AppResult<Vec<Rating>> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    lists::ratings(&conn)
}

#[tauri::command]
pub fn list_genres(state: State<'_, AppState>) -> AppResult<Vec<Genre>> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    lists::genres(&conn)
}

#[tauri::command]
pub fn list_platforms(state: State<'_, AppState>) -> AppResult<Vec<Platform>> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    lists::platforms(&conn)
}

#[tauri::command]
pub fn create_platform(state: State<'_, AppState>, name: String) -> AppResult<Platform> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    lists::create_platform(&conn, &name)
}
