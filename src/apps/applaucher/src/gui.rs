use eframe::{egui, App, Frame, NativeOptions};
use crate::desktop_parser::{DesktopApp, resolve_icon};
use crate::search::filter_apps;
use crate::launcher::launch_app;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::ffi::OsStr;

// GUI logic for displaying and selecting applications
pub fn run_gui(apps: Vec<DesktopApp>) {
    let options = NativeOptions::default();
    let _ = eframe::run_native(
        "App Launcher",
        options,
        Box::new(|cc| Box::new(AppLauncher::with_ctx(apps, cc))),
    );
}

struct AppLauncher {
    apps: Vec<DesktopApp>,
    query: String,
    filtered: Vec<usize>,
    selected: usize,
    selected_app: Option<usize>,
    icon_cache: HashMap<String, egui::TextureHandle>,
    missing_icons: HashSet<String>,
    pending_rx: Option<Receiver<(String, egui::ColorImage)>>,
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
            icon_cache: HashMap::new(),
            missing_icons: HashSet::new(),
            pending_rx: None,
        }
    }

    fn with_ctx(apps: Vec<DesktopApp>, cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = AppLauncher::new(apps);
        // Spawn a background thread to decode images and send ColorImage to the UI thread
        let (tx, rx) = channel();
        let apps_for_thread = app.apps.clone();
        std::thread::spawn(move || {
            for a in apps_for_thread.iter() {
                if let Some(icon_name) = a.icon.as_ref() {
                    // Prefer the resolved path returned by parser; otherwise try resolve_icon
                    let mut candidate_paths = Vec::new();
                    if icon_name.contains('/') {
                        candidate_paths.push(icon_name.clone());
                    } else if let Some(resolved) = resolve_icon(icon_name) {
                        candidate_paths.push(resolved);
                    }
                    for path in candidate_paths.iter() {
                        if Path::new(path).exists() {
                            // handle svg separately
                            if path.ends_with(".svg") {
                                if let Some(raster) = find_raster_sibling(path) {
                                    if let Ok(img) = image::open(raster) {
                                        let img = img.resize_exact(25, 25, image::imageops::FilterType::Lanczos3).to_rgba8();
                                        let size = [img.width() as usize, img.height() as usize];
                                        let pixels = img.into_vec();
                                        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                                        let _ = tx.send((icon_name.clone(), color_image));
                                        break;
                                    }
                                }
                            } else if let Ok(img) = image::open(path) {
                                let img = img.resize_exact(25, 25, image::imageops::FilterType::Lanczos3).to_rgba8();
                                let size = [img.width() as usize, img.height() as usize];
                                let pixels = img.into_vec();
                                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                                let _ = tx.send((icon_name.clone(), color_image));
                                break;
                            }
                        }
                    }
                }
            }
        });
        app.pending_rx = Some(rx);
        // Optionally preload a few first icons synchronously to reduce initial blankness
        let first_icon = app.apps.get(0).and_then(|a| a.icon.clone());
        if let Some(name) = first_icon {
            let _ = app.load_and_cache_icon(&cc.egui_ctx, &name);
        }
        app
    }

    fn load_and_cache_icon(&mut self, ctx: &egui::Context, name: &str) -> Option<egui::TextureHandle> {
        if self.icon_cache.contains_key(name) || self.missing_icons.contains(name) {
            return self.icon_cache.get(name).cloned();
        }
        // If name looks like a path, try directly
        let mut tried = Vec::new();
        if name.contains('/') {
            tried.push(name.to_string());
        } else if let Some(resolved) = resolve_icon(name) {
            tried.push(resolved);
        }
        for path in tried.iter() {
            if Path::new(path).exists() {
                // handle svg separately
                if path.ends_with(".svg") {
                    if let Some(raster) = find_raster_sibling(path) {
                        if let Ok(img) = image::open(raster) {
                            let img = img.resize_exact(25, 25, image::imageops::FilterType::Lanczos3).to_rgba8();
                            let size = [img.width() as usize, img.height() as usize];
                            let pixels = img.into_vec();
                            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                            let image_data = egui::ImageData::Color(color_image.into());
                            let handle = ctx.load_texture(name, image_data, egui::TextureOptions::default());
                            self.icon_cache.insert(name.to_string(), handle.clone());
                            return Some(handle);
                        }
                    }
                } else if let Ok(img) = image::open(path) {
                    let img = img.resize_exact(25, 25, image::imageops::FilterType::Lanczos3).to_rgba8();
                    let size = [img.width() as usize, img.height() as usize];
                    let pixels = img.into_vec();
                    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                    let image_data = egui::ImageData::Color(color_image.into());
                    let handle = ctx.load_texture(name, image_data, egui::TextureOptions::default());
                    self.icon_cache.insert(name.to_string(), handle.clone());
                    return Some(handle);
                }
            }
        }
        self.missing_icons.insert(name.to_string());
        None
    }

    fn get_icon(&mut self, ctx: &egui::Context, icon_name_or_path: &Option<String>) -> Option<egui::TextureHandle> {
        // Drain pending images sent by the background thread and create textures for them
        if let Some(rx) = &self.pending_rx {
            while let Ok((name, color_image)) = rx.try_recv() {
                let image_data = egui::ImageData::Color(color_image.into());
                let handle = ctx.load_texture(&name, image_data, egui::TextureOptions::default());
                self.icon_cache.insert(name.clone(), handle);
            }
        }

        if let Some(name) = icon_name_or_path {
            if let Some(handle) = self.icon_cache.get(name) {
                return Some(handle.clone());
            }
            if self.missing_icons.contains(name) {
                return None;
            }
            // try to load now (runtime fallback)
            if let Some(handle) = self.load_and_cache_icon(ctx, name) {
                return Some(handle);
            }
            self.missing_icons.insert(name.clone());
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

fn find_raster_sibling(path: &str) -> Option<String> {
    let p = Path::new(path);
    if let Some(parent) = p.parent() {
        if let Some(stem) = p.file_stem().and_then(OsStr::to_str) {
            for ext in &["png", "xpm"] {
                let candidate = parent.join(format!("{}.{}", stem, ext));
                if candidate.exists() {
                    if let Some(s) = candidate.to_str() { return Some(s.to_string()); }
                }
            }
        }
    }
    None
}