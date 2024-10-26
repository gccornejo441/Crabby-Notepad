use eframe::{self, egui, HardwareAcceleration, NativeOptions};
use std::path::PathBuf;

pub struct Application {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to Crabby!");
            if ui.button("Click").clicked() {
                print!("Hello world.")
            }
        });
    }
}

impl Application {
    pub fn init(app: Application) {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([app.width as f32, app.height as f32]),
            ..Default::default()
        };

        eframe::run_native(
            &app.title,
            native_options,
            Box::new(|_cc| {
                Ok(Box::new(Application {
                    title: app.title.to_string(),
                    width: app.width,
                    height: app.height,
                }))
            }),
        );
    }
}
