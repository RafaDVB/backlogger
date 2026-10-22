use crate::db::games;
use crate::error::{AppError, AppResult};
use crate::models::*;
use rusqlite::{Connection, params};

pub fn sync_status(conn: &Connection, game_id: i64) -> AppResult<()> {
    let owned_links: i64 = conn.query_row(
        "SELECT COUNT(*) FROM game_platforms WHERE game_id = ?1 AND state = 'owned'",
        [game_id],
        |row| row.get(0),
    )?;
    let (status, shelf_slot, outcome): (Status, Option<i64>, Option<Outcome>) = conn.query_row(
        "SELECT status, shelf_slot, outcome FROM games WHERE id = ?1",
        [game_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;

    let should_be = if owned_links > 0 {
        Status::Owned
    } else {
        Status::Interested
    };
    if should_be == status {
        return Ok(());
    }
    if should_be == Status::Interested {
        if shelf_slot.is_some() {
            return Err(AppError::StillOnShelf);
        }
        if outcome.is_some() {
            return Err(AppError::HasOutcome);
        }
    }
    conn.execute(
        "UPDATE games SET status = ?1 WHERE id = ?2",
        params![should_be.as_str(), game_id],
    )?;
    Ok(())
}

pub fn set_platform_state(
    conn: &mut Connection,
    game_id: i64,
    platform_id: i64,
    state: Option<PlatformState>,
) -> AppResult<()> {
    games::require_game(conn, game_id)?;
    let tx = conn.transaction()?;
    match state {
        Some(state) => {
            tx.execute(
                "INSERT INTO game_platforms (game_id, platform_id, state) VALUES (?1, ?2, ?3)
                 ON CONFLICT(game_id, platform_id) DO UPDATE SET state = excluded.state",
                params![game_id, platform_id, state.as_str()],
            )?;
        }
        None => {
            tx.execute(
                "DELETE FROM game_platforms WHERE game_id = ?1 AND platform_id = ?2",
                params![game_id, platform_id],
            )?;
        }
    }
    sync_status(&tx, game_id)?;
    tx.commit()?;
    Ok(())
}

pub fn mark_owned(conn: &mut Connection, game_id: i64, platform_id: Option<i64>) -> AppResult<()> {
    games::require_game(conn, game_id)?;
    let tx = conn.transaction()?;
    match platform_id {
        Some(platform_id) => {
            tx.execute(
                "INSERT INTO game_platforms (game_id, platform_id, state) VALUES (?1, ?2, 'owned')
                 ON CONFLICT(game_id, platform_id) DO UPDATE SET state = 'owned'",
                params![game_id, platform_id],
            )?;
        }
        None => {
            let changed = tx.execute(
                "UPDATE game_platforms SET state = 'owned' WHERE game_id = ?1",
                [game_id],
            )?;
            if changed == 0 {
                return Err(AppError::Invalid(
                    "link at least one platform to this game before marking it as owned".into(),
                ));
            }
        }
    }
    sync_status(&tx, game_id)?;
    tx.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use rusqlite::OptionalExtension;

    fn seeded() -> Connection {
        let conn = db::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../../dev/seed.sql"))
            .unwrap();
        conn
    }

    fn platform_id(conn: &Connection, name: &str) -> i64 {
        conn.query_row("SELECT id FROM platforms WHERE name = ?1", [name], |r| {
            r.get(0)
        })
        .unwrap()
    }

    fn state_of(conn: &Connection, game_id: i64, platform: &str) -> Option<PlatformState> {
        let pid = platform_id(conn, platform);
        conn.query_row(
            "SELECT state FROM game_platforms WHERE game_id = ?1 AND platform_id = ?2",
            params![game_id, pid],
            |r| r.get(0),
        )
        .optional()
        .unwrap()
    }

    fn status_of(conn: &Connection, game_id: i64) -> Status {
        games::get(conn, game_id).unwrap().status
    }

    // Seed: 1 Hollow Knight (shelf slot 1, owned on PC and Switch), 2 Celeste (outcome set,
    // owned on PC and Switch), 4 Disco Elysium (interested, PC), 5 Baba Is You (interested,
    // PC and Switch).

    #[test]
    fn the_seed_data_follows_the_ownership_rule() {
        let conn = seeded();
        let broken: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM games g
                 WHERE (g.status = 'owned') !=
                       EXISTS(SELECT 1 FROM game_platforms gp
                              WHERE gp.game_id = g.id AND gp.state = 'owned')",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(broken, 0);
    }

    #[test]
    fn owned_on_one_platform_and_wanted_on_another() {
        let mut conn = seeded();
        let switch = platform_id(&conn, "Nintendo Switch");

        set_platform_state(&mut conn, 5, switch, Some(PlatformState::Owned)).unwrap();

        assert_eq!(status_of(&conn, 5), Status::Owned);
        assert_eq!(
            state_of(&conn, 5, "Nintendo Switch"),
            Some(PlatformState::Owned)
        );
        assert_eq!(state_of(&conn, 5, "PC"), Some(PlatformState::Wanted));
    }

    #[test]
    fn owned_on_several_platforms() {
        let mut conn = seeded();
        let pc = platform_id(&conn, "PC");
        let switch = platform_id(&conn, "Nintendo Switch");

        set_platform_state(&mut conn, 4, switch, Some(PlatformState::Owned)).unwrap();
        set_platform_state(&mut conn, 4, pc, Some(PlatformState::Owned)).unwrap();

        assert_eq!(state_of(&conn, 4, "PC"), Some(PlatformState::Owned));
        assert_eq!(
            state_of(&conn, 4, "Nintendo Switch"),
            Some(PlatformState::Owned)
        );
    }

    #[test]
    fn un_owning_the_last_owned_platform_returns_to_interested() {
        let mut conn = seeded();
        let pc = platform_id(&conn, "PC");
        let new = NewGame {
            title: "Tunic".into(),
            release_year: None,
            platforms: vec![PlatformLink {
                platform_id: pc,
                state: PlatformState::Owned,
            }],
            genre_ids: vec![],
        };
        let id = games::create(&mut conn, &new).unwrap();
        assert_eq!(status_of(&conn, id), Status::Owned);

        set_platform_state(&mut conn, id, pc, Some(PlatformState::Wanted)).unwrap();

        assert_eq!(status_of(&conn, id), Status::Interested);
    }

    #[test]
    fn the_last_owned_platform_of_a_shelved_game_cannot_be_un_owned() {
        let mut conn = seeded();
        let pc = platform_id(&conn, "PC");
        let switch = platform_id(&conn, "Nintendo Switch");

        // Hollow Knight is on the shelf. Losing one platform is fine...
        set_platform_state(&mut conn, 1, pc, Some(PlatformState::Wanted)).unwrap();
        // ...but not the last one.
        let result = set_platform_state(&mut conn, 1, switch, Some(PlatformState::Wanted));

        assert!(matches!(result, Err(AppError::StillOnShelf)));
        assert_eq!(
            state_of(&conn, 1, "Nintendo Switch"),
            Some(PlatformState::Owned)
        ); // rolled back
        assert_eq!(status_of(&conn, 1), Status::Owned);
    }

    #[test]
    fn unlinking_counts_as_un_owning() {
        let mut conn = seeded();
        let pc = platform_id(&conn, "PC");
        let switch = platform_id(&conn, "Nintendo Switch");

        set_platform_state(&mut conn, 1, switch, None).unwrap();
        let result = set_platform_state(&mut conn, 1, pc, None);

        assert!(matches!(result, Err(AppError::StillOnShelf)));
        assert_eq!(state_of(&conn, 1, "PC"), Some(PlatformState::Owned));
    }

    #[test]
    fn the_last_owned_platform_of_a_game_with_an_outcome_cannot_be_un_owned() {
        let mut conn = seeded();
        let pc = platform_id(&conn, "PC");
        let switch = platform_id(&conn, "Nintendo Switch");

        // Celeste has the outcome "completed".
        set_platform_state(&mut conn, 2, pc, Some(PlatformState::Wanted)).unwrap();
        let result = set_platform_state(&mut conn, 2, switch, Some(PlatformState::Wanted));

        assert!(matches!(result, Err(AppError::HasOutcome)));
        assert_eq!(status_of(&conn, 2), Status::Owned);
    }

    #[test]
    fn mark_owned_on_one_platform_leaves_the_others_wanted() {
        let mut conn = seeded();
        let switch = platform_id(&conn, "Nintendo Switch");

        mark_owned(&mut conn, 5, Some(switch)).unwrap();

        assert_eq!(status_of(&conn, 5), Status::Owned);
        assert_eq!(
            state_of(&conn, 5, "Nintendo Switch"),
            Some(PlatformState::Owned)
        );
        assert_eq!(state_of(&conn, 5, "PC"), Some(PlatformState::Wanted));
    }

    #[test]
    fn mark_owned_without_a_platform_owns_every_linked_platform() {
        let mut conn = seeded();
        mark_owned(&mut conn, 4, None).unwrap();
        assert_eq!(status_of(&conn, 4), Status::Owned);
        assert_eq!(state_of(&conn, 4, "PC"), Some(PlatformState::Owned));
    }

    #[test]
    fn mark_owned_needs_a_platform_from_somewhere() {
        let mut conn = seeded();
        let bare = NewGame {
            title: "Bare".into(),
            release_year: None,
            platforms: vec![],
            genre_ids: vec![],
        };
        let id = games::create(&mut conn, &bare).unwrap();

        assert!(matches!(
            mark_owned(&mut conn, id, None),
            Err(AppError::Invalid(_))
        ));
        assert!(matches!(
            mark_owned(&mut conn, 999, None),
            Err(AppError::NotFound)
        ));
    }

    #[test]
    fn a_new_game_gets_its_status_from_its_platforms() {
        let mut conn = seeded();
        let pc = platform_id(&conn, "PC");
        let owned = NewGame {
            title: "A".into(),
            release_year: None,
            platforms: vec![PlatformLink {
                platform_id: pc,
                state: PlatformState::Owned,
            }],
            genre_ids: vec![],
        };
        let wanted = NewGame {
            title: "B".into(),
            release_year: None,
            platforms: vec![PlatformLink {
                platform_id: pc,
                state: PlatformState::Wanted,
            }],
            genre_ids: vec![],
        };
        let none = NewGame {
            title: "C".into(),
            release_year: None,
            platforms: vec![],
            genre_ids: vec![],
        };

        let owned_id = games::create(&mut conn, &owned).unwrap();
        let wanted_id = games::create(&mut conn, &wanted).unwrap();
        let none_id = games::create(&mut conn, &none).unwrap();

        assert_eq!(status_of(&conn, owned_id), Status::Owned);
        assert_eq!(status_of(&conn, wanted_id), Status::Interested);
        assert_eq!(status_of(&conn, none_id), Status::Interested);
    }
}
