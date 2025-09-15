use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database
}

#[derive(Debug, Deserialize)]
pub struct Database {
    #[serde(rename = "TODO_DB")]
    pub todo_db: String,
}
