use crate::error::{AppError, AppResult};
use crate::models::*;
use rusqlite::{Connection, OptionalExtension};

pub fn list(conn: &Connection, filter: &GameFilter) -> AppResult<Vec<GameSummary>> {
    let mut stmt = conn.prepare(
        "SELECT g.id, g.title, g.release_year, g.status, g.outcome, g.shelf_slot, g.cover_path,
                COALESCE(
                  (SELECT s.path FROM screenshots s WHERE s.id = g.featured_screenshot_id),
                  (SELECT s.path FROM screenshots s WHERE s.game_id = g.id
                   ORDER BY s.sort_order, s.id LIMIT 1)
                )
         FROM games g
         WHERE (?1 IS NULL OR g.status = ?1)
         ORDER BY g.title COLLATE NOCASE",
    )?;
    let rows = stmt.query_map([filter.status.map(Status::as_str)], |row| {
        Ok(GameSummary {
            id: row.get(0)?,
            title: row.get(1)?,
            release_year: row.get(2)?,
            status: row.get(3)?,
            outcome: row.get(4)?,
            shelf_slot: row.get(5)?,
            cover_path: row.get(6)?,
            featured_path: row.get(7)?,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn get(conn: &Connection, id: i64) -> AppResult<GameDetail> {
    let mut game = conn
        .query_row(
            "SELECT g.id, g.title, g.release_year, g.status, g.outcome, g.shelf_slot,
                    g.cover_path, g.featured_screenshot_id, g.notes, g.rawg_id,
                    r.id, r.label, r.sort_order, r.color
             FROM games g
             LEFT JOIN ratings r ON r.id = g.rating_id
             WHERE g.id = ?1",
            [id],
            |row| {
                let rating = match row.get::<_, Option<i64>>(10)? {
                    Some(rating_id) => Some(Rating {
                        id: rating_id,
                        label: row.get(11)?,
                        sort_order: row.get(12)?,
                        color: row.get(13)?,
                    }),
                    None => None,
                };
                Ok(GameDetail {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    release_year: row.get(2)?,
                    status: row.get(3)?,
                    outcome: row.get(4)?,
                    shelf_slot: row.get(5)?,
                    cover_path: row.get(6)?,
                    featured_screenshot_id: row.get(7)?,
                    notes: row.get(8)?,
                    rawg_id: row.get(9)?,
                    rating,
                    platforms: Vec::new(),
                    genres: Vec::new(),
                    screenshots: Vec::new(),
                })
            },
        )
        .optional()?
        .ok_or(AppError::NotFound)?;

    game.platforms = platforms_for(conn, id)?;
    game.genres = genres_for(conn, id)?;
    game.screenshots = screenshots_for(conn, id)?;
    Ok(game)
}

fn platforms_for(conn: &Connection, game_id: i64) -> AppResult<Vec<Platform>> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.name FROM platforms p
         JOIN game_platforms gp ON gp.platform_id = p.id
         WHERE gp.game_id = ?1 ORDER BY p.name",
    )?;
    let rows = stmt.query_map([game_id], |row| {
        Ok(Platform { id: row.get(0)?, name: row.get(1)? })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

fn genres_for(conn: &Connection, game_id: i64) -> AppResult<Vec<Genre>> {
    let mut stmt = conn.prepare(
        "SELECT g.id, g.name FROM genres g
         JOIN game_genres gg ON gg.genre_id = g.id
         WHERE gg.game_id = ?1 ORDER BY g.name",
    )?;
    let rows = stmt.query_map([game_id], |row| {
        Ok(Genre { id: row.get(0)?, name: row.get(1)? })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

fn screenshots_for(conn: &Connection, game_id: i64) -> AppResult<Vec<Screenshot>> {
    let mut stmt = conn.prepare(
        "SELECT id, path, sort_order FROM screenshots
         WHERE game_id = ?1 ORDER BY sort_order, id",
    )?;
    let rows = stmt.query_map([game_id], |row| {
        Ok(Screenshot { id: row.get(0)?, path: row.get(1)?, sort_order: row.get(2)? })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    fn seeded() -> Connection {
        let conn = db::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../../dev/seed.sql")).unwrap();
        conn
    }

    #[test]
    fn lists_games_by_status() {
        let conn = seeded();
        let owned = list(&conn, &GameFilter { status: Some(Status::Owned) }).unwrap();
        assert_eq!(owned.len(), 4);
        assert_eq!(list(&conn, &GameFilter::default()).unwrap().len(), 6);
    }

    #[test]
    fn get_returns_joined_data() {
        let conn = seeded();
        let celeste = get(&conn, 2).unwrap();
        assert_eq!(celeste.title, "Celeste");
        assert_eq!(celeste.rating.unwrap().label, "9");
        assert_eq!(celeste.platforms.len(), 2);
        assert_eq!(celeste.genres[0].name, "Platformer");
    }

    #[test]
    fn missing_game_is_not_found() {
        let conn = seeded();
        assert!(matches!(get(&conn, 999), Err(AppError::NotFound)));
    }

    #[test]
    fn interested_game_cannot_be_on_the_shelf() {
        let conn = seeded();
        let result = conn.execute("UPDATE games SET shelf_slot = 3 WHERE title = 'Baba Is You'", []);
        assert!(result.is_err());
    }
}