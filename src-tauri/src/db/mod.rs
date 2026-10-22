pub mod games;
pub mod lists;
pub mod ownership;
pub mod screenshots;
pub mod shelf;
pub mod list_edit;

use crate::error::AppResult;
use rusqlite::Connection;
use std::path::Path;

// Append new migrations here; never edit one that has already shipped.
const MIGRATIONS: &[&str] = &[
    include_str!("../../migrations/0001-init.sql"),
    include_str!("../../migrations/0002-presets.sql"),
    include_str!("../../migrations/0003-platform_state.sql"),
    include_str!("../../migrations/0004-store_presets.sql"),
];

pub fn open(path: &Path) -> AppResult<Connection> {
    init(Connection::open(path)?)
}

#[cfg(test)]
pub fn open_in_memory() -> AppResult<Connection> {
    init(Connection::open_in_memory()?)
}

fn init(mut conn: Connection) -> AppResult<Connection> {
    conn.pragma_update(None, "foreign_keys", "ON")?;
    migrate(&mut conn)?;
    Ok(conn)
}

fn migrate(conn: &mut Connection) -> AppResult<()> {
    let applied: i64 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
    for (index, sql) in MIGRATIONS.iter().enumerate().skip(applied as usize) {
        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.pragma_update(None, "user_version", (index + 1) as i64)?;
        tx.commit()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_store_migration_keeps_pc_when_games_use_it() {
        let conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();
        for sql in &MIGRATIONS[..3] {
            conn.execute_batch(sql).unwrap();
        }
        conn.execute_batch(
            "INSERT INTO games (title, status) VALUES ('Old', 'interested');
             INSERT INTO game_platforms (game_id, platform_id)
               SELECT 1, id FROM platforms WHERE name = 'PC';",
        )
        .unwrap();

        conn.execute_batch(MIGRATIONS[3]).unwrap();

        let mut stmt = conn.prepare("SELECT name FROM platforms").unwrap();
        let names: Vec<String> = stmt
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert!(names.contains(&"PC".to_string())); // still used, so it stays
        assert!(!names.contains(&"macOS".to_string())); // unused, so it goes
        assert!(names.contains(&"Steam".to_string()));
    }
}
