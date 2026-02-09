use slog::Logger;
use std::collections::HashMap;

use crate::config::TungstenConfig;

/// Estado principal del compositor Tungsten
pub struct TungstenState {
    pub log: Logger,
    pub config: TungstenConfig,
    pub workspaces: Vec<Workspace>,
    pub current_workspace: usize,
    pub windows: HashMap<u64, Window>,
    pub focus_stack: Vec<u64>,
}

/// Representa un espacio de trabajo (workspace)
pub struct Workspace {
    pub id: usize,
    pub name: String,
    pub windows: Vec<u64>,
    pub layout: LayoutMode,
}

/// Modos de layout disponibles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutMode {
    Tiling,      // Mosaico automático
    Floating,    // Ventanas flotantes
    Maximized,   // Una ventana maximizada
    Stacking,    // Ventanas apiladas
}

/// Representa una ventana en el compositor
pub struct Window {
    pub id: u64,
    pub title: String,
    pub app_id: String,
    pub geometry: Geometry,
    pub is_focused: bool,
    pub is_maximized: bool,
    pub is_fullscreen: bool,
    pub is_minimized: bool,
    pub workspace_id: usize,
}

/// Geometría de una ventana
#[derive(Debug, Clone, Copy)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl TungstenState {
    pub fn new(log: Logger, config: TungstenConfig) -> Self {
        let workspace_count = config.workspaces.count;
        let mut workspaces = Vec::with_capacity(workspace_count);

        for i in 0..workspace_count {
            let name = config
                .workspaces
                .names
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("Workspace {}", i + 1));

            workspaces.push(Workspace {
                id: i,
                name,
                windows: Vec::new(),
                layout: LayoutMode::Tiling,
            });
        }

        slog::info!(log, "Inicializado estado con {} espacios de trabajo", workspace_count);

        Self {
            log,
            config,
            workspaces,
            current_workspace: 0,
            windows: HashMap::new(),
            focus_stack: Vec::new(),
        }
    }

    /// Cambia al siguiente espacio de trabajo
    pub fn next_workspace(&mut self) {
        self.current_workspace = (self.current_workspace + 1) % self.workspaces.len();
        slog::info!(
            self.log,
            "Cambiado a workspace: {}",
            self.workspaces[self.current_workspace].name
        );
    }

    /// Cambia al espacio de trabajo anterior
    pub fn prev_workspace(&mut self) {
        if self.current_workspace == 0 {
            self.current_workspace = self.workspaces.len() - 1;
        } else {
            self.current_workspace -= 1;
        }
        slog::info!(
            self.log,
            "Cambiado a workspace: {}",
            self.workspaces[self.current_workspace].name
        );
    }

    /// Cambia a un espacio de trabajo específico
    pub fn goto_workspace(&mut self, index: usize) {
        if index < self.workspaces.len() {
            self.current_workspace = index;
            slog::info!(
                self.log,
                "Cambiado a workspace: {}",
                self.workspaces[self.current_workspace].name
            );
        }
    }

    /// Agrega una nueva ventana al estado
    pub fn add_window(&mut self, window: Window) {
        let id = window.id;
        let workspace_id = window.workspace_id;
        
        self.windows.insert(id, window);
        self.workspaces[workspace_id].windows.push(id);
        self.focus_window(id);
        
        slog::info!(self.log, "Nueva ventana agregada: {}", id);
    }

    /// Elimina una ventana del estado
    pub fn remove_window(&mut self, id: u64) {
        if let Some(window) = self.windows.remove(&id) {
            let workspace = &mut self.workspaces[window.workspace_id];
            workspace.windows.retain(|&win_id| win_id != id);
            self.focus_stack.retain(|&win_id| win_id != id);
            
            slog::info!(self.log, "Ventana eliminada: {}", id);
        }
    }

    /// Establece el foco en una ventana
    pub fn focus_window(&mut self, id: u64) {
        // Remover el foco de la ventana anterior
        for window in self.windows.values_mut() {
            window.is_focused = false;
        }

        // Establecer foco en la nueva ventana
        if let Some(window) = self.windows.get_mut(&id) {
            window.is_focused = true;
            
            // Actualizar stack de foco
            self.focus_stack.retain(|&win_id| win_id != id);
            self.focus_stack.push(id);
            
            slog::debug!(self.log, "Foco establecido en ventana: {}", id);
        }
    }

    /// Obtiene la ventana con foco actual
    pub fn get_focused_window(&self) -> Option<&Window> {
        self.focus_stack
            .last()
            .and_then(|&id| self.windows.get(&id))
    }

    /// Cierra la ventana con foco
    pub fn close_focused_window(&mut self) {
        if let Some(&id) = self.focus_stack.last() {
            self.remove_window(id);
        }
    }

    /// Maximiza/desmaximiza la ventana con foco
    pub fn toggle_maximize_focused(&mut self) {
        if let Some(&id) = self.focus_stack.last() {
            if let Some(window) = self.windows.get_mut(&id) {
                window.is_maximized = !window.is_maximized;
                slog::info!(self.log, "Ventana {} maximizada: {}", id, window.is_maximized);
            }
        }
    }

    /// Pone en pantalla completa la ventana con foco
    pub fn toggle_fullscreen_focused(&mut self) {
        if let Some(&id) = self.focus_stack.last() {
            if let Some(window) = self.windows.get_mut(&id) {
                window.is_fullscreen = !window.is_fullscreen;
                slog::info!(self.log, "Ventana {} fullscreen: {}", id, window.is_fullscreen);
            }
        }
    }
}
