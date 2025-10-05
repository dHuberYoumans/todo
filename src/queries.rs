pub const COLLECTION: &str = "collection";

pub fn create_collection() -> String {
    format!(
        r#"
            CREATE TABLE IF NOT EXISTS {table} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            );"#,
        table = COLLECTION,
    )
}

pub fn fetch_collection() -> String {
    format!(
        r#"
            SELECT (name) FROM {table};
        "#,
        table = COLLECTION,
    )
}

pub fn fetch_due_date(date: i64) -> String {
    let current = std::env::var("CURRENT").unwrap_or("todo".into());
    format!(
        r#"
        SELECT * FROM {current}
        WHERE due={date};
    "#,
    )
}

pub fn add_to_collection(list: &str) -> String {
    format!(
        r#"
            INSERT INTO {table} (name) VALUES ('{list}');
        "#,
        table = COLLECTION,
        list = list,
    )
}

pub fn add_to_table(table: &str, list_id: i64) -> String {
    format!(
        r#"
        INSERT INTO {table} (task, list_id, status, prio, due, tag, created_at)
        VALUES (?1, {list_id}, ?2, ?3, ?4, ?5, ?6);
        "#
    )
}

pub fn create_list(table: &str) -> String {
    format!(
        r#"
            CREATE TABLE IF NOT EXISTS {table} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                list_id INTEGER NOT NULL REFERENCES {collection}(id),
                task TEXT NOT NULL,
                status INTEGER DEFAULT 0,
                prio INTEGER,
                due INTEGER,
                tag TEXT,
                created_at INTEGER
            );"#,
        collection = COLLECTION
    )
}

pub fn fetch_all_ids(list: &str) -> String {
    format!(
        r#"
            SELECT id FROM {table};
        "#,
        table = list,
    )
}

pub fn delete_list(list: &str) -> String {
    format!(
        r#"
            BEGIN;
            DELETE FROM {collection} WHERE name='{list}';
            DROP TABLE IF EXISTS {list};
            COMMIT;
        "#,
        collection = COLLECTION
    )
}

pub fn delete_by_id(list: &str) -> String {
    format!(
        r#"
            DELETE FROM {list} WHERE id=?1;
        "#
    )
}

pub fn fetch_task_by_id(list: &str) -> String {
    format!(
        r#"
            SELECT task FROM {list} WHERE id=?1;
        "#
    )
}

pub fn fetch_list_id(name: &str) -> String {
    format!(
        r#"
            SELECT id FROM {table} WHERE name='{name}';
        "#,
        table = COLLECTION,
    )
}

pub fn fetch_tags(list: &str) -> String {
    format!(
        r#"
            SELECT DISTINCT tag FROM {list};
        "#,
    )
}

pub fn unpdate_task_by_id(list: &str) -> String {
    format!(
        r#"
            UPDATE {list} SET task=?2 WHERE id=?1;
        "#
    )
}

pub fn update_status(list: &str) -> String {
    format!(
        r#"
        UPDATE {list} SET status=?1 WHERE id=?2;
        "#
    )
}

pub fn delete_task(list: &str, id: i64) -> String {
    format!(
        r#"
           DELETE FROM {list} WHERE id={id};
        "#
    )
}
