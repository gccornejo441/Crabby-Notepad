mod app;

use app::CrabbyApp;
use config::Config;
use log::info;
use winit::event_loop::{ControlFlow, EventLoop};

mod config;

fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let config = Config::new();

    CrabbyApp::init(config);
}
