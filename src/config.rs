use crabby_notepad::path::update_check_file_path;
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
    pub app: AppConfig,
}

#[derive(Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub resizable: bool,
    pub maximized: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EditorConfig {
    pub font_size: u32,
    pub font_family: String,
    pub line_height: f32,
    pub theme: String,
    pub wrap_text: bool,
    pub show_line_numbers: bool,
    pub highlight_syntax: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub release_info: String,
    pub show_left_panel: bool,
}

impl Config {
    fn load_from_updates() -> Result<AppConfig, Box<dyn std::error::Error>> {
        let file_path = update_check_file_path()?;
        
        let contents = fs::read_to_string(file_path)?;
        let app_config: AppConfig = toml::from_str(&contents)?;

        Ok(app_config)
    }

    /// Creates a new configuration instance with default values.
    pub fn new() -> Config {
        let window_config = WindowConfig {
            width: 500,
            height: 500,
            title: "Crabby Notepad".to_string(),
            resizable: true,
            maximized: false,
        };

        let editor_config = EditorConfig {
            font_family: "Consolas".to_string(),
            font_size: 14,
            line_height: 1.5,
            theme: "Light".to_string(),
            wrap_text: true,
            show_line_numbers: true,
            highlight_syntax: true,
        };

        let app_config = Self::load_from_updates().unwrap_or_else(|_| AppConfig {
            release_info: "Initial Release".to_string(),
            show_left_panel: true,
        });

        Config {
            window: window_config,
            editor: editor_config,
            app: app_config,
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

        // File configs:
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("settings.toml")?;

        file.write_all(toml_string.as_bytes())?;

        Ok(())
    }
}
