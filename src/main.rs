use gui::window;

// Initializes the main window.
mod config;
mod gui;

fn main() {
    let config = config::Config::new();
    config.load().expect("Failed to load configuration");

    let window_title = config.window.title;
    let window_width = config.window.width;
    let window_height = config.window.height;

    let window = window::Window::new(&window_title, window_width, window_height).expect("Failed to create window");

    let mut message = windows::Win32::UI::WindowsAndMessaging::MSG::default();
    while unsafe { windows::Win32::UI::WindowsAndMessaging::GetMessageW(&mut message, None, 0, 0).into() } {
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::DispatchMessageW(&message);
        }
    }
}