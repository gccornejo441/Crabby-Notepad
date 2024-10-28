use std::path::PathBuf;

use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub static CURRENT_DIR: Lazy<ProjectDirs> = Lazy::new(|| {
    ProjectDirs::from("", "", "crabby_notepad").expect("Failed to determine project directories")
});

/// Returns the file path for the `update_time.txt` file in the local data directory.
pub fn update_check_file_path() -> PathBuf {
    CURRENT_DIR.data_local_dir().join("update_time.txt")
}
