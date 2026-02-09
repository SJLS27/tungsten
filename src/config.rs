use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TungstenConfig {
    pub general: GeneralConfig,
    pub keybindings: KeybindingsConfig,
    pub workspaces: WorkspacesConfig,
    pub panel: PanelConfig,
    pub window_management: WindowManagementConfig,
    pub decorations: DecorationsConfig,
    pub effects: EffectsConfig,
    pub applications: ApplicationsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub theme: String,
    pub animations: bool,
    pub animation_duration: u32,
    pub compositing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingsConfig {
    pub terminal: String,
    pub launcher: String,
    pub close_window: String,
    pub fullscreen: String,
    pub maximize: String,
    pub minimize: String,
    pub workspace_next: String,
    pub workspace_prev: String,
    pub workspace_1: String,
    pub workspace_2: String,
    pub workspace_3: String,
    pub workspace_4: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspacesConfig {
    pub count: usize,
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelConfig {
    pub enabled: bool,
    pub height: u32,
    pub position: String,
    pub opacity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowManagementConfig {
    pub border_width: u32,
    pub active_border_color: [f32; 4],
    pub inactive_border_color: [f32; 4],
    pub gaps_inner: u32,
    pub gaps_outer: u32,
    pub focus_follows_mouse: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecorationsConfig {
    pub server_side: bool,
    pub buttons: Vec<String>,
    pub titlebar_height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectsConfig {
    pub shadows: bool,
    pub blur: bool,
    pub inactive_opacity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationsConfig {
    pub terminal: String,
    pub launcher: String,
    pub file_manager: String,
}

impl Default for TungstenConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                theme: "default".to_string(),
                animations: true,
                animation_duration: 200,
                compositing: true,
            },
            keybindings: KeybindingsConfig {
                terminal: "Super+Return".to_string(),
                launcher: "Super+Space".to_string(),
                close_window: "Super+Q".to_string(),
                fullscreen: "Super+F".to_string(),
                maximize: "Super+M".to_string(),
                minimize: "Super+N".to_string(),
                workspace_next: "Super+Right".to_string(),
                workspace_prev: "Super+Left".to_string(),
                workspace_1: "Super+1".to_string(),
                workspace_2: "Super+2".to_string(),
                workspace_3: "Super+3".to_string(),
                workspace_4: "Super+4".to_string(),
            },
            workspaces: WorkspacesConfig {
                count: 4,
                names: vec![
                    "Principal".to_string(),
                    "Trabajo".to_string(),
                    "Web".to_string(),
                    "Media".to_string(),
                ],
            },
            panel: PanelConfig {
                enabled: true,
                height: 32,
                position: "top".to_string(),
                opacity: 0.9,
            },
            window_management: WindowManagementConfig {
                border_width: 2,
                active_border_color: [0.3, 0.5, 0.8, 1.0],
                inactive_border_color: [0.2, 0.2, 0.2, 0.6],
                gaps_inner: 8,
                gaps_outer: 4,
                focus_follows_mouse: false,
            },
            decorations: DecorationsConfig {
                server_side: true,
                buttons: vec!["minimize".to_string(), "maximize".to_string(), "close".to_string()],
                titlebar_height: 28,
            },
            effects: EffectsConfig {
                shadows: true,
                blur: false,
                inactive_opacity: 0.95,
            },
            applications: ApplicationsConfig {
                terminal: "foot".to_string(),
                launcher: "wofi --show drun".to_string(),
                file_manager: "thunar".to_string(),
            },
        }
    }
}

impl TungstenConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: TungstenConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(path, toml_string)?;
        Ok(())
    }
}
