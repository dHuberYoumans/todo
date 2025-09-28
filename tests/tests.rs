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
    fn new() -> Self {
        let tmp = TempDir::new().unwrap();
        let home = tmp.to_path_buf();
        let config_file = tmp.child("config.toml");
        let config_path = tmp.path().join(".todo/.test.db");
        config_file
            .write_str(
                format!(
                    r#"[database]
    TODO_DB="{}""#,
                    config_path.to_string_lossy()
                )
                .as_str(),
            )
            .unwrap();
        Self {
            tmp,
            home: home,
            config_path: config_path.to_path_buf(),
            config_file: config_file.to_path_buf(),
        }
    }

    fn cmd(&self) -> Command {
        let mut cmd = Command::cargo_bin("todo").unwrap();
        cmd.env("HOME", &self.home).env("CONFIG", &self.config_file);
        cmd
    }

    fn add(&self, task: &str, prio: &str, due: &str, tag: &str) -> Command {
        let input = format!("add -m {task} --prio={prio} --due={due} --tag={tag}");
        let args: Vec<&str> = input.split_whitespace().collect();
        let mut add_task = self.cmd();
        add_task.args(&args);
        add_task
    }

    fn get_stdout(&self, flag: Option<&str>) -> Vec<u8> {
        let out = match flag {
            Some("--done") => &self
                .cmd()
                .arg("list")
                .arg("--done")
                .assert()
                .get_output()
                .stdout
                .clone(),
            Some("--all") => &self
                .cmd()
                .arg("list")
                .arg("--all")
                .assert()
                .get_output()
                .stdout
                .clone(),
            Some("--sort tag") => &self
                .cmd()
                .arg("list")
                .arg("--sort")
                .arg("tag")
                .assert()
                .get_output()
                .stdout
                .clone(),
            Some("--sort prio") => &self
                .cmd()
                .arg("list")
                .arg("--sort")
                .arg("prio")
                .assert()
                .get_output()
                .stdout
                .clone(),
            Some("--sort due") => &self
                .cmd()
                .arg("list")
                .arg("--sort")
                .arg("due")
                .assert()
                .get_output()
                .stdout
                .clone(),
            _ => &self.cmd().arg("list").assert().get_output().stdout.clone(),
        };
        out.clone()
    }
}

#[test]
fn init() {
    let mock = MockData::new();
    mock.cmd()
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initializing.."))
        .stdout(predicate::str::contains("Setting up database.."))
        .stdout(predicate::str::contains("Creating new_list.."))
        .stdout(predicate::str::contains("✔ Created new list 'todo'"))
        .stdout(predicate::str::contains("✔ Added 'todo' to collection"))
        .stdout(predicate::str::contains("✔ All done"));
    mock.tmp
        .child(mock.config_path)
        .assert(predicates::path::exists());
}

#[test]
fn list() {
    let expected = r#"
╭───────┬──────┬────────┬──────┬─────┬─────╮
│ id    │ task │ status │ prio │ due │ tag │
╰───────┴──────┴────────┴──────┴─────┴─────╯"#;
    let mock = MockData::new();
    mock.cmd().arg("init").assert().success();
    let out = mock.get_stdout(None);
    let stdout = String::from_utf8_lossy(&out).trim().to_string();
    assert_eq!(stdout, expected.trim());
}

#[test]
fn add() {
    let expected = r#"
╭───────┬───────┬────────┬──────┬───────┬──────╮
│ id    │ task  │ status │ prio │ due   │ tag  │
├───────┼───────┼────────┼──────┼───────┼──────┤
│ 1     │ first │ ✘      │ P1   │ Today │ #tag │
╰───────┴───────┴────────┴──────┴───────┴──────╯
"#;
    let mock = MockData::new();
    mock.cmd().arg("init").assert().success();
    mock.add("first", "1", "today", "tag").assert().success();
    let out = mock.get_stdout(None);
    let stdout = String::from_utf8_lossy(&out).trim().to_string();
    assert_eq!(stdout, expected.trim());
}

#[test]
fn sort() {
    let by_tag = r#"
╭───────┬────────┬────────┬──────┬────────────┬──────╮
│ id    │ task   │ status │ prio │ due        │ tag  │
├───────┼────────┼────────┼──────┼────────────┼──────┤
│ 2     │ second │ ✘      │ P2   │ Today      │ #abc │
├───────┼────────┼────────┼──────┼────────────┼──────┤
│ 1     │ first  │ ✘      │ P1   │ Tomorrow   │ #tag │
├───────┼────────┼────────┼──────┼────────────┼──────┤
│ 3     │ third  │ ✘      │ P3   │ 2020-01-01 │ #xyz │
╰───────┴────────┴────────┴──────┴────────────┴──────╯
    "#;
    let by_prio = r#"
╭───────┬────────┬────────┬──────┬────────────┬──────╮
│ id    │ task   │ status │ prio │ due        │ tag  │
├───────┼────────┼────────┼──────┼────────────┼──────┤
│ 1     │ first  │ ✘      │ P1   │ Tomorrow   │ #tag │
├───────┼────────┼────────┼──────┼────────────┼──────┤
│ 2     │ second │ ✘      │ P2   │ Today      │ #abc │
├───────┼────────┼────────┼──────┼────────────┼──────┤
│ 3     │ third  │ ✘      │ P3   │ 2020-01-01 │ #xyz │
╰───────┴────────┴────────┴──────┴────────────┴──────╯
"#;
    let by_due = r#"
╭───────┬────────┬────────┬──────┬────────────┬──────╮
│ id    │ task   │ status │ prio │ due        │ tag  │
├───────┼────────┼────────┼──────┼────────────┼──────┤
│ 3     │ third  │ ✘      │ P3   │ 2020-01-01 │ #xyz │
├───────┼────────┼────────┼──────┼────────────┼──────┤
│ 2     │ second │ ✘      │ P2   │ Today      │ #abc │
├───────┼────────┼────────┼──────┼────────────┼──────┤
│ 1     │ first  │ ✘      │ P1   │ Tomorrow   │ #tag │
╰───────┴────────┴────────┴──────┴────────────┴──────╯
"#;
    let mock = MockData::new();
    mock.cmd().arg("init").assert().success();
    mock.add("first", "1", "tomorrow", "tag").assert().success();
    mock.add("second", "2", "today", "abc").assert().success();
    mock.add("third", "3", "01-01-2020", "xyz")
        .assert()
        .success();
    // sort by tag
    let mut out = mock.get_stdout(Some("--sort tag"));
    let mut stdout = String::from_utf8_lossy(&out).trim().to_string();
    assert_eq!(stdout, by_tag.trim());
    // sort by prio
    out = mock.get_stdout(Some("--sort prio"));
    stdout = String::from_utf8_lossy(&out).trim().to_string();
    assert_eq!(stdout, by_prio.trim());
    // sort by due
    out = mock.get_stdout(Some("--sort due"));
    stdout = String::from_utf8_lossy(&out).trim().to_string();
    assert_eq!(stdout, by_due.trim());
}

#[test]
fn close_open() {
    let open = r#"
╭───────┬───────┬────────┬──────┬───────┬──────╮
│ id    │ task  │ status │ prio │ due   │ tag  │
├───────┼───────┼────────┼──────┼───────┼──────┤
│ 1     │ first │ ✘      │ P1   │ Today │ #tag │
╰───────┴───────┴────────┴──────┴───────┴──────╯
"#;
    let closed = r#"
╭───────┬───────┬────────┬──────┬───────┬──────╮
│ id    │ task  │ status │ prio │ due   │ tag  │
├───────┼───────┼────────┼──────┼───────┼──────┤
│ 1     │ first │ ✔      │ P1   │ Today │ #tag │
╰───────┴───────┴────────┴──────┴───────┴──────╯
"#;

    let mock = MockData::new();
    mock.cmd().arg("init").assert().success();
    mock.add("first", "1", "today", "tag").assert().success();
    mock.cmd().arg("close").arg("1").assert().success();
    // close the task
    let out_closed = mock.get_stdout(Some("--done"));
    let mut stdout = String::from_utf8_lossy(&out_closed).trim().to_string();
    assert_eq!(stdout, closed.trim());
    // open the task
    mock.cmd().arg("open").arg("1").assert().success();
    let out_open = mock.get_stdout(None);
    stdout = String::from_utf8_lossy(&out_open).trim().to_string();
    assert_eq!(stdout, open.trim());
}

#[test]
fn delete_delete_all() {
    let expected = r#"
╭───────┬──────┬────────┬──────┬─────┬─────╮
│ id    │ task │ status │ prio │ due │ tag │
╰───────┴──────┴────────┴──────┴─────┴─────╯"#;
    let mock = MockData::new();
    mock.cmd().arg("init").assert().success();
    mock.add("first", "1", "today", "tag").assert().success();
    mock.cmd().arg("delete").arg("1").assert().success();
    let mut out = mock.get_stdout(None);
    let mut stdout = String::from_utf8_lossy(&out).trim().to_string();
    assert_eq!(stdout, expected.trim());
    mock.add("first", "1", "today", "tag1").assert().success();
    mock.add("second", "2", "tomorrow", "tag2")
        .assert()
        .success();
    mock.cmd().arg("delete-all").assert().success();
    out = mock.get_stdout(None);
    stdout = String::from_utf8_lossy(&out).trim().to_string();
    assert_eq!(stdout, expected.trim());
}

#[test]
fn new_list_load_and_who_is_this() {
    let expected = "This is foo. Ready for duty!";
    let mock = MockData::new();
    mock.cmd().arg("init").assert().success();
    mock.cmd().arg("new-list").arg("foo").assert().success();
    mock.cmd().arg("load").arg("foo").assert().success();
    let out = mock
        .cmd()
        .arg("who-is-this")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8_lossy(&out).trim().to_string();
    assert_eq!(stdout, expected.trim());
}
