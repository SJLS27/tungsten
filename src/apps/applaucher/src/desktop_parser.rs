use std::fs;
use std::path::{Path, PathBuf};

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
    // We'll parse line by line and also capture action sections
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
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
            // For each action name, search for a matching [Desktop Action <Name>] section
            for action_name_key in action_names {
                // Find the section header and parse until next section
                let section_header = format!("[Desktop Action {}]", action_name_key);
                let mut action_name = None;
                let mut action_exec = None;
                // Search for the section
                let mut j = 0;
                while j < lines.len() {
                    if lines[j].trim() == section_header {
                        // parse subsequent lines until next [ starts
                        j += 1;
                        while j < lines.len() && !lines[j].trim().starts_with('[') {
                            if lines[j].starts_with("Name=") {
                                action_name = Some(lines[j][5..].to_string());
                            } else if lines[j].starts_with("Exec=") {
                                action_exec = Some(lines[j][5..].to_string());
                            }
                            j += 1;
                        }
                        break;
                    }
                    j += 1;
                }
                if let (Some(action_name), Some(action_exec)) = (action_name, action_exec) {
                    if is_blacklisted_action(&action_name, &action_exec) {
                        continue;
                    }
                    actions.push(DesktopAction { name: action_name, exec: action_exec });
                }
            }
        }
        i += 1;
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

/// Try to resolve an icon name to a file path (png/svg/xpm) in standard icon directories
pub fn resolve_icon(icon_name: &str) -> Option<String> {
    use std::ffi::OsStr;

    let exts = ["png", "svg", "xpm"];

    // If icon_name looks like a path, check it directly (with or without extension)
    if icon_name.contains('/') {
        let p = Path::new(icon_name);
        if p.exists() {
            return Some(icon_name.to_string());
        }
        for ext in exts.iter() {
            let try_path = format!("{}.{}", icon_name, ext);
            if Path::new(&try_path).exists() {
                return Some(try_path);
            }
        }
    }

    // Candidate base directories to search (include Flatpak and Snap locations)
    let mut base_dirs: Vec<PathBuf> = vec![
        PathBuf::from("/usr/share/icons"),
        PathBuf::from("/usr/share/pixmaps"),
        PathBuf::from(format!("{}/.local/share/icons", std::env::var("HOME").unwrap_or_default())),
        PathBuf::from("/var/lib/flatpak/exports/share/icons"),
        PathBuf::from(format!("{}/.local/share/flatpak/exports/share/icons", std::env::var("HOME").unwrap_or_default())),
        PathBuf::from("/var/lib/snapd/desktop/icons"),
        PathBuf::from("/usr/share/icons/hicolor"),
    ];

    // also add common app-theme sized folders (hicolor/*/apps) under /var/lib/flatpak/exports/share/icons
    base_dirs.push(PathBuf::from("/var/lib/flatpak/exports/share/icons/hicolor"));
    base_dirs.push(PathBuf::from(format!("{}/.local/share/flatpak/exports/share/icons/hicolor", std::env::var("HOME").unwrap_or_default())));

    // helper: recursive search for file whose stem matches icon_name and ext is in exts
    fn search_recursive(dir: &Path, icon_name: &str, exts: &[&str]) -> Option<PathBuf> {
        if !dir.exists() { return None; }
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    if let Some(found) = search_recursive(&p, icon_name, exts) {
                        return Some(found);
                    }
                } else if let Some(stem) = p.file_stem().and_then(OsStr::to_str) {
                    if stem == icon_name {
                        if let Some(ext) = p.extension().and_then(OsStr::to_str) {
                            if exts.iter().any(|e| *e == ext) {
                                return Some(p);
                            }
                        }
                    }
                    // Also allow filenames that start with the icon name (some icon names have suffixes)
                    if stem.starts_with(icon_name) {
                        if let Some(ext) = p.extension().and_then(OsStr::to_str) {
                            if exts.iter().any(|e| *e == ext) {
                                return Some(p);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    // First, try classic locations with direct <icon_name>.<ext>
    for dir in [
        "/usr/share/icons/hicolor/48x48/apps/",
        "/usr/share/icons/hicolor/256x256/apps/",
        &format!("{}/.local/share/icons/", std::env::var("HOME").unwrap_or_default()),
        "/usr/share/pixmaps/",
        "/var/lib/snapd/desktop/icons/",
    ] {
        for ext in exts.iter() {
            let path = format!("{}{}.{}", dir, icon_name, ext);
            if Path::new(&path).exists() {
                return Some(path);
            }
        }
    }

    // Then recursively search the candidate base dirs
    for base in base_dirs.iter() {
        if let Some(found) = search_recursive(base, icon_name, &exts) {
            if let Some(s) = found.to_str() { return Some(s.to_string()); }
        }
    }

    // Fallback: try looking under flatpak application directories (icons next to .desktop)
    // We'll look for <icon_name>.* in the same folder as any flatpak desktop files
    let flatpak_app_dirs = [
        PathBuf::from("/var/lib/flatpak/exports/share/applications"),
        PathBuf::from(format!("{}/.local/share/flatpak/exports/share/applications", std::env::var("HOME").unwrap_or_default())),
    ];
    for apps_dir in flatpak_app_dirs.iter() {
        if apps_dir.exists() {
            if let Ok(entries) = fs::read_dir(apps_dir) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.is_file() {
                        if let Some(parent) = p.parent() {
                            if let Ok(entries2) = fs::read_dir(parent) {
                                for e2 in entries2.flatten() {
                                    let p2 = e2.path();
                                    if let Some(stem) = p2.file_stem().and_then(OsStr::to_str) {
                                        if stem == icon_name {
                                            if let Some(ext) = p2.extension().and_then(OsStr::to_str) {
                                                if exts.iter().any(|x| *x == ext) {
                                                    if let Some(s) = p2.to_str() { return Some(s.to_string()); }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn collect_desktop_files_from_dir(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_desktop_files_from_dir(&path, out);
            } else if path.extension().map(|e| e == "desktop").unwrap_or(false) {
                out.push(path);
            }
        }
    }
}

pub fn parse_desktop_files() -> Vec<DesktopApp> {
    let mut apps = Vec::new();
    // Standard directories plus flatpak and snap locations
    let mut dirs: Vec<PathBuf> = vec![
        PathBuf::from("/usr/share/applications"),
        PathBuf::from("/usr/local/share/applications"),
        PathBuf::from(format!("{}/.local/share/applications", std::env::var("HOME").unwrap_or_default())),
        PathBuf::from("/var/lib/flatpak/exports/share/applications"),
        PathBuf::from(format!("{}/.local/share/flatpak/exports/share/applications", std::env::var("HOME").unwrap_or_default())),
        PathBuf::from("/var/lib/snapd/desktop/applications"),
    ];

    // Also include any snap desktop dirs under /snap/*/current/meta/gui (optional) if they exist
    if let Ok(entries) = fs::read_dir(Path::new("/snap")) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                let snap_meta = p.join("current/meta/gui");
                if snap_meta.exists() {
                    dirs.push(snap_meta);
                }
            }
        }
    }

    // Collect desktop files from each directory (recursively)
    let mut desktop_paths: Vec<PathBuf> = Vec::new();
    for d in dirs.iter() {
        if d.exists() {
            collect_desktop_files_from_dir(d, &mut desktop_paths);
        }
    }

    // Parse each desktop file
    for path in desktop_paths {
        if let Some(app) = parse_desktop_file(&path) {
            apps.push(app);
        }
    }

    // Post-process: resolve icon names to actual file paths once to avoid expensive lookups later
    for app in apps.iter_mut() {
        if let Some(icon_name) = app.icon.as_ref() {
            if let Some(resolved) = resolve_icon(icon_name) {
                app.icon = Some(resolved);
            }
        }
    }

    apps
}