use rusqlite::{named_params, Connection, Result};

pub struct Collection;

impl Collection {
    pub const TABLE: &'static str = "collection";

    pub fn create_table(conn: &Connection) -> Result<()> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );",
            Self::TABLE
        );
        conn.execute_batch(&sql)?;
        Ok(())
    }

    pub fn insert(conn: &Connection, name: &str) -> Result<()> {
        //log::debug!("executing query `{}`", &queries::add_to_collection(&list));
        let sql = format!("INSERT INTO {} (name) VALUES (:name);", Self::TABLE);
        conn.execute(&sql, named_params! { ":name": name, })?;
        Ok(())
    }

    pub fn fetch_all(conn: &Connection) -> Result<Vec<String>> {
        //log::debug!("executing query {}", &queries::fetch_collection());
        let sql = format!("SELECT name FROM {}", Self::TABLE);
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn fetch_id(conn: &Connection, name: &str) -> Result<i64> {
        let sql = format!("SELECT id FROM {} WHERE name = (:name);", Self::TABLE);
        conn.query_row(&sql, named_params! { ":name": name }, |row| row.get(0))
    }

    pub fn delete(conn: &mut Connection, list_name: &str) -> Result<()> {
        //log::debug!("executing query `{}`", &queries::delete_list(&list));
        let safe_name = sanitize_identifier(list_name)?;
        let sql = format!("DELETE FROM {} WHERE name = (:name);", Self::TABLE);
        let tx = conn.transaction()?;
        tx.execute(&sql, named_params! { ":name": list_name })?;
        tx.execute(&format!("DROP TABLE IF EXISTS {safe_name};"), [])?;
        tx.commit()?;
        Ok(())
    }
}

fn sanitize_identifier(s: &str) -> Result<String, rusqlite::Error> {
    if s.chars().all(|c| c.is_alphanumeric() || c == '_') {
        Ok(s.to_string())
    } else {
        Err(rusqlite::Error::InvalidQuery)
    }
}
