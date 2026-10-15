use crate::error::AppResult;
use crate::models::Rating;
use rusqlite::Connection;

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
}