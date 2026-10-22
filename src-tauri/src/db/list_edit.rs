use crate::db::ownership;
use crate::error::{AppError, AppResult};
use crate::models::*;
use rusqlite::{params, Connection, OptionalExtension};
use std::collections::HashSet;

const MAX_NAME_LENGTH: usize = 60;

const GENRE_USES: &str = "games g JOIN game_genres gg ON gg.game_id = g.id WHERE gg.genre_id = ?1";
const RATING_USES: &str = "games g WHERE g.rating_id = ?1";
const PLATFORM_USES: &str =
    "games g JOIN game_platforms gp ON gp.game_id = g.id WHERE gp.platform_id = ?1";


fn clean_name(raw: &str, what: &str) -> AppResult<String> {
    let name = raw.trim();
    if name.is_empty() {
        return Err(AppError::Invalid(format!("a {what} needs a name")));
    }
    if name.chars().count() > MAX_NAME_LENGTH {
        return Err(AppError::Invalid(format!(
            "a {what} name can have at most {MAX_NAME_LENGTH} characters"
        )));
    }
    Ok(name.to_string())
}

fn require_row(conn: &Connection, table: &str, id: i64) -> AppResult<()> {
    let exists: bool = conn.query_row(
        &format!("SELECT EXISTS(SELECT 1 FROM {table} WHERE id = ?1)"),
        [id],
        |row| row.get(0),
    )?;
    if exists {
        Ok(())
    } else {
        Err(AppError::NotFound)
    }
}

fn ensure_name_free(
    conn: &Connection,
    table: &str,
    column: &str,
    name: &str,
    except_id: i64,
    what: &str,
) -> AppResult<()> {
    let taken: bool = conn.query_row(
        &format!(
            "SELECT EXISTS(SELECT 1 FROM {table} WHERE {column} = ?1 COLLATE NOCASE AND id != ?2)"
        ),
        params![name, except_id],
        |row| row.get(0),
    )?;
    if taken {
        Err(AppError::Invalid(format!("a {what} with that name already exists")))
    } else {
        Ok(())
    }
}

fn rename_row(conn: &Connection, table: &str, what: &str, id: i64, name: &str) -> AppResult<String> {
    let name = clean_name(name, what)?;
    require_row(conn, table, id)?;
    ensure_name_free(conn, table, "name", &name, id, what)?;
    conn.execute(
        &format!("UPDATE {table} SET name = ?1 WHERE id = ?2"),
        params![name, id],
    )?;
    Ok(name)
}

fn preview(conn: &Connection, uses: &str, id: i64) -> AppResult<DeletePreview> {
    let total: i64 =
        conn.query_row(&format!("SELECT COUNT(*) FROM {uses}"), [id], |row| row.get(0))?;
    let mut stmt = conn.prepare(&format!(
        "SELECT g.id, g.title, g.release_year FROM {uses} ORDER BY g.title COLLATE NOCASE LIMIT 5"
    ))?;
    let rows = stmt.query_map([id], |row| {
        Ok(PreviewGame { id: row.get(0)?, title: row.get(1)?, release_year: row.get(2)? })
    })?;
    Ok(DeletePreview { total, first: rows.collect::<Result<Vec<_>, _>>()?, blocked: Vec::new() })
}


pub fn create_genre(conn: &Connection, name: &str) -> AppResult<Genre> {
    let name = clean_name(name, "genre")?;
    let existing = conn
        .query_row(
            "SELECT id, name FROM genres WHERE name = ?1 COLLATE NOCASE",
            [name.as_str()],
            |row| Ok(Genre { id: row.get(0)?, name: row.get(1)? }),
        )
        .optional()?;
    if let Some(genre) = existing {
        return Ok(genre);
    }
    conn.execute("INSERT INTO genres (name) VALUES (?1)", [name.as_str()])?;
    Ok(Genre { id: conn.last_insert_rowid(), name })
}

pub fn rename_genre(conn: &Connection, id: i64, name: &str) -> AppResult<Genre> {
    let name = rename_row(conn, "genres", "genre", id, name)?;
    Ok(Genre { id, name })
}

pub fn preview_delete_genre(conn: &Connection, id: i64) -> AppResult<DeletePreview> {
    require_row(conn, "genres", id)?;
    preview(conn, GENRE_USES, id)
}

pub fn delete_genre(conn: &Connection, id: i64) -> AppResult<()> {
    require_row(conn, "genres", id)?;
    conn.execute("DELETE FROM genres WHERE id = ?1", [id])?;
    Ok(())
}

fn clean_color(color: Option<String>) -> AppResult<Option<String>> {
    let Some(color) = color else { return Ok(None) };
    let color = color.trim();
    if color.is_empty() {
        return Ok(None);
    }
    let valid = color.len() == 7
        && color.starts_with('#')
        && color[1..].chars().all(|c| c.is_ascii_hexdigit());
    if valid {
        Ok(Some(color.to_ascii_lowercase()))
    } else {
        Err(AppError::Invalid("a color looks like #aabbcc".into()))
    }
}

fn rating_by_id(conn: &Connection, id: i64) -> AppResult<Rating> {
    Ok(conn.query_row(
        "SELECT id, label, sort_order, color FROM ratings WHERE id = ?1",
        [id],
        |row| {
            Ok(Rating {
                id: row.get(0)?,
                label: row.get(1)?,
                sort_order: row.get(2)?,
                color: row.get(3)?,
            })
        },
    )?)
}

pub fn create_rating(conn: &Connection, label: &str, color: Option<String>) -> AppResult<Rating> {
    let label = clean_name(label, "rating")?;
    let color = clean_color(color)?;
    ensure_name_free(conn, "ratings", "label", &label, 0, "rating")?;
    let next: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order) + 1, 1) FROM ratings",
        [],
        |row| row.get(0),
    )?;
    conn.execute(
        "INSERT INTO ratings (label, sort_order, color) VALUES (?1, ?2, ?3)",
        params![label, next, color],
    )?;
    rating_by_id(conn, conn.last_insert_rowid())
}

pub fn update_rating(
    conn: &Connection,
    id: i64,
    label: &str,
    color: Option<String>,
) -> AppResult<Rating> {
    let label = clean_name(label, "rating")?;
    let color = clean_color(color)?;
    require_row(conn, "ratings", id)?;
    ensure_name_free(conn, "ratings", "label", &label, id, "rating")?;
    conn.execute(
        "UPDATE ratings SET label = ?1, color = ?2 WHERE id = ?3",
        params![label, color, id],
    )?;
    rating_by_id(conn, id)
}

pub fn reorder_ratings(conn: &mut Connection, ids: &[i64]) -> AppResult<()> {
    let tx = conn.transaction()?;
    let existing: HashSet<i64> = {
        let mut stmt = tx.prepare("SELECT id FROM ratings")?;
        let rows = stmt.query_map([], |row| row.get::<_, i64>(0))?;
        rows.collect::<Result<_, _>>()?
    };
    let given: HashSet<i64> = ids.iter().copied().collect();
    if given.len() != ids.len() || given != existing {
        return Err(AppError::Invalid("the order must list every rating exactly once".into()));
    }
    for (position, id) in ids.iter().enumerate() {
        tx.execute(
            "UPDATE ratings SET sort_order = ?1 WHERE id = ?2",
            params![position as i64 + 1, id],
        )?;
    }
    tx.commit()?;
    Ok(())
}

pub fn preview_delete_rating(conn: &Connection, id: i64) -> AppResult<DeletePreview> {
    require_row(conn, "ratings", id)?;
    preview(conn, RATING_USES, id)
}

pub fn delete_rating(conn: &Connection, id: i64) -> AppResult<()> {
    require_row(conn, "ratings", id)?;
    conn.execute("DELETE FROM ratings WHERE id = ?1", [id])?;
    Ok(())
}

pub fn rename_platform(conn: &Connection, id: i64, name: &str) -> AppResult<Platform> {
    let name = rename_row(conn, "platforms", "platform", id, name)?;
    Ok(Platform { id, name })
}

pub fn preview_delete_platform(conn: &Connection, id: i64) -> AppResult<DeletePreview> {
    require_row(conn, "platforms", id)?;
    let mut result = preview(conn, PLATFORM_USES, id)?;
    let mut stmt = conn.prepare(
        "SELECT g.title FROM games g
         JOIN game_platforms gp
           ON gp.game_id = g.id AND gp.platform_id = ?1 AND gp.state = 'owned'
         WHERE (g.shelf_slot IS NOT NULL OR g.outcome IS NOT NULL)
           AND NOT EXISTS (SELECT 1 FROM game_platforms other
                           WHERE other.game_id = g.id AND other.state = 'owned'
                             AND other.platform_id != ?1)
         ORDER BY g.title COLLATE NOCASE",
    )?;
    result.blocked = stmt
        .query_map([id], |row| row.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(result)
}

pub fn delete_platform(conn: &mut Connection, id: i64) -> AppResult<()> {
    require_row(conn, "platforms", id)?;
    let tx = conn.transaction()?;
    let affected: Vec<i64> = {
        let mut stmt = tx.prepare("SELECT game_id FROM game_platforms WHERE platform_id = ?1")?;
        let rows = stmt.query_map([id], |row| row.get::<_, i64>(0))?;
        rows.collect::<Result<Vec<_>, _>>()?
    };
    tx.execute("DELETE FROM platforms WHERE id = ?1", [id])?; // links go with it
    for game_id in affected {
        ownership::sync_status(&tx, game_id)?; // refuses, and rolls back, if it would break the rule
    }
    tx.commit()?;
    Ok(())
}

pub fn merge_platform(conn: &mut Connection, from_id: i64, to_id: i64) -> AppResult<()> {
    if from_id == to_id {
        return Err(AppError::Invalid("choose a different platform to merge into".into()));
    }
    require_row(conn, "platforms", from_id)?;
    require_row(conn, "platforms", to_id)?;
    let tx = conn.transaction()?;
    tx.execute(
        "UPDATE game_platforms SET state = 'owned'
         WHERE platform_id = ?2 AND state = 'wanted'
           AND game_id IN (SELECT game_id FROM game_platforms
                           WHERE platform_id = ?1 AND state = 'owned')",
        params![from_id, to_id],
    )?;
    tx.execute(
        "UPDATE OR IGNORE game_platforms SET platform_id = ?2 WHERE platform_id = ?1",
        params![from_id, to_id],
    )?;
    tx.execute("DELETE FROM platforms WHERE id = ?1", [from_id])?;
    tx.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{self, games, lists};

    fn seeded() -> Connection {
        let conn = db::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../../dev/seed.sql")).unwrap();
        conn
    }

    fn id_of(conn: &Connection, table: &str, name: &str) -> i64 {
        conn.query_row(
            &format!("SELECT id FROM {table} WHERE name = ?1"),
            [name],
            |row| row.get(0),
        )
        .unwrap()
    }

    fn assert_ownership_rule_holds(conn: &Connection) {
        let broken: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM games g
                 WHERE (g.status = 'owned') !=
                       EXISTS(SELECT 1 FROM game_platforms gp
                              WHERE gp.game_id = g.id AND gp.state = 'owned')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(broken, 0);
    }

    #[test]
    fn creating_a_genre_trims_and_reuses_existing_names() {
        let conn = seeded();
        let created = create_genre(&conn, "  Metroidvania ").unwrap();
        assert_eq!(created.name, "Metroidvania");
        assert_eq!(create_genre(&conn, "metroidvania").unwrap().id, created.id);
        assert!(matches!(create_genre(&conn, "   "), Err(AppError::Invalid(_))));
    }

    #[test]
    fn renaming_checks_for_duplicates_and_missing_rows() {
        let conn = seeded();
        let rpg = id_of(&conn, "genres", "RPG");
        assert_eq!(rename_genre(&conn, rpg, "Role-playing").unwrap().name, "Role-playing");

        let action = id_of(&conn, "genres", "Action");
        assert!(matches!(rename_genre(&conn, action, "role-PLAYING"), Err(AppError::Invalid(_))));
        assert!(matches!(rename_genre(&conn, 9999, "X"), Err(AppError::NotFound)));
    }

    #[test]
    fn the_delete_preview_shows_the_first_five_and_the_total() {
        let mut conn = seeded();
        let genre = create_genre(&conn, "Roguelite").unwrap();
        for i in 0..7 {
            let new = NewGame {
                title: format!("Game {i}"),
                release_year: Some(2020),
                platforms: vec![],
                genre_ids: vec![genre.id],
            };
            games::create(&mut conn, &new).unwrap();
        }

        let preview = preview_delete_genre(&conn, genre.id).unwrap();

        assert_eq!(preview.total, 7);
        assert_eq!(preview.first.len(), 5);
        assert_eq!(preview.first[0].title, "Game 0");
        assert!(preview.blocked.is_empty());
    }

    #[test]
    fn deleting_a_genre_removes_it_from_its_games_but_keeps_the_games() {
        let conn = seeded();
        let platformer = id_of(&conn, "genres", "Platformer");
        assert_eq!(preview_delete_genre(&conn, platformer).unwrap().total, 2);

        delete_genre(&conn, platformer).unwrap();

        let hollow_knight = games::get(&conn, 1).unwrap();
        assert!(hollow_knight.genres.is_empty());
        assert_eq!(hollow_knight.title, "Hollow Knight");
    }

    #[test]
    fn a_new_rating_goes_last_and_must_be_unique_with_a_valid_color() {
        let conn = seeded();
        let rating = create_rating(&conn, "Love and hate", Some("#AA3366".into())).unwrap();
        assert_eq!(rating.sort_order, 11);
        assert_eq!(rating.color.as_deref(), Some("#aa3366"));

        assert!(matches!(create_rating(&conn, "love AND hate", None), Err(AppError::Invalid(_))));
        assert!(matches!(create_rating(&conn, "9", None), Err(AppError::Invalid(_)))); // a preset
        assert!(matches!(
            create_rating(&conn, "Bad color", Some("red".into())),
            Err(AppError::Invalid(_))
        ));
    }

    #[test]
    fn a_rating_can_be_edited() {
        let conn = seeded();
        let rating = create_rating(&conn, "Meh", Some("#112233".into())).unwrap();

        let updated = update_rating(&conn, rating.id, "Not for me", None).unwrap();

        assert_eq!(updated.label, "Not for me");
        assert_eq!(updated.color, None);
        assert_eq!(updated.sort_order, rating.sort_order);
    }

    #[test]
    fn ratings_can_be_reordered() {
        let mut conn = seeded();
        let mut ids: Vec<i64> = lists::ratings(&conn).unwrap().iter().map(|r| r.id).collect();
        ids.reverse();

        reorder_ratings(&mut conn, &ids).unwrap();

        let labels: Vec<String> =
            lists::ratings(&conn).unwrap().into_iter().map(|r| r.label).collect();
        assert_eq!(labels.first().map(String::as_str), Some("10"));
        assert_eq!(labels.last().map(String::as_str), Some("1"));

        ids.pop(); // forgets one rating
        assert!(matches!(reorder_ratings(&mut conn, &ids), Err(AppError::Invalid(_))));
    }

    #[test]
    fn deleting_a_rating_clears_it_from_its_games() {
        let conn = seeded();
        let nine = lists::ratings(&conn).unwrap().into_iter().find(|r| r.label == "9").unwrap();

        let preview = preview_delete_rating(&conn, nine.id).unwrap();
        assert_eq!(preview.total, 1);
        assert_eq!(preview.first[0].title, "Celeste");

        delete_rating(&conn, nine.id).unwrap();

        let celeste = games::get(&conn, 2).unwrap();
        assert!(celeste.rating.is_none());
        assert_eq!(celeste.title, "Celeste");
    }

    #[test]
    fn renaming_a_platform_checks_for_duplicates() {
        let conn = seeded();
        let switch = id_of(&conn, "platforms", "Nintendo Switch");
        assert_eq!(rename_platform(&conn, switch, "Switch").unwrap().name, "Switch");
        assert!(matches!(rename_platform(&conn, switch, "steam"), Err(AppError::Invalid(_))));
    }

    #[test]
    fn merging_moves_the_games_and_owned_wins() {
        let mut conn = seeded();
        let steam = id_of(&conn, "platforms", "Steam");
        let switch = id_of(&conn, "platforms", "Nintendo Switch");
        // Baba Is You: wanted on Steam, owned on Switch.
        ownership::set_platform_state(&mut conn, 5, switch, Some(PlatformState::Owned)).unwrap();

        merge_platform(&mut conn, switch, steam).unwrap();

        let baba = games::get(&conn, 5).unwrap();
        assert_eq!(baba.platforms.len(), 1);
        assert_eq!(baba.platforms[0].name, "Steam");
        assert_eq!(baba.platforms[0].state, PlatformState::Owned);
        assert_eq!(baba.status, Status::Owned);
        assert_eq!(games::get(&conn, 1).unwrap().platforms.len(), 1); // was on both platforms
        assert!(!lists::platforms(&conn).unwrap().iter().any(|p| p.id == switch));
        assert_ownership_rule_holds(&conn);
    }

    #[test]
    fn merging_needs_two_different_existing_platforms() {
        let mut conn = seeded();
        let steam = id_of(&conn, "platforms", "Steam");
        assert!(matches!(merge_platform(&mut conn, steam, steam), Err(AppError::Invalid(_))));
        assert!(matches!(merge_platform(&mut conn, steam, 9999), Err(AppError::NotFound)));
    }

    #[test]
    fn deleting_a_platform_that_would_un_own_a_shelved_game_is_blocked() {
        let mut conn = seeded();
        let steam = id_of(&conn, "platforms", "Steam");
        let switch = id_of(&conn, "platforms", "Nintendo Switch");
        // Hollow Knight is on the shelf, owned on Steam and Switch. Leave only Switch owned.
        ownership::set_platform_state(&mut conn, 1, steam, Some(PlatformState::Wanted)).unwrap();

        let preview = preview_delete_platform(&conn, switch).unwrap();
        assert_eq!(preview.blocked, vec!["Hollow Knight".to_string()]);

        assert!(matches!(delete_platform(&mut conn, switch), Err(AppError::StillOnShelf)));
        assert!(lists::platforms(&conn).unwrap().iter().any(|p| p.id == switch)); // still there
    }

    #[test]
    fn deleting_a_platform_returns_unprotected_games_to_interested() {
        let mut conn = seeded();
        let switch = id_of(&conn, "platforms", "Nintendo Switch");
        let new = NewGame {
            title: "Handheld".into(),
            release_year: None,
            platforms: vec![PlatformLink { platform_id: switch, state: PlatformState::Owned }],
            genre_ids: vec![],
        };
        let id = games::create(&mut conn, &new).unwrap();
        assert_eq!(games::get(&conn, id).unwrap().status, Status::Owned);

        delete_platform(&mut conn, switch).unwrap();

        let game = games::get(&conn, id).unwrap();
        assert_eq!(game.status, Status::Interested);
        assert!(game.platforms.is_empty());
        assert_ownership_rule_holds(&conn);
        assert!(matches!(delete_platform(&mut conn, switch), Err(AppError::NotFound)));
    }
}