use slog::Logger;
use std::process::Command;

use crate::config::TungstenConfig;
use crate::state::TungstenState;

/// Manejador de eventos del teclado
pub struct KeyboardHandler {
    log: Logger,
    config: TungstenConfig,
}

impl KeyboardHandler {
    pub fn new(log: Logger, config: TungstenConfig) -> Self {
        Self { log, config }
    }

    /// Procesa una combinación de teclas
    pub fn handle_keybinding(&self, state: &mut TungstenState, binding: &str) {
        slog::debug!(self.log, "Procesando keybinding: {}", binding);

        // Terminal
        if binding == self.config.keybindings.terminal {
            self.launch_terminal();
        }
        // Launcher
        else if binding == self.config.keybindings.launcher {
            self.launch_application_launcher();
        }
        // Cerrar ventana
        else if binding == self.config.keybindings.close_window {
            state.close_focused_window();
        }
        // Pantalla completa
        else if binding == self.config.keybindings.fullscreen {
            state.toggle_fullscreen_focused();
        }
        // Maximizar
        else if binding == self.config.keybindings.maximize {
            state.toggle_maximize_focused();
        }
        // Navegación entre workspaces
        else if binding == self.config.keybindings.workspace_next {
            state.next_workspace();
        }
        else if binding == self.config.keybindings.workspace_prev {
            state.prev_workspace();
        }
        // Workspaces específicos
        else if binding == self.config.keybindings.workspace_1 {
            state.goto_workspace(0);
        }
        else if binding == self.config.keybindings.workspace_2 {
            state.goto_workspace(1);
        }
        else if binding == self.config.keybindings.workspace_3 {
            state.goto_workspace(2);
        }
        else if binding == self.config.keybindings.workspace_4 {
            state.goto_workspace(3);
        }
    }

    fn launch_terminal(&self) {
        let terminal = &self.config.applications.terminal;
        slog::info!(self.log, "Lanzando terminal: {}", terminal);
        
        if let Err(e) = Command::new(terminal).spawn() {
            slog::error!(self.log, "Error lanzando terminal: {}", e);
        }
    }

    fn launch_application_launcher(&self) {
        let launcher = &self.config.applications.launcher;
        slog::info!(self.log, "Lanzando launcher: {}", launcher);
        
        let parts: Vec<&str> = launcher.split_whitespace().collect();
        if let Some((cmd, args)) = parts.split_first() {
            if let Err(e) = Command::new(cmd).args(args).spawn() {
                slog::error!(self.log, "Error lanzando launcher: {}", e);
            }
        }
    }
}

/// Manejador de eventos del mouse
pub struct PointerHandler {
    log: Logger,
}

impl PointerHandler {
    pub fn new(log: Logger) -> Self {
        Self { log }
    }

    /// Maneja el movimiento del mouse
    pub fn handle_motion(&self, _state: &mut TungstenState, x: f64, y: f64) {
        slog::trace!(self.log, "Mouse motion: ({}, {})", x, y);
    }

    /// Maneja clicks del mouse
    pub fn handle_button(&self, state: &mut TungstenState, button: u32, pressed: bool) {
        if pressed {
            slog::debug!(self.log, "Mouse button pressed: {}", button);
            
            // Aquí iría la lógica para determinar qué ventana fue clickeada
            // y establecer el foco en ella
        }
    }

    /// Maneja el scroll del mouse
    pub fn handle_axis(&self, _state: &mut TungstenState, axis: f64) {
        slog::trace!(self.log, "Mouse axis: {}", axis);
    }
}

/// Manejador de eventos de ventanas
pub struct WindowHandler {
    log: Logger,
}

impl WindowHandler {
    pub fn new(log: Logger) -> Self {
        Self { log }
    }

    /// Maneja el mapeo de una nueva ventana
    pub fn handle_map_request(&self, state: &mut TungstenState, window_id: u64, title: String, app_id: String) {
        use crate::state::{Window, Geometry};

        slog::info!(self.log, "Nueva ventana: {} ({})", title, app_id);

        let window = Window {
            id: window_id,
            title,
            app_id,
            geometry: Geometry {
                x: 100,
                y: 100,
                width: 800,
                height: 600,
            },
            is_focused: false,
            is_maximized: false,
            is_fullscreen: false,
            is_minimized: false,
            workspace_id: state.current_workspace,
        };

        state.add_window(window);
    }

    /// Maneja el cierre de una ventana
    pub fn handle_unmap(&self, state: &mut TungstenState, window_id: u64) {
        slog::info!(self.log, "Ventana cerrada: {}", window_id);
        state.remove_window(window_id);
    }

    /// Maneja cambios en el título de la ventana
    pub fn handle_title_changed(&self, state: &mut TungstenState, window_id: u64, new_title: String) {
        if let Some(window) = state.windows.get_mut(&window_id) {
            slog::debug!(self.log, "Título cambiado: {} -> {}", window.title, new_title);
            window.title = new_title;
        }
    }
}
