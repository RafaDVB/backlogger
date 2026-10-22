use crate::db::{games, screenshots};
use crate::error::{AppError, AppResult};
use crate::images::{self, Kind};
use crate::models::{GameDetail, Screenshot};
use crate::state::AppState;
use std::path::Path;
use tauri::State;

#[tauri::command]
pub fn get_images_dir(state: State<'_, AppState>) -> String {
    state.images_dir.to_string_lossy().into_owned()
}

#[tauri::command]
pub fn add_screenshot(
    state: State<'_, AppState>,
    game_id: i64,
    source_path: String,
) -> AppResult<Screenshot> {
    // Copy first, without holding the database lock: file copies can be slow.
    let relative = images::import(&state.images_dir, Kind::Screenshot, Path::new(&source_path))?;
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    // If the database refuses (say the game doesn't exist), don't leave the copy behind.
    screenshots::add(&conn, game_id, &relative).inspect_err(|_| {
        images::remove(&state.images_dir, &relative);
    })
}

#[tauri::command]
pub fn delete_screenshot(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let path = {
        let conn = state.db.lock().map_err(|_| AppError::Lock)?;
        screenshots::delete(&conn, id)?
    }; // the lock is released here, before touching the disk
    images::remove(&state.images_dir, &path);
    Ok(())
}

#[tauri::command]
pub fn set_cover(
    state: State<'_, AppState>,
    game_id: i64,
    source_path: String,
) -> AppResult<GameDetail> {
    let relative = images::import(&state.images_dir, Kind::Cover, Path::new(&source_path))?;
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    match screenshots::set_cover(&conn, game_id, &relative) {
        Ok(old_cover) => {
            if let Some(old) = old_cover {
                images::remove(&state.images_dir, &old);
            }
            games::get(&conn, game_id)
        }
        Err(e) => {
            images::remove(&state.images_dir, &relative);
            Err(e)
        }
    }
}

fn header(request: &tauri::ipc::Request<'_>, name: &str) -> AppResult<String> {
    request
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
        .ok_or_else(|| AppError::Invalid(format!("missing {name} header")))
}

#[tauri::command]
pub fn add_screenshot_from_bytes(
    state: State<'_, AppState>,
    request: tauri::ipc::Request<'_>,
) -> AppResult<Screenshot> {
    let tauri::ipc::InvokeBody::Raw(bytes) = request.body() else {
        return Err(AppError::Invalid("expected raw image data".into()));
    };
    let game_id: i64 = header(&request, "x-game-id")?
        .parse()
        .map_err(|_| AppError::Invalid("that game id isn't a number".into()))?;
    let extension = header(&request, "x-extension")?;

    let relative = images::import_bytes(&state.images_dir, Kind::Screenshot, &extension, bytes)?;
    let conn = state.db.lock().map_err(|_| AppError::Lock)?;
    screenshots::add(&conn, game_id, &relative).map_err(|e| {
        images::remove(&state.images_dir, &relative);
        e
    })
}
