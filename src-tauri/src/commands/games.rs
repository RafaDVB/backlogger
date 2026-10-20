use crate::db::games;
use crate::error::{AppError, AppResult};
use crate::models::{GameDetail, GameEdit, GameFilter, GameSummary, NewGame, Outcome};
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
    let image_paths = {
        let mut conn = state.db.lock().map_err(|_| AppError::Lock)?;
        games::delete(&mut conn, id)?
    };
    for path in &image_paths {
        crate::images::remove(&state.images_dir, path);
    }
    Ok(())
}

#[tauri::command]
pub fn set_featured_screenshot(
    state: State<'_, AppState>,
    game_id: i64,
    screenshot_id: Option<i64>,
) -> AppResult<GameDetail> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    games::set_featured_screenshot(&conn, game_id, screenshot_id)?;
    games::get(&conn, game_id)
}

#[tauri::command]
pub fn set_rating(
    state: State<'_, AppState>,
    game_id: i64,
    rating_id: Option<i64>,
) -> AppResult<GameDetail> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    games::set_rating(&conn, game_id, rating_id)?;
    games::get(&conn, game_id)
}

#[tauri::command]
pub fn set_outcome(
    state: State<'_, AppState>,
    game_id: i64,
    outcome: Option<Outcome>,
) -> AppResult<GameDetail> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    games::set_outcome(&conn, game_id, outcome)?;
    games::get(&conn, game_id)
}

#[tauri::command]
pub fn set_notes(state: State<'_, AppState>, game_id: i64, notes: String) -> AppResult<()> {
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    games::set_notes(&conn, game_id, &notes)
}