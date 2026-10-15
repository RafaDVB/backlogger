#[tauri::command]
pub fn ping(message: String) -> String {
    format!("Rust received: {message}")
}
