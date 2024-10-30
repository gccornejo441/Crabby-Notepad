use std::{fs, io, path::PathBuf};

use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub static CURRENT_DIR: Lazy<ProjectDirs> = Lazy::new(|| {
    ProjectDirs::from("", "CrabbyShack", "Crabby Notepad").expect("Failed to determine project directories")
});

/// Returns the file path for the `update_time.txt` file in the local data directory.
pub fn update_check_file_path() -> io::Result<PathBuf> {
    let path = CURRENT_DIR.data_local_dir().join("update_time.toml");

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    Ok(path)
}

pub fn config_file_path() -> io::Result<PathBuf> {
    let path = CURRENT_DIR.data_local_dir().join("settings.toml");

    if !path.exists() {

    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    Ok(path)
}