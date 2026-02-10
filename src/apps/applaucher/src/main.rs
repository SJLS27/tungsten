// Entry point for the app launcher
mod desktop_parser;
mod gui;
mod launcher;
mod search;

fn main() {
    let apps = desktop_parser::parse_desktop_files();
    gui::run_gui(apps);
}