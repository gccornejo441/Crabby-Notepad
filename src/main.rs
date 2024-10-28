mod app;

use app::CrabbyApp;
use config::Config;
use log::info;
use winit::event_loop::{ControlFlow, EventLoop};

mod config;

fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let config = Config::new().load().unwrap_or_else(|err| {
        info!("Failed to load configuration: {}", err);
        Config {
            window: config::WindowConfig {
                title: "Crabby Notepad".to_string(),
                width: 800,
                height: 600,
            },
            editor: config::EditorConfig {
                font_size: 12,
                font_family: "Arial".to_string(),
            },
        }
    });

    CrabbyApp::init(CrabbyApp {
        title: config.window.title,
        width: config.window.width,
        height: config.window.height,
        release_info: "LATEST".to_string(),
        show_left_panel: true
    });
}
