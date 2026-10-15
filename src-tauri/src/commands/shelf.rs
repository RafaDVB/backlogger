use crate::db::shelf;
use crate::error::{AppError, AppResult};
use crate::models::{RemoveResult, ShelfEntry};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn get_shelf(state: State<'_, AppState>) -> AppResult<Vec<ShelfEntry>> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    shelf::get(&conn)
}

#[tauri::command]
pub fn place_on_shelf(state: State<'_, AppState>, game_id: i64, slot: i64) -> AppResult<()> {
    let mut conn = state.db.lock().map_err(|_| AppError::Lock)?;
    shelf::place(&mut conn, game_id, slot)
}

#[tauri::command]
pub fn move_shelf_slot(state: State<'_, AppState>, from: i64, to: i64) -> AppResult<()> {
    let mut conn = state.db.lock().map_err(|_| AppError::Lock)?;
    shelf::move_slot(&mut conn, from, to)
}

#[tauri::command]
pub fn remove_from_shelf(
    state: State<'_, AppState>,
    game_id: i64,
    result: RemoveResult,
) -> AppResult<()> {
    let mut conn = state.db.lock().map_err(|_| AppError::Lock)?;
    shelf::remove(&mut conn, game_id, &result)
}