// config.rs: 
// Handles configurations that might involve paths, settings, and other configurations that might need to be loaded or saved.
use std::{fs::{self, OpenOptions}, io::Read};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub window: WindowConfig,
    pub editor: EditorConfig
}

#[derive(Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

#[derive(Deserialize)]
pub struct EditorConfig {
    pub font_size: u32,
    pub font_family: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string("Settings.toml")?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}