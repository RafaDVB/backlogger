use crate::error::{AppError, AppResult};
use crate::models::{Appearance, Settings};
use rusqlite::{Connection, OptionalExtension, params};

pub const RAWG_API_KEY: &str = "rawg_api_key";
pub const UI_STATE: &str = "ui_state";
pub const THEME: &str = "theme";
pub const BACKGROUND: &str = "background";
pub const LIBRARY_SIZE: &str = "library_size";

const THEMES: [&str; 3] = ["ember", "aqua", "violet"];
const BACKGROUNDS: [&str; 5] = ["stars", "rain", "depths", "ocean", "off"];
const LIBRARY_SIZE_MIN: i64 = 140;
const LIBRARY_SIZE_MAX: i64 = 420;

pub fn get(conn: &Connection, key: &str) -> AppResult<Option<String>> {
    Ok(conn
        .query_row("SELECT value FROM settings WHERE key = ?1", [key], |row| {
            row.get(0)
        })
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
    Ok(Settings {
        rawg_api_key: get(conn, RAWG_API_KEY)?,
    })
}

pub fn appearance(conn: &Connection) -> AppResult<Appearance> {
    let defaults = Appearance::default();

    let theme = get(conn, THEME)?
        .filter(|value| THEMES.contains(&value.as_str()))
        .unwrap_or(defaults.theme);
    let background = get(conn, BACKGROUND)?
        .filter(|value| BACKGROUNDS.contains(&value.as_str()))
        .unwrap_or(defaults.background);
    let library_size = get(conn, LIBRARY_SIZE)?
        .and_then(|value| value.parse::<i64>().ok())
        .filter(|size| (LIBRARY_SIZE_MIN..=LIBRARY_SIZE_MAX).contains(size))
        .unwrap_or(defaults.library_size);

    Ok(Appearance {
        theme,
        background,
        library_size,
    })
}

pub fn set_appearance(conn: &Connection, appearance: &Appearance) -> AppResult<()> {
    if !THEMES.contains(&appearance.theme.as_str()) {
        return Err(AppError::Invalid(format!(
            "unknown palette: {}",
            appearance.theme
        )));
    }
    if !BACKGROUNDS.contains(&appearance.background.as_str()) {
        return Err(AppError::Invalid(format!(
            "unknown background: {}",
            appearance.background
        )));
    }
    if !(LIBRARY_SIZE_MIN..=LIBRARY_SIZE_MAX).contains(&appearance.library_size) {
        return Err(AppError::Invalid(format!(
            "the library size must be between {LIBRARY_SIZE_MIN} and {LIBRARY_SIZE_MAX}"
        )));
    }

    set(conn, THEME, &appearance.theme)?;
    set(conn, BACKGROUND, &appearance.background)?;
    set(conn, LIBRARY_SIZE, &appearance.library_size.to_string())
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

    #[test]
    fn the_saved_view_round_trips() {
        let conn = db::open_in_memory().unwrap();
        assert_eq!(get(&conn, UI_STATE).unwrap(), None);

        set(&conn, UI_STATE, r#"{"tab":"owned"}"#).unwrap();

        assert_eq!(
            get(&conn, UI_STATE).unwrap().as_deref(),
            Some(r#"{"tab":"owned"}"#)
        );
    }

    #[test]
    fn the_appearance_defaults_when_nothing_is_saved() {
        let conn = db::open_in_memory().unwrap();
        assert_eq!(appearance(&conn).unwrap(), Appearance::default());
    }

    #[test]
    fn the_appearance_round_trips() {
        let conn = db::open_in_memory().unwrap();
        let chosen = Appearance {
            theme: "aqua".into(),
            background: "rain".into(),
            library_size: 300,
        };

        set_appearance(&conn, &chosen).unwrap();

        assert_eq!(appearance(&conn).unwrap(), chosen);
    }

    #[test]
    fn an_unknown_value_is_rejected_and_nothing_is_saved() {
        let conn = db::open_in_memory().unwrap();

        let bad_theme = Appearance {
            theme: "neon".into(),
            ..Appearance::default()
        };
        assert!(set_appearance(&conn, &bad_theme).is_err());

        let bad_background = Appearance {
            background: "snow".into(),
            ..Appearance::default()
        };
        assert!(set_appearance(&conn, &bad_background).is_err());

        let bad_size = Appearance {
            library_size: 9000,
            ..Appearance::default()
        };
        assert!(set_appearance(&conn, &bad_size).is_err());

        assert_eq!(get(&conn, THEME).unwrap(), None);
        assert_eq!(get(&conn, LIBRARY_SIZE).unwrap(), None);
    }

    #[test]
    fn a_damaged_saved_value_falls_back_to_the_default() {
        let conn = db::open_in_memory().unwrap();
        set(&conn, THEME, "neon").unwrap();
        set(&conn, LIBRARY_SIZE, "huge").unwrap();

        assert_eq!(appearance(&conn).unwrap(), Appearance::default());
    }
}
