use crate::error::{AppError, AppResult};
use crate::models::{Genre, Platform, Rating};
use rusqlite::{Connection, OptionalExtension};

pub fn ratings(conn: &Connection) -> AppResult<Vec<Rating>> {
    let mut stmt =
        conn.prepare("SELECT id, label, sort_order, color FROM ratings ORDER BY sort_order")?;
    let rows = stmt.query_map([], |row| {
        Ok(Rating {
            id: row.get(0)?,
            label: row.get(1)?,
            sort_order: row.get(2)?,
            color: row.get(3)?,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn genres(conn: &Connection) -> AppResult<Vec<Genre>> {
    let mut stmt = conn.prepare("SELECT id, name FROM genres ORDER BY name COLLATE NOCASE")?;
    let rows = stmt.query_map([], |row| {
        Ok(Genre {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn platforms(conn: &Connection) -> AppResult<Vec<Platform>> {
    let mut stmt = conn.prepare("SELECT id, name FROM platforms ORDER BY name COLLATE NOCASE")?;
    let rows = stmt.query_map([], |row| {
        Ok(Platform {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn create_platform(conn: &Connection, name: &str) -> AppResult<Platform> {
    let name = name.trim();
    if name.is_empty() {
        return Err(AppError::Invalid("a platform needs a name".into()));
    }
    let existing = conn
        .query_row(
            "SELECT id, name FROM platforms WHERE name = ?1 COLLATE NOCASE",
            [name],
            |row| {
                Ok(Platform {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            },
        )
        .optional()?;
    if let Some(platform) = existing {
        return Ok(platform);
    }
    conn.execute("INSERT INTO platforms (name) VALUES (?1)", [name])?;
    Ok(Platform {
        id: conn.last_insert_rowid(),
        name: name.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn ratings_are_seeded_one_to_ten_in_order() {
        let conn = db::open_in_memory().unwrap();
        let ratings = ratings(&conn).unwrap();
        assert_eq!(ratings.len(), 10);
        assert_eq!(ratings[0].label, "1");
        assert_eq!(ratings[9].label, "10");
    }

    #[test]
    fn presets_fill_genres_and_platforms() {
        let conn = db::open_in_memory().unwrap();
        assert!(genres(&conn).unwrap().iter().any(|g| g.name == "RPG"));
        assert!(
            platforms(&conn)
                .unwrap()
                .iter()
                .any(|p| p.name == "Nintendo Switch")
        );
    }

    #[test]
    fn creating_an_existing_platform_returns_it_regardless_of_casing() {
        let conn = db::open_in_memory().unwrap();
        let first = create_platform(&conn, "pc").unwrap();
        let again = create_platform(&conn, "  PC ").unwrap();
        assert_eq!(first.id, again.id);
    }

    #[test]
    fn a_new_platform_is_added_and_a_blank_one_is_rejected() {
        let conn = db::open_in_memory().unwrap();
        let steam_deck = create_platform(&conn, "Steam Deck").unwrap();
        assert!(
            platforms(&conn)
                .unwrap()
                .iter()
                .any(|p| p.id == steam_deck.id)
        );
        assert!(matches!(
            create_platform(&conn, "  "),
            Err(AppError::Invalid(_))
        ));
    }

    #[test]
    fn stores_are_presets_and_the_pc_entry_is_not() {
        let conn = db::open_in_memory().unwrap();
        let names: Vec<String> = platforms(&conn).unwrap().into_iter().map(|p| p.name).collect();
        assert!(names.contains(&"Steam".to_string()));
        assert!(names.contains(&"Epic Games Store".to_string()));
        assert!(!names.contains(&"PC".to_string()));
    }
}
