use anyhow::{anyhow, Context, Result};
use flate2::read::GzDecoder;
use serde::Deserialize;
use std::env::temp_dir;
use std::os::unix::fs::PermissionsExt;
use std::{env, fs, path::Path};

use crate::domain::TodoList;

const REPO: &str = "dHuberYoumans/todo";

#[derive(Deserialize, Debug)]
struct ApiResponse {
    html_url: String,
}

impl TodoList {
    pub fn upgrade(&self, version: Option<String>) -> Result<()> {
        let version = version.unwrap_or(latest_version()?);
        println!("→ Upgrading to version {}...", &version);
        let asset = asset(&version)?;
        log::debug!("Resolved asset {}", &asset);
        let old_bin = env::current_exe()?;
        log::debug!("Current binary path: {}", &old_bin.display());
        let tmp_dir = temp_dir();
        let archive_path = tmp_dir.join(format!("{asset}.tar.gz"));
        let url = format!("https://github.com/{REPO}/releases/download/{version}/{asset}.tar.gz");
        println!("→ Downloading release asset...");
        log::debug!("Download URL: {}", &url);
        let download = reqwest::blocking::get(&url)
            .context("failed to download release asset")?
            .error_for_status()
            .context("GitHub returned error while downloading asset")?
            .bytes()
            .context("failed to read downloaded archive")?;
        fs::write(&archive_path, &download)?;
        println!("→ Extracting archive...");
        extract_tar_gz(&archive_path, tmp_dir.as_path())?;
        println!("→ Replacing executable...");
        let new_bin = tmp_dir.join("todo");
        fs::set_permissions(&new_bin, fs::Permissions::from_mode(0o755))?;
        fs::rename(&new_bin, &old_bin)?;
        println!("✔ Upgrade complete");
        Ok(())
    }
}

pub fn asset(version: &str) -> Result<String> {
    let target = target()?;
    Ok(format!("todo-{version}-{target}"))
}

pub fn target() -> Result<&'static str> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    match (os, arch) {
        ("macos", "aarch64") => Ok("aarch64-apple-darwin"),
        ("macos", "x86_64") => Ok("x86_64-apple-darwin"),
        ("linux", "x86_64") => Ok("x86_64-unknown-linux-gnu"),
        ("linux", "aarch64") => Ok("aarch64-unknown-linux-gnu"),
        _ => anyhow::bail!("unsupported platform"),
    }
}

fn latest_version() -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.github.com/repos/{REPO}/releases/latest");
    let resp = client.get(url).header("User-Agent", "todo-cli").send()?;
    let resp_json: ApiResponse = resp.json()?;
    let tag = resp_json
        .html_url
        .split('/')
        .next_back()
        .ok_or_else(|| anyhow!("Could not determine the latest version"))?;
    Ok(tag.to_string())
}

fn extract_tar_gz(archive: &Path, out_dir: &Path) -> Result<()> {
    let file = fs::File::open(archive)?;
    let gz = GzDecoder::new(file);
    let mut archive = tar::Archive::new(gz);
    archive
        .unpack(out_dir)
        .context("failed to unpack archive")?;
    Ok(())
}
