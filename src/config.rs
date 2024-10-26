// config.rs:
// Handles configurations that might involve paths, settings, and other configurations that might need to be loaded or saved.
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    io::Write,
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub window: WindowConfig,
    pub editor: EditorConfig,
}

#[derive(Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditorConfig {
    pub font_size: u32,
    pub font_family: String,
}

impl Config {
    /// Creates a new configuration instance with default values.
    pub fn new() -> Config {

        let window_config = WindowConfig {
            width: 500,
            height: 500,
            title: "Crabby Notepad".to_string(),
        };

        let editor_config = EditorConfig {
            font_family: "Consolas".to_string(),
            font_size: 12,
        };

        Config {
            window: window_config,
            editor: editor_config,
        }
    }

    /// Loads saved configuration.
    pub fn load(&self) -> Result<Config, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string("Settings.toml")?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }

    /// Saves new configurations.
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string(&self)?;

        // file configs:
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("Test_Settings.toml")?;

        file.write_all(toml_string.as_bytes())?;

        Ok(())
    }
}
