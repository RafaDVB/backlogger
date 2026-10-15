use crate::db::games;
use crate::error::{AppError, AppResult};
use crate::models::*;
use rusqlite::{params, Connection, OptionalExtension};

const SLOT_COUNT: i64 = 6;

fn validate_slot(slot: i64) -> AppResult<()> {
    if (1..=SLOT_COUNT).contains(&slot) {
        Ok(())
    } else {
        Err(AppError::InvalidSlot)
    }
}

fn game_state(conn: &Connection, game_id: i64) -> AppResult<(Status, Option<i64>)> {
    conn.query_row(
        "SELECT status, shelf_slot FROM games WHERE id = ?1",
        [game_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )
    .optional()?
    .ok_or(AppError::NotFound)
}

fn game_in_slot(conn: &Connection, slot: i64) -> AppResult<Option<i64>> {
    Ok(conn
        .query_row("SELECT id FROM games WHERE shelf_slot = ?1", [slot], |row| row.get(0))
        .optional()?)
}

pub fn get(conn: &Connection) -> AppResult<Vec<ShelfEntry>> {
    let filter = GameFilter { on_shelf: Some(true), ..Default::default() };
    let mut on_shelf = games::list(conn, &filter)?;
    Ok((1..=SLOT_COUNT)
        .map(|slot| {
            let index = on_shelf.iter().position(|g| g.shelf_slot == Some(slot));
            ShelfEntry { slot, game: index.map(|i| on_shelf.swap_remove(i)) }
        })
        .collect())
}

pub fn place(conn: &mut Connection, game_id: i64, slot: i64) -> AppResult<()> {
    validate_slot(slot)?;
    let tx = conn.transaction()?;
    let (status, current_slot) = game_state(&tx, game_id)?;
    if status != Status::Owned {
        return Err(AppError::NotOwned);
    }
    if current_slot.is_some() {
        return Err(AppError::AlreadyOnShelf);
    }
    if game_in_slot(&tx, slot)?.is_some() {
        return Err(AppError::SlotTaken);
    }
    tx.execute("UPDATE games SET shelf_slot = ?1 WHERE id = ?2", params![slot, game_id])?;
    tx.commit()?;
    Ok(())
}

pub fn move_slot(conn: &mut Connection, from: i64, to: i64) -> AppResult<()> {
    validate_slot(from)?;
    validate_slot(to)?;
    if from == to {
        return Ok(());
    }
    let tx = conn.transaction()?;
    let from_id = game_in_slot(&tx, from)?.ok_or(AppError::NotOnShelf)?;
    let to_id = game_in_slot(&tx, to)?;

    tx.execute("UPDATE games SET shelf_slot = NULL WHERE id = ?1", [from_id])?;
    if let Some(to_id) = to_id {
        tx.execute("UPDATE games SET shelf_slot = ?1 WHERE id = ?2", params![from, to_id])?;
    }
    tx.execute("UPDATE games SET shelf_slot = ?1 WHERE id = ?2", params![to, from_id])?;
    tx.commit()?;
    Ok(())
}

pub fn remove(conn: &mut Connection, game_id: i64, result: &RemoveResult) -> AppResult<()> {
    let tx = conn.transaction()?;
    let (_, slot) = game_state(&tx, game_id)?;
    if slot.is_none() {
        return Err(AppError::NotOnShelf);
    }
    match result {
        RemoveResult::Finished { rating_id } => {
            tx.execute(
                "UPDATE games SET shelf_slot = NULL, outcome = 'completed',
                        rating_id = COALESCE(?1, rating_id)
                 WHERE id = ?2",
                params![rating_id, game_id],
            )?;
        }
        RemoveResult::Dropped => {
            tx.execute(
                "UPDATE games SET shelf_slot = NULL, outcome = 'dropped' WHERE id = ?1",
                [game_id],
            )?;
        }
        RemoveResult::NotFinished => {
            tx.execute("UPDATE games SET shelf_slot = NULL WHERE id = ?1", [game_id])?;
        }
    }
    tx.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    // Seed: 1 Hollow Knight (slot 1), 2 Celeste (owned, rated 9), 3 Hades (slot 2),
    // 4 Disco Elysium and 5 Baba Is You (interested), 6 Stardew Valley (owned).
    fn seeded() -> Connection {
        let conn = db::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../../dev/seed.sql")).unwrap();
        conn
    }

    fn row(conn: &Connection, id: i64) -> (Option<i64>, Option<String>, Option<i64>) {
        conn.query_row(
            "SELECT shelf_slot, outcome, rating_id FROM games WHERE id = ?1",
            [id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .unwrap()
    }

    #[test]
    fn shelf_always_has_six_entries() {
        let shelf = get(&seeded()).unwrap();
        assert_eq!(shelf.len(), 6);
        assert_eq!(shelf.iter().filter(|e| e.game.is_some()).count(), 2);
        assert_eq!(shelf[0].game.as_ref().unwrap().title, "Hollow Knight");
    }

    #[test]
    fn places_an_owned_game_in_an_empty_slot() {
        let mut conn = seeded();
        place(&mut conn, 2, 3).unwrap();
        assert_eq!(row(&conn, 2).0, Some(3));
    }

    #[test]
    fn rejects_games_that_are_not_owned() {
        let mut conn = seeded();
        assert!(matches!(place(&mut conn, 5, 3), Err(AppError::NotOwned)));
    }

    #[test]
    fn marking_owned_unlocks_the_shelf() {
        let mut conn = seeded();
        games::mark_owned(&conn, 5).unwrap();
        place(&mut conn, 5, 3).unwrap();
    }

    #[test]
    fn rejects_a_taken_slot() {
        let mut conn = seeded();
        assert!(matches!(place(&mut conn, 2, 1), Err(AppError::SlotTaken)));
    }

    #[test]
    fn rejects_a_game_already_on_the_shelf() {
        let mut conn = seeded();
        assert!(matches!(place(&mut conn, 1, 4), Err(AppError::AlreadyOnShelf)));
    }

    #[test]
    fn rejects_slots_outside_one_to_six() {
        let mut conn = seeded();
        assert!(matches!(place(&mut conn, 2, 0), Err(AppError::InvalidSlot)));
        assert!(matches!(place(&mut conn, 2, 7), Err(AppError::InvalidSlot)));
    }

    #[test]
    fn moving_onto_an_occupied_slot_swaps() {
        let mut conn = seeded();
        move_slot(&mut conn, 1, 2).unwrap();
        assert_eq!(row(&conn, 1).0, Some(2));
        assert_eq!(row(&conn, 3).0, Some(1));
    }

    #[test]
    fn moving_to_an_empty_slot_just_moves() {
        let mut conn = seeded();
        move_slot(&mut conn, 1, 5).unwrap();
        assert_eq!(row(&conn, 1).0, Some(5));
    }

    #[test]
    fn cannot_move_from_an_empty_slot() {
        let mut conn = seeded();
        assert!(matches!(move_slot(&mut conn, 4, 5), Err(AppError::NotOnShelf)));
    }

    #[test]
    fn finishing_sets_completed_and_the_rating() {
        let mut conn = seeded();
        remove(&mut conn, 1, &RemoveResult::Finished { rating_id: Some(8) }).unwrap();
        assert_eq!(row(&conn, 1), (None, Some("completed".to_string()), Some(8)));
    }

    #[test]
    fn finishing_without_a_rating_keeps_the_old_one() {
        let mut conn = seeded();
        place(&mut conn, 2, 3).unwrap();
        remove(&mut conn, 2, &RemoveResult::Finished { rating_id: None }).unwrap();
        assert_eq!(row(&conn, 2), (None, Some("completed".to_string()), Some(9)));
    }

    #[test]
    fn dropping_sets_the_dropped_outcome() {
        let mut conn = seeded();
        remove(&mut conn, 1, &RemoveResult::Dropped).unwrap();
        assert_eq!(row(&conn, 1), (None, Some("dropped".to_string()), None));
    }

    #[test]
    fn not_finished_only_clears_the_slot() {
        let mut conn = seeded();
        remove(&mut conn, 3, &RemoveResult::NotFinished).unwrap();
        assert_eq!(row(&conn, 3), (None, None, None));
    }

    #[test]
    fn cannot_remove_a_game_that_is_not_on_the_shelf() {
        let mut conn = seeded();
        assert!(matches!(
            remove(&mut conn, 2, &RemoveResult::NotFinished),
            Err(AppError::NotOnShelf)
        ));
    }
}