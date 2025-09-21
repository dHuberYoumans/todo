use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::path::PathBuf;

struct MockData {
    tmp: TempDir,
    home: PathBuf,
    config_path: PathBuf,
    config_file: PathBuf,
}

impl MockData {
    fn new() -> Self{
        let tmp = TempDir::new().unwrap();
        let home = tmp.to_path_buf();
        let config_file = tmp.child("config.toml");
        let config_path = tmp.path().join(".todo/.test.db");
        config_file.write_str(format!(r#"[database]
    TODO_DB="{}""#,
            config_path.to_string_lossy()).as_str()).unwrap();
        Self {
            tmp,
            home: home,
            config_path: config_path.to_path_buf(),
            config_file: config_file.to_path_buf(),
        }
    }
}

#[test]
fn init_flow() {
    let mock_data = MockData::new();
    let home = mock_data.home;
    let config_path = mock_data.config_path;
    let config_file = mock_data.config_file;
    Command::cargo_bin("todo").unwrap()
        .env("HOME", home)
        .env("CONFIG", config_file)
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initializing.."))
        .stdout(predicate::str::contains("Setting up database.."))
        .stdout(predicate::str::contains("Creating new_list.."))
        .stdout(predicate::str::contains("Created new todo list 'todo'"))
        .stdout(predicate::str::contains("Added todo to collection"))
        .stdout(predicate::str::contains("Database located at"))
        .stdout(predicate::str::contains("All done"));
    mock_data.tmp.child(config_path).assert(predicates::path::exists());
}

