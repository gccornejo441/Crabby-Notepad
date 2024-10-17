// Initializes the main window.
mod config;

fn main() {
    let config = config::Config::load().expect("Failed to load configuration");
    
    println!("The window title is {}", config.window.title);
}