use crate::error::AppResult;
use crate::models::Settings;
use rusqlite::{params, Connection, OptionalExtension};

pub const RAWG_API_KEY: &str = "rawg_api_key";

pub fn get(conn: &Connection, key: &str) -> AppResult<Option<String>> {
    Ok(conn
        .query_row("SELECT value FROM settings WHERE key = ?1", [key], |row| row.get(0))
        .optional()?)
}

pub fn set(conn: &Connection, key: &str, value: &str) -> AppResult<()> {
    let value = value.trim();
    if value.is_empty() {
        conn.execute("DELETE FROM settings WHERE key = ?1", [key])?;
    } else {
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
    }
    Ok(())
}

pub fn all(conn: &Connection) -> AppResult<Settings> {
    Ok(Settings { rawg_api_key: get(conn, RAWG_API_KEY)? })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn a_setting_can_be_saved_changed_and_read_back() {
        let conn = db::open_in_memory().unwrap();
        assert_eq!(get(&conn, RAWG_API_KEY).unwrap(), None);

        set(&conn, RAWG_API_KEY, "  abc123 ").unwrap();
        assert_eq!(get(&conn, RAWG_API_KEY).unwrap().as_deref(), Some("abc123"));

        set(&conn, RAWG_API_KEY, "def456").unwrap();
        assert_eq!(all(&conn).unwrap().rawg_api_key.as_deref(), Some("def456"));
    }

    #[test]
    fn a_blank_value_removes_the_setting() {
        let conn = db::open_in_memory().unwrap();
        set(&conn, RAWG_API_KEY, "abc").unwrap();
        set(&conn, RAWG_API_KEY, "   ").unwrap();
        assert_eq!(get(&conn, RAWG_API_KEY).unwrap(), None);
    }
}