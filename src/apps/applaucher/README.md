# App Launcher (applaucher)

A minimal application launcher in Rust, inspired by wofi. Lists installed applications by parsing .desktop files, provides a simple GUI for searching and launching apps.

## Usage

1. Build the launcher:
   ```sh
   cargo build --release
   ```
2. Run the launcher:
   ```sh
   cargo run --release
   ```

## Features
- Lists installed applications from standard directories
- Search/filter apps by name
- Launch selected app

## Dependencies
- egui, eframe, dirs

## Future Improvements
- Fuzzy search
- Icon display
- Configurable appearance
