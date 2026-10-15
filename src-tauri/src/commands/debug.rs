use crate::error::{AppError, AppResult};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn seed_sample_data(state: State<'_, AppState>) -> AppResult<()> {
    let mut conn = state.db.lock().map_err(|_| AppError::Lock)?;
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM games", [], |row| row.get(0))?;
    if count == 0 {
        let tx = conn.transaction()?;
        tx.execute_batch(include_str!("../../dev/seed.sql"))?;
        tx.commit()?;
    }
    Ok(())
}
