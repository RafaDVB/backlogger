use crate::db::{list_edit, lists};
use crate::error::{AppError, AppResult};
use crate::models::{Genre, Platform, Rating, DeletePreview};
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

#[tauri::command]
pub fn create_genre(state: State<'_, AppState>, name: String) -> AppResult<Genre> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    list_edit::create_genre(&conn, &name)
}

#[tauri::command]
pub fn rename_genre(state: State<'_, AppState>, id: i64, name: String) -> AppResult<Genre> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    list_edit::rename_genre(&conn, id, &name)
}

#[tauri::command]
pub fn preview_delete_genre(state: State<'_, AppState>, id: i64) -> AppResult<DeletePreview> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    list_edit::preview_delete_genre(&conn, id)
}

#[tauri::command]
pub fn delete_genre(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    list_edit::delete_genre(&conn, id)
}

#[tauri::command]
pub fn create_rating(
    state: State<'_, AppState>,
    label: String,
    color: Option<String>,
) -> AppResult<Rating> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    list_edit::create_rating(&conn, &label, color)
}

#[tauri::command]
pub fn update_rating(
    state: State<'_, AppState>,
    id: i64,
    label: String,
    color: Option<String>,
) -> AppResult<Rating> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    list_edit::update_rating(&conn, id, &label, color)
}

#[tauri::command]
pub fn reorder_ratings(state: State<'_, AppState>, ids: Vec<i64>) -> AppResult<Vec<Rating>> {
    let mut conn = state.db.lock().map_err(|_| AppError::Lock)?;
    list_edit::reorder_ratings(&mut conn, &ids)?;
    lists::ratings(&conn)
}

#[tauri::command]
pub fn preview_delete_rating(state: State<'_, AppState>, id: i64) -> AppResult<DeletePreview> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    list_edit::preview_delete_rating(&conn, id)
}

#[tauri::command]
pub fn delete_rating(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    list_edit::delete_rating(&conn, id)
}

#[tauri::command]
pub fn rename_platform(state: State<'_, AppState>, id: i64, name: String) -> AppResult<Platform> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    list_edit::rename_platform(&conn, id, &name)
}

#[tauri::command]
pub fn merge_platform(state: State<'_, AppState>, from_id: i64, to_id: i64) -> AppResult<()> {
    let mut conn = state.db.lock().map_err(|_| AppError::Lock)?;
    list_edit::merge_platform(&mut conn, from_id, to_id)
}

#[tauri::command]
pub fn preview_delete_platform(state: State<'_, AppState>, id: i64) -> AppResult<DeletePreview> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    list_edit::preview_delete_platform(&conn, id)
}

#[tauri::command]
pub fn delete_platform(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let mut conn = state.db.lock().map_err(|_| AppError::Lock)?;
    list_edit::delete_platform(&mut conn, id)
}