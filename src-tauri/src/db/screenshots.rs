use crate::error::{AppError, AppResult};
use crate::models::Screenshot;
use rusqlite::{Connection, OptionalExtension, params};

pub fn add(conn: &Connection, game_id: i64, path: &str) -> AppResult<Screenshot> {
    let sort_order: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order) + 1, 0) FROM screenshots WHERE game_id = ?1",
        [game_id],
        |row| row.get(0),
    )?;
    conn.execute(
        "INSERT INTO screenshots (game_id, path, sort_order) VALUES (?1, ?2, ?3)",
        params![game_id, path, sort_order],
    )?;
    Ok(Screenshot {
        id: conn.last_insert_rowid(),
        path: path.to_string(),
        sort_order,
    })
}

/// Deletes the row and returns its file path so the caller can delete the file.
pub fn delete(conn: &Connection, id: i64) -> AppResult<String> {
    let path: String = conn
        .query_row("SELECT path FROM screenshots WHERE id = ?1", [id], |row| {
            row.get(0)
        })
        .optional()?
        .ok_or(AppError::NotFound)?;
    conn.execute("DELETE FROM screenshots WHERE id = ?1", [id])?;
    Ok(path)
}

/// Points a game at a new cover and returns the old cover path, if it had one.
pub fn set_cover(conn: &Connection, game_id: i64, path: &str) -> AppResult<Option<String>> {
    let old: Option<String> = conn
        .query_row(
            "SELECT cover_path FROM games WHERE id = ?1",
            [game_id],
            |row| row.get(0),
        )
        .optional()?
        .ok_or(AppError::NotFound)?;
    conn.execute(
        "UPDATE games SET cover_path = ?1 WHERE id = ?2",
        params![path, game_id],
    )?;
    Ok(old)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{self, games};
    use crate::models::GameFilter;

    fn seeded() -> Connection {
        let conn = db::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../../dev/seed.sql"))
            .unwrap();
        conn
    }

    #[test]
    fn new_screenshots_go_to_the_end() {
        let conn = seeded();
        assert_eq!(add(&conn, 1, "screenshots/a.png").unwrap().sort_order, 3); // had 0, 1, 2
        assert_eq!(add(&conn, 6, "screenshots/b.png").unwrap().sort_order, 0); // had none
    }

    #[test]
    fn adding_to_a_missing_game_fails() {
        let conn = seeded();
        assert!(add(&conn, 999, "screenshots/a.png").is_err());
    }

    #[test]
    fn deleting_the_featured_screenshot_falls_back_to_the_first() {
        let conn = seeded();
        let featured_id: i64 = conn
            .query_row(
                "SELECT featured_screenshot_id FROM games WHERE id = 3",
                [],
                |r| r.get(0),
            )
            .unwrap();

        assert_eq!(delete(&conn, featured_id).unwrap(), "placeholder:Hades 2");

        let all = games::list(&conn, &GameFilter::default()).unwrap();
        let hades = all.iter().find(|g| g.title == "Hades").unwrap();
        assert_eq!(hades.featured_path.as_deref(), Some("placeholder:Hades 1"));
    }

    #[test]
    fn set_cover_returns_the_old_cover() {
        let conn = seeded();
        let old = set_cover(&conn, 1, "covers/new.png").unwrap();
        assert_eq!(old.as_deref(), Some("placeholder:Hollow Knight (cover)"));
        assert_eq!(
            games::get(&conn, 1).unwrap().cover_path.as_deref(),
            Some("covers/new.png")
        );
        assert!(matches!(
            set_cover(&conn, 999, "covers/x.png"),
            Err(AppError::NotFound)
        ));
    }
}
