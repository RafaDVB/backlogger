use crate::db::games;
use crate::error::{AppError, AppResult};
use crate::models::{GameDetail, GameFilter, GameSummary, GameEdit, NewGame};
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

#[tauri::command]
pub fn create_game(state: State<'_, AppState>, new_game: NewGame) -> AppResult<GameDetail> {
    let mut conn = state.db.lock().map_err(|_| AppError::Lock)?;
    let id = games::create(&mut conn, &new_game)?;
    games::get(&conn, id)
}

#[tauri::command]
pub fn update_game(state: State<'_, AppState>, id: i64, edit: GameEdit) -> AppResult<GameDetail> {
    let mut conn = state.db.lock().map_err(|_| AppError::Lock)?;
    games::update(&mut conn, id, &edit)?;
    games::get(&conn, id)
}

#[tauri::command]
pub fn delete_game(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let mut conn = state.db.lock().map_err(|_| AppError::Lock)?;
    let _image_paths = games::delete(&mut conn, id)?;
    Ok(())
}