use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct DesktopAction {
    pub name: String,
    pub exec: String,
}

#[derive(Clone)]
pub struct DesktopApp {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub actions: Vec<DesktopAction>,
}

fn is_blacklisted_action(name: &str, exec: &str) -> bool {
    let s = format!("{} {}", name, exec).to_lowercase();
    // Blacklist common keywords for private/incognito modes and similar actions
    const BLACKLIST: [&str; 5] = ["incognito", "private", "guest", "private-window", "new-private"];
    BLACKLIST.iter().any(|kw| s.contains(kw))
}

fn parse_desktop_file(path: &Path) -> Option<DesktopApp> {
    let content = fs::read_to_string(path).ok()?;
    let mut name = None;
    let mut exec = None;
    let mut icon = None;
    let mut app_type = None;
    let mut nodisplay = false;
    let mut actions: Vec<DesktopAction> = Vec::new();
    let mut lines = content.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with("Type=") {
            app_type = Some(line[5..].to_string());
        } else if line.starts_with("NoDisplay=") {
            nodisplay = line[10..].trim() == "true";
        } else if line.starts_with("Name=") {
            name = Some(line[5..].to_string());
        } else if line.starts_with("Exec=") {
            exec = Some(line[5..].to_string());
        } else if line.starts_with("Icon=") {
            icon = Some(line[5..].to_string());
        } else if line.starts_with("Actions=") {
            // Actions=NewWindow;Incognito;
            let action_names: Vec<&str> = line[8..].split(';').filter(|s| !s.is_empty()).collect();
            for _action in action_names {
                // Look for [Desktop Action <action>] section
                let mut action_name = None;
                let mut action_exec = None;
                while let Some(next_line) = lines.peek() {
                    if next_line.trim().starts_with('[') { break; }
                    let next_line = lines.next().unwrap();
                    if next_line.starts_with("Name=") {
                        action_name = Some(next_line[5..].to_string());
                    } else if next_line.starts_with("Exec=") {
                        action_exec = Some(next_line[5..].to_string());
                    }
                }
                if let (Some(action_name), Some(action_exec)) = (action_name, action_exec) {
                    // Skip blacklisted actions like incognito/private
                    if is_blacklisted_action(&action_name, &action_exec) {
                        continue;
                    }
                    actions.push(DesktopAction { name: action_name, exec: action_exec });
                }
            }
        }
    }
    if let (Some(name), Some(exec)) = (name, exec) {
        if app_type.as_deref() == Some("Application") && !nodisplay {
            Some(DesktopApp { name, exec, icon, actions })
        } else {
            None
        }
    } else {
        None
    }
}

/// Try to resolve an icon name to a file path (png/svg) in standard icon directories
pub fn resolve_icon(icon_name: &str) -> Option<String> {
    let icon_dirs = [
        "/usr/share/icons/hicolor/48x48/apps/",
        "/usr/share/icons/hicolor/256x256/apps/",
        "/usr/share/pixmaps/",
        &format!("{}/.local/share/icons/", std::env::var("HOME").unwrap_or_default()),
    ];
    let extensions = ["png", "svg", "xpm"];
    for dir in icon_dirs.iter() {
        for ext in extensions.iter() {
            let path = format!("{}{}.{}", dir, icon_name, ext);
            if Path::new(&path).exists() {
                return Some(path);
            }
        }
    }
    None
}

pub fn parse_desktop_files() -> Vec<DesktopApp> {
    let mut apps = Vec::new();
    let dirs = [
        "/usr/share/applications",
        "/usr/local/share/applications",
        &format!("{}/.local/share/applications", std::env::var("HOME").unwrap_or_default()),
    ];
    for dir in dirs.iter() {
        let path = Path::new(dir);
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "desktop").unwrap_or(false) {
                    if let Some(app) = parse_desktop_file(&path) {
                        apps.push(app);
                    }
                }
            }
        }
    }
    apps
}