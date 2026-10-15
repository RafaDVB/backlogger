use crate::db::games;
use crate::error::{AppError, AppResult};
use crate::models::{GameDetail, GameFilter, GameSummary};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn list_games(state: State<'_, AppState>, filter: GameFilter) -> AppResult<Vec<GameSummary>> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    games::list(&conn, &filter)
}

#[tauri::command]
pub fn get_game(state: State<'_, AppState>, id: i64) -> AppResult<GameDetail> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    games::get(&conn, id)
}

#[tauri::command]
pub fn mark_owned(state: State<'_, AppState>, id: i64) -> AppResult<GameDetail> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    games::mark_owned(&conn, id)?;
    games::get(&conn, id)
}