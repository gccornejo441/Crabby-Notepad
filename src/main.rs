mod app;

use app::Application;
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

    // Application::init(app);
    Application::init(Application {
        title: config.window.title,
        width: config.window.width,
        height: config.window.height,
    });
}
