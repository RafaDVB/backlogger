use serde::Serialize;

#[tauri::command]
pub fn ping(message: String) -> String {
    format!("Rust received: {message}")
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LegalInfo {
    license: &'static str,
    third_party: &'static str,
}

#[tauri::command]
pub fn get_legal_info() -> LegalInfo {
    LegalInfo {
        license: include_str!("../../../LICENSE"),
        third_party: include_str!("../../../THIRD_PARTY_LICENSES.md"),
    }
}
