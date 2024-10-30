use crabby_notepad::path::update_check_file_path;
use eframe::{self, egui, glow::Context, HardwareAcceleration, NativeOptions};
use egui::{ScrollArea, SidePanel, Ui};
use log::error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;

#[derive(Debug, Clone)]
struct FileNode {
    name: String,
    path: PathBuf,
    is_dir: bool,
    children: Vec<FileNode>,
}

impl FileNode {
    fn from_path(path: &Path) -> Self {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let is_dir = path.is_dir();
        let children = if is_dir {
            match fs::read_dir(path) {
                Ok(entries) => entries
                    .filter_map(|entry| entry.ok())
                    .map(|entry| FileNode::from_path(&entry.path()))
                    .collect(),
                Err(_) => vec![],
            }
        } else {
            vec![]
        };
        FileNode {
            name,
            path: path.to_path_buf(),
            is_dir,
            children,
        }
    }
}

pub struct CrabbyApp {
    pub release_info: String,
    pub show_left_panel: bool,
    root_dir: FileNode,
}

impl CrabbyApp {
    pub fn new(root_path: PathBuf) -> Self {
        let root_dir = FileNode::from_path(&root_path);

        Self {
            release_info: "latest".to_string(),
            show_left_panel: true,
            root_dir,
        }
    }

    pub fn init(config: Config) {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([config.window.width as f32, config.window.height as f32]),
            ..Default::default()
        };

        let _ = eframe::run_native(
            &config.window.title,
            native_options,
            Box::new(|_cc| {
                Ok(Box::new(CrabbyApp {
                    release_info: config.app.release_info,
                    show_left_panel: config.app.show_left_panel,
                    root_dir: FileNode::from_path(Path::new(".")), // Load the current directory as the root
                }))
            }),
        );
    }

    /// Displays the file list in a scrollable area.
    pub fn files_list(&mut self, ui: &mut Ui) {
        ui.heading("Files");
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| display_file_tree(ui, &self.root_dir));
    }
}

impl eframe::App for CrabbyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_left_panel {
                SidePanel::left("left_panel")
                    .exact_width(300.0)
                    .show(ctx, |ui| {
                        ui.heading("Files");
                        self.files_list(ui);
                    });
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let path = update_check_file_path().unwrap();

        if let Err(e) = std::fs::write(&path, &self.release_info) {
            error!("Failed to write update check time to {path:?}: {e}");
        }
    }
}

/// Recursively displays the file tree using collapsible nodes.
fn display_file_tree(ui: &mut egui::Ui, node: &FileNode) {
    if node.is_dir {
        egui::CollapsingHeader::new(&node.name)
            .default_open(false)
            .show(ui, |ui| {
                for child in &node.children {
                    display_file_tree(ui, child);
                }
            });
    } else {
        ui.label(&node.name);
    }
}
