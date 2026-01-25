use anyhow::{Context, Result};
use std::path::PathBuf;
use std::{env, fs, io::Read, process};

use crate::application::editor::Editor;

const TMP_FILE: &str = "./EDIT_TASK";

pub struct SysEditor;

impl Editor for SysEditor {
    fn edit(&self, old_text: Option<String>) -> Result<String> {
        let editor = env::var("EDITOR").unwrap_or(String::from("vi"));
        let path = PathBuf::from(TMP_FILE);
        fs::File::create(&path).unwrap_or_else(|_| panic!("✘ Could not open file {}", TMP_FILE));
        if let Some(text) = old_text {
            fs::write(&path, text)
                .unwrap_or_else(|_| panic!("✘ Could not write to file {}", TMP_FILE));
        };
        process::Command::new(editor)
            .arg(&path)
            .status()
            .expect("✘ Couldn't open your editor");
        let mut task = String::new();
        fs::File::open(&path)
            .unwrap_or_else(|_| panic!("✘ Could not open file {}", TMP_FILE))
            .read_to_string(&mut task)
            .expect("✘ Couldn't parse task");
        cleanup_tmp_files().context("✘ An error occured during cleanup")?;
        Ok(task)
    }
}

fn cleanup_tmp_files() -> Result<()> {
    let pattern = format!("{TMP_FILE}*");
    for file in glob::glob(&pattern)? {
        match file {
            Ok(path) => std::fs::remove_file(path)?,
            Err(e) => eprintln!("✘ Could not find file: {e}"),
        }
    }
    Ok(())
}
