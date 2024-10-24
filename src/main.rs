use gui::window;

use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, MSG};

mod config;
mod gui;

fn main() {
    let config = config::Config::new();
    config.load().expect("Failed to load configuration");

    let window_title = config.window.title;
    let window_width = config.window.width;
    let window_height = config.window.height;

    let window = window::Window::new(&window_title, window_width, window_height).expect("Failed to create window");

    let mut message = MSG::default();
    while unsafe { GetMessageW(&mut message, None, 0, 0).into() } {
        unsafe {
            DispatchMessageW(&message);
        }
    }
}