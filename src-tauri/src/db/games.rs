use crate::error::{AppError, AppResult};
use crate::models::*;
use rusqlite::{Connection, OptionalExtension, params};
use std::collections::HashMap;

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
           AND (?2 IS NULL OR (g.shelf_slot IS NOT NULL) = ?2)
         ORDER BY g.title COLLATE NOCASE",
    )?;
    let rows = stmt.query_map(
        params![filter.status.map(Status::as_str), filter.on_shelf],
        |row| {
            Ok(GameSummary {
                id: row.get(0)?,
                title: row.get(1)?,
                release_year: row.get(2)?,
                status: row.get(3)?,
                outcome: row.get(4)?,
                shelf_slot: row.get(5)?,
                cover_path: row.get(6)?,
                featured_path: row.get(7)?,
                screenshot_paths: Vec::new(),
            })
        },
    )?;
    let mut games = rows.collect::<Result<Vec<_>, _>>()?;
    let mut shots: HashMap<i64, Vec<String>> = HashMap::new();
    let mut stmt =
        conn.prepare("SELECT game_id, path FROM screenshots ORDER BY game_id, sort_order, id")?;
    let pairs = stmt.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;
    for pair in pairs {
        let (game_id, path) = pair?;
        shots.entry(game_id).or_default().push(path);
    }
    for game in &mut games {
        game.screenshot_paths = shots.remove(&game.id).unwrap_or_default();
    }
    Ok(games)
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
        Ok(Platform {
            id: row.get(0)?,
            name: row.get(1)?,
        })
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
        Ok(Genre {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

fn screenshots_for(conn: &Connection, game_id: i64) -> AppResult<Vec<Screenshot>> {
    let mut stmt = conn.prepare(
        "SELECT id, path, sort_order FROM screenshots
         WHERE game_id = ?1 ORDER BY sort_order, id",
    )?;
    let rows = stmt.query_map([game_id], |row| {
        Ok(Screenshot {
            id: row.get(0)?,
            path: row.get(1)?,
            sort_order: row.get(2)?,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn mark_owned(conn: &Connection, id: i64) -> AppResult<()> {
    let changed = conn.execute("UPDATE games SET status = 'owned' WHERE id = ?1", [id])?;
    if changed == 0 {
        Err(AppError::NotFound)
    } else {
        Ok(())
    }
}

fn clean_title(title: &str) -> AppResult<String> {
    let title = title.trim();
    if title.is_empty() {
        return Err(AppError::Invalid("a game needs a title".into()));
    }
    Ok(title.to_string())
}

fn check_year(year: Option<i64>) -> AppResult<()> {
    match year {
        Some(y) if !(1950..=2100).contains(&y) => {
            Err(AppError::Invalid("that release year looks wrong".into()))
        }
        _ => Ok(()),
    }
}

fn set_links(
    conn: &Connection,
    game_id: i64,
    platform_ids: &[i64],
    genre_ids: &[i64],
) -> AppResult<()> {
    conn.execute("DELETE FROM game_platforms WHERE game_id = ?1", [game_id])?;
    conn.execute("DELETE FROM game_genres WHERE game_id = ?1", [game_id])?;
    for platform_id in platform_ids {
        conn.execute(
            "INSERT OR IGNORE INTO game_platforms (game_id, platform_id) VALUES (?1, ?2)",
            params![game_id, platform_id],
        )?;
    }
    for genre_id in genre_ids {
        conn.execute(
            "INSERT OR IGNORE INTO game_genres (game_id, genre_id) VALUES (?1, ?2)",
            params![game_id, genre_id],
        )?;
    }
    Ok(())
}

pub fn create(conn: &mut Connection, new: &NewGame) -> AppResult<i64> {
    let title = clean_title(&new.title)?;
    check_year(new.release_year)?;
    let tx = conn.transaction()?;
    tx.execute(
        "INSERT INTO games (title, release_year, status) VALUES (?1, ?2, ?3)",
        params![title, new.release_year, new.status.as_str()],
    )?;
    let id = tx.last_insert_rowid();
    set_links(&tx, id, &new.platform_ids, &new.genre_ids)?;
    tx.commit()?;
    Ok(id)
}

pub fn update(conn: &mut Connection, id: i64, edit: &GameEdit) -> AppResult<()> {
    let title = clean_title(&edit.title)?;
    check_year(edit.release_year)?;
    let tx = conn.transaction()?;
    let changed = tx.execute(
        "UPDATE games SET title = ?1, release_year = ?2 WHERE id = ?3",
        params![title, edit.release_year, id],
    )?;
    if changed == 0 {
        return Err(AppError::NotFound);
    }
    set_links(&tx, id, &edit.platform_ids, &edit.genre_ids)?;
    tx.commit()?;
    Ok(())
}

pub fn delete(conn: &mut Connection, id: i64) -> AppResult<Vec<String>> {
    let tx = conn.transaction()?;

    let cover: Option<String> = tx
        .query_row("SELECT cover_path FROM games WHERE id = ?1", [id], |row| {
            row.get(0)
        })
        .optional()?
        .ok_or(AppError::NotFound)?;
    let mut paths: Vec<String> = Vec::new();
    paths.extend(cover);

    {
        let mut stmt = tx.prepare("SELECT path FROM screenshots WHERE game_id = ?1")?;
        let rows = stmt.query_map([id], |row| row.get::<_, String>(0))?;
        for row in rows {
            paths.push(row?);
        }
    }

    tx.execute("DELETE FROM games WHERE id = ?1", [id])?;
    tx.commit()?;
    Ok(paths)
}

fn require_game(conn: &Connection, id: i64) -> AppResult<()> {
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM games WHERE id = ?1)",
        [id],
        |row| row.get(0),
    )?;
    if exists {
        Ok(())
    } else {
        Err(AppError::NotFound)
    }
}

pub fn set_featured_screenshot(
    conn: &Connection,
    game_id: i64,
    screenshot_id: Option<i64>,
) -> AppResult<()> {
    require_game(conn, game_id)?;
    if let Some(shot) = screenshot_id {
        let belongs: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM screenshots WHERE id = ?1 AND game_id = ?2)",
            params![shot, game_id],
            |row| row.get(0),
        )?;
        if !belongs {
            return Err(AppError::Invalid(
                "that screenshot doesn't belong to this game".into(),
            ));
        }
    }
    conn.execute(
        "UPDATE games SET featured_screenshot_id = ?1 WHERE id = ?2",
        params![screenshot_id, game_id],
    )?;
    Ok(())
}

pub fn set_rating(conn: &Connection, game_id: i64, rating_id: Option<i64>) -> AppResult<()> {
    require_game(conn, game_id)?;
    conn.execute(
        "UPDATE games SET rating_id = ?1 WHERE id = ?2",
        params![rating_id, game_id],
    )?;
    Ok(())
}

pub fn set_outcome(conn: &Connection, game_id: i64, outcome: Option<Outcome>) -> AppResult<()> {
    require_game(conn, game_id)?;
    if outcome.is_some() {
        let status: Status =
            conn.query_row("SELECT status FROM games WHERE id = ?1", [game_id], |row| {
                row.get(0)
            })?;
        if status != Status::Owned {
            return Err(AppError::NotOwned);
        }
    }
    conn.execute(
        "UPDATE games SET outcome = ?1 WHERE id = ?2",
        params![outcome.map(Outcome::as_str), game_id],
    )?;
    Ok(())
}

pub fn set_notes(conn: &Connection, game_id: i64, notes: &str) -> AppResult<()> {
    require_game(conn, game_id)?;
    conn.execute(
        "UPDATE games SET notes = ?1 WHERE id = ?2",
        params![notes, game_id],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    fn seeded() -> Connection {
        let conn = db::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../../dev/seed.sql"))
            .unwrap();
        conn
    }

    #[test]
    fn lists_games_by_status() {
        let conn = seeded();
        let owned = list(
            &conn,
            &GameFilter {
                status: Some(Status::Owned),
                ..Default::default()
            },
        )
        .unwrap();
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
        let result = conn.execute(
            "UPDATE games SET shelf_slot = 3 WHERE title = 'Baba Is You'",
            [],
        );
        assert!(result.is_err());
    }

    #[test]
    fn summaries_carry_screenshots_and_respect_featured_choice() {
        let conn = seeded();
        let games = list(&conn, &GameFilter::default()).unwrap();

        let hades = games.iter().find(|g| g.title == "Hades").unwrap();
        assert_eq!(hades.screenshot_paths.len(), 3);
        assert_eq!(hades.featured_path.as_deref(), Some("placeholder:Hades 2"));

        let stardew = games.iter().find(|g| g.title == "Stardew Valley").unwrap();
        assert!(stardew.screenshot_paths.is_empty());
        assert!(stardew.featured_path.is_none());
    }

    #[test]
    fn lists_only_shelved_games() {
        let conn = seeded();
        let filter = GameFilter {
            on_shelf: Some(true),
            ..Default::default()
        };
        assert_eq!(list(&conn, &filter).unwrap().len(), 2);
    }

    fn count(conn: &Connection, sql: &str) -> i64 {
        conn.query_row(sql, [], |row| row.get(0)).unwrap()
    }

    fn new_game(title: &str) -> NewGame {
        NewGame {
            title: title.into(),
            release_year: Some(2024),
            status: Status::Interested,
            platform_ids: vec![],
            genre_ids: vec![],
        }
    }

    #[test]
    fn creates_a_game_with_platforms_and_genres() {
        let mut conn = seeded();
        let pc: i64 = conn
            .query_row("SELECT id FROM platforms WHERE name = 'PC'", [], |r| {
                r.get(0)
            })
            .unwrap();
        let rpg: i64 = conn
            .query_row("SELECT id FROM genres WHERE name = 'RPG'", [], |r| r.get(0))
            .unwrap();
        let new = NewGame {
            platform_ids: vec![pc],
            genre_ids: vec![rpg],
            ..new_game("Elden Ring")
        };

        let id = create(&mut conn, &new).unwrap();

        let game = get(&conn, id).unwrap();
        assert_eq!(game.title, "Elden Ring");
        assert_eq!(game.status, Status::Interested);
        assert_eq!(game.platforms.len(), 1);
        assert_eq!(game.genres[0].name, "RPG");
    }

    #[test]
    fn create_trims_the_title() {
        let mut conn = seeded();
        let id = create(&mut conn, &new_game("  Tunic  ")).unwrap();
        assert_eq!(get(&conn, id).unwrap().title, "Tunic");
    }

    #[test]
    fn create_rejects_a_blank_title_and_a_silly_year() {
        let mut conn = seeded();
        assert!(matches!(
            create(&mut conn, &new_game("   ")),
            Err(AppError::Invalid(_))
        ));
        let old = NewGame {
            release_year: Some(1800),
            ..new_game("Ancient")
        };
        assert!(matches!(create(&mut conn, &old), Err(AppError::Invalid(_))));
    }

    #[test]
    fn a_failed_create_leaves_nothing_behind() {
        let mut conn = seeded();
        let before = count(&conn, "SELECT COUNT(*) FROM games");
        let bad = NewGame {
            platform_ids: vec![9999],
            ..new_game("Ghost")
        };
        assert!(create(&mut conn, &bad).is_err());
        assert_eq!(count(&conn, "SELECT COUNT(*) FROM games"), before);
    }

    #[test]
    fn update_replaces_the_editable_fields_only() {
        let mut conn = seeded();
        let pc: i64 = conn
            .query_row("SELECT id FROM platforms WHERE name = 'PC'", [], |r| {
                r.get(0)
            })
            .unwrap();
        let edit = GameEdit {
            title: "Celeste (Edited)".into(),
            release_year: Some(2018),
            platform_ids: vec![pc],
            genre_ids: vec![],
        };

        update(&mut conn, 2, &edit).unwrap();

        let game = get(&conn, 2).unwrap();
        assert_eq!(game.title, "Celeste (Edited)");
        assert_eq!(game.platforms.len(), 1);
        assert!(game.genres.is_empty());
        assert_eq!(game.status, Status::Owned);
        assert!(game.rating.is_some());
    }

    #[test]
    fn update_of_a_missing_game_is_not_found() {
        let mut conn = seeded();
        let edit = GameEdit {
            title: "Nope".into(),
            release_year: None,
            platform_ids: vec![],
            genre_ids: vec![],
        };
        assert!(matches!(
            update(&mut conn, 999, &edit),
            Err(AppError::NotFound)
        ));
    }

    #[test]
    fn delete_removes_the_game_its_links_and_returns_its_image_paths() {
        let mut conn = seeded();
        let paths = delete(&mut conn, 1).unwrap();

        assert_eq!(paths.len(), 4); // the cover and three screenshots
        assert!(matches!(get(&conn, 1), Err(AppError::NotFound)));
        assert_eq!(
            count(&conn, "SELECT COUNT(*) FROM screenshots WHERE game_id = 1"),
            0
        );
        assert_eq!(
            count(
                &conn,
                "SELECT COUNT(*) FROM game_platforms WHERE game_id = 1"
            ),
            0
        );
    }

    #[test]
    fn delete_of_a_missing_game_is_not_found() {
        let mut conn = seeded();
        assert!(matches!(delete(&mut conn, 999), Err(AppError::NotFound)));
    }

    fn id_of_screenshot(conn: &Connection, path: &str) -> i64 {
        conn.query_row("SELECT id FROM screenshots WHERE path = ?1", [path], |r| {
            r.get(0)
        })
        .unwrap()
    }

    fn featured_path_of(conn: &Connection, title: &str) -> Option<String> {
        list(conn, &GameFilter::default())
            .unwrap()
            .into_iter()
            .find(|g| g.title == title)
            .unwrap()
            .featured_path
    }

    #[test]
    fn featuring_a_screenshot_and_resetting_it() {
        let conn = seeded();
        let second = id_of_screenshot(&conn, "placeholder:Hollow Knight 2");

        set_featured_screenshot(&conn, 1, Some(second)).unwrap();
        assert_eq!(
            featured_path_of(&conn, "Hollow Knight").as_deref(),
            Some("placeholder:Hollow Knight 2")
        );

        set_featured_screenshot(&conn, 1, None).unwrap();
        assert_eq!(
            featured_path_of(&conn, "Hollow Knight").as_deref(),
            Some("placeholder:Hollow Knight 1")
        );
    }

    #[test]
    fn cannot_feature_another_games_screenshot() {
        let conn = seeded();
        let hades_shot = id_of_screenshot(&conn, "placeholder:Hades 1");
        assert!(matches!(
            set_featured_screenshot(&conn, 1, Some(hades_shot)),
            Err(AppError::Invalid(_))
        ));
        assert!(matches!(
            set_featured_screenshot(&conn, 999, None),
            Err(AppError::NotFound)
        ));
    }

    #[test]
    fn rating_can_be_changed_and_cleared() {
        let conn = seeded();
        set_rating(&conn, 2, Some(3)).unwrap();
        assert_eq!(get(&conn, 2).unwrap().rating.unwrap().label, "3");

        set_rating(&conn, 2, None).unwrap();
        assert!(get(&conn, 2).unwrap().rating.is_none());

        assert!(matches!(
            set_rating(&conn, 999, None),
            Err(AppError::NotFound)
        ));
    }

    #[test]
    fn outcome_is_only_for_owned_games() {
        let conn = seeded();
        set_outcome(&conn, 1, Some(Outcome::Dropped)).unwrap();
        assert_eq!(get(&conn, 1).unwrap().outcome, Some(Outcome::Dropped));

        set_outcome(&conn, 1, None).unwrap();
        assert_eq!(get(&conn, 1).unwrap().outcome, None);

        // Baba Is You (game 5) is only "interested".
        assert!(matches!(
            set_outcome(&conn, 5, Some(Outcome::Completed)),
            Err(AppError::NotOwned)
        ));
    }

    #[test]
    fn notes_are_saved_and_read_back() {
        let conn = seeded();
        set_notes(&conn, 1, "Password: ABCD\nSecond line").unwrap();
        assert_eq!(get(&conn, 1).unwrap().notes, "Password: ABCD\nSecond line");
        assert!(matches!(
            set_notes(&conn, 999, "x"),
            Err(AppError::NotFound)
        ));
    }
}
