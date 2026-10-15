pub mod games;
pub mod lists;
pub mod shelf;

use crate::error::AppResult;
use rusqlite::Connection;
use std::path::Path;

// Append new migrations here; never edit one that has already shipped.
const MIGRATIONS: &[&str] = &[
    include_str!("../../migrations/0001-init.sql"),
    include_str!("../../migrations/0002-presets.sql")
];

pub fn open(path: &Path) -> AppResult<Connection> {
    init(Connection::open(path)?)
}

#[cfg(test)]
pub fn open_in_memory() -> AppResult<Connection> {
    init(Connection::open_in_memory()?)
}

fn init(mut conn: Connection) -> AppResult<Connection> {
    // SQLite ignores foreign keys unless you turn them on for every connection.
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