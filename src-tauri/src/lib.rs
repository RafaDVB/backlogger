mod commands;
mod db;
mod error;
mod models;
mod state;
mod images;

use state::AppState;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let conn = db::open(&dir.join("backlogger.db"))?;

            let images_dir = dir.join("images");
            std::fs::create_dir_all(&images_dir)?;

            app.manage(AppState { db: Mutex::new(conn), images_dir });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::system::ping,
            commands::games::list_games,
            commands::games::get_game,
            commands::games::mark_owned,
            commands::shelf::get_shelf,
            commands::shelf::place_on_shelf,
            commands::shelf::move_shelf_slot,
            commands::shelf::remove_from_shelf,
            commands::lists::list_ratings,
            commands::debug::seed_sample_data,
            commands::games::create_game,
            commands::games::update_game,
            commands::games::delete_game,
            commands::lists::list_genres,
            commands::lists::list_platforms,
            commands::lists::create_platform,
            commands::images::get_images_dir,
            commands::images::add_screenshot,
            commands::images::delete_screenshot,
            commands::images::set_cover
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
