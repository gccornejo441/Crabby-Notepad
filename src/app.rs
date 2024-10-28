use crabby_notepad::path::update_check_file_path;
use eframe::{self, egui, glow::Context, HardwareAcceleration, NativeOptions};
use egui::{ScrollArea, SidePanel, Ui};
use log::error;
use std::path::PathBuf;

pub struct CrabbyApp {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub release_info: String,
    pub show_left_panel: bool,
}

impl CrabbyApp {
    pub fn init(app: CrabbyApp) {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([app.width as f32, app.height as f32]),
            ..Default::default()
        };

        eframe::run_native(
            &app.title,
            native_options,
            Box::new(|_cc| {
                Ok(Box::new(CrabbyApp {
                    title: app.title.to_string(),
                    width: app.width,
                    height: app.height,
                    release_info: app.release_info,
                    show_left_panel: app.show_left_panel,
                }))
            }),
        );
    }

    pub fn files_list(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.heading("Files");
        ScrollArea::vertical().auto_shrink([false; 2]);
    }
}

impl eframe::App for CrabbyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to Crabby!");
            if ui.button("Click").clicked() {
                print!("Hello world.")
            }

            if self.show_left_panel {
                SidePanel::left("left_panel")
                    .exact_width(300.00)
                    .show(ctx, |ui| ui.label("Files"));
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let path = update_check_file_path();
        if let Err(e) = std::fs::write(&path, &self.release_info) {
            error!("Failed to write update check time to {path:?}: {e}");
        }
    }
}
