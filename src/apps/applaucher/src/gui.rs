use eframe::{egui, App, Frame, NativeOptions};
use crate::desktop_parser::{DesktopApp, resolve_icon};
use crate::search::filter_apps;
use crate::launcher::launch_app;

// GUI logic for displaying and selecting applications
pub fn run_gui(apps: Vec<DesktopApp>) {
    let options = NativeOptions::default();
    let _ = eframe::run_native(
        "App Launcher",
        options,
        Box::new(|_cc| Box::new(AppLauncher::new(apps))),
    );
}

struct AppLauncher {
    apps: Vec<DesktopApp>,
    query: String,
    filtered: Vec<usize>,
    selected: usize,
    selected_app: Option<usize>,
    icon_cache: std::collections::HashMap<String, Option<egui::ImageData>>,
}

impl AppLauncher {
    fn new(apps: Vec<DesktopApp>) -> Self {
        let filtered = (0..apps.len()).collect();
        Self {
            apps,
            query: String::new(),
            filtered,
            selected: 0,
            selected_app: None,
            icon_cache: std::collections::HashMap::new(),
        }
    }

    fn get_icon(&mut self, ctx: &egui::Context, icon_name: &Option<String>) -> Option<egui::TextureHandle> {
        if let Some(name) = icon_name {
            if let Some(Some(image)) = self.icon_cache.get(name) {
                return Some(ctx.load_texture(name, image.clone(), egui::TextureOptions::default()));
            }
            if let Some(path) = resolve_icon(name) {
                if let Ok(image) = image::open(&path) {
                    // Resize the image to 25x25 pixels for consistent icon size
                    let image = image.resize_exact(25, 25, image::imageops::FilterType::Lanczos3).to_rgba8();
                    let size = [image.width() as usize, image.height() as usize];
                    let pixels = image.into_vec();
                    let img = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                    self.icon_cache.insert(name.clone(), Some(egui::ImageData::Color(img.clone().into())));
                    return Some(ctx.load_texture(name, egui::ImageData::Color(img.into()), egui::TextureOptions::default()));
                }
            }
            self.icon_cache.insert(name.clone(), None);
        }
        None
    }
}

impl App for AppLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        use egui::Key;
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.selected_app {
                None => {
                    ui.text_edit_singleline(&mut self.query);
                    let filtered_indices: Vec<usize> = filter_apps(&self.apps, &self.query)
                        .iter()
                        .map(|app| self.apps.iter().position(|a| a.name == app.name).unwrap())
                        .collect();
                    self.filtered = filtered_indices.clone();

                    // Keyboard navigation
                    if ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
                        if self.selected + 1 < self.filtered.len() {
                            self.selected += 1;
                        }
                    }
                    if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
                        if self.selected > 0 {
                            self.selected -= 1;
                        }
                    }
                    if ctx.input(|i| i.key_pressed(Key::Enter)) {
                        if let Some(&idx) = self.filtered.get(self.selected) {
                            self.selected_app = Some(idx);
                        }
                    }

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (i, &idx) in filtered_indices.iter().enumerate() {
                            let app_icon = self.apps[idx].icon.clone();
                            let app_name = self.apps[idx].name.clone();
                            let selected = i == self.selected;
                            ui.horizontal(|ui| {
                                if let Some(icon) = self.get_icon(ctx, &app_icon) {
                                    ui.image(&icon);
                                }
                                let label = ui.selectable_label(selected, &app_name);
                                if label.clicked() {
                                    self.selected_app = Some(idx);
                                }
                            });
                        }
                    });
                }
                Some(idx) => {
                    let app_icon = self.apps[idx].icon.clone();
                    let app_name = self.apps[idx].name.clone();
                    let app_exec = self.apps[idx].exec.clone();
                    let app_actions = self.apps[idx].actions.clone();
                    ui.heading(&app_name);
                    if let Some(icon) = self.get_icon(ctx, &app_icon) {
                        ui.image(&icon);
                    }
                    ui.separator();
                    if ui.button("Launch").clicked() || ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                        launch_app(&app_exec);
                        self.selected_app = None;
                    }
                    if !app_actions.is_empty() {
                        ui.label("Other actions:");
                        for action in &app_actions {
                            if ui.button(&action.name).clicked() {
                                launch_app(&action.exec);
                                self.selected_app = None;
                            }
                        }
                    }
                    if ui.button("Back").clicked() || ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.selected_app = None;
                    }
                }
            }
        });
    }
}