use slog::Logger;

/// Manager de dispositivos de entrada (teclado, mouse, touchpad)
pub struct InputManager {
    log: Logger,
    keyboard: KeyboardState,
    pointer: PointerState,
}

impl InputManager {
    pub fn new(log: Logger) -> Self {
        slog::info!(log, "Inicializando Input Manager");
        
        Self {
            log,
            keyboard: KeyboardState::new(),
            pointer: PointerState::new(),
        }
    }

    /// Procesa eventos de teclado
    pub fn handle_keyboard_event(&mut self, event: KeyboardEvent) -> Option<String> {
        self.keyboard.process_event(event)
    }

    /// Procesa eventos de mouse/touchpad
    pub fn handle_pointer_event(&mut self, event: PointerEvent) {
        self.pointer.process_event(event);
    }
}

/// Estado del teclado
pub struct KeyboardState {
    modifiers: Modifiers,
    pressed_keys: Vec<u32>,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            modifiers: Modifiers::default(),
            pressed_keys: Vec::new(),
        }
    }

    /// Procesa un evento de teclado y retorna el binding si es una combinación válida
    pub fn process_event(&mut self, event: KeyboardEvent) -> Option<String> {
        match event {
            KeyboardEvent::Key { keycode, state } => {
                if state == KeyState::Pressed {
                    self.pressed_keys.push(keycode);
                    self.check_keybinding(keycode)
                } else {
                    self.pressed_keys.retain(|&k| k != keycode);
                    None
                }
            }
            KeyboardEvent::Modifiers { modifiers } => {
                self.modifiers = modifiers;
                None
            }
        }
    }

    fn check_keybinding(&self, keycode: u32) -> Option<String> {
        // Convertir keycode y modificadores a string de binding
        // Por ejemplo: "Super+Return", "Alt+Tab", etc.
        
        let mut binding = String::new();
        
        if self.modifiers.ctrl {
            binding.push_str("Ctrl+");
        }
        if self.modifiers.alt {
            binding.push_str("Alt+");
        }
        if self.modifiers.shift {
            binding.push_str("Shift+");
        }
        if self.modifiers.super_key {
            binding.push_str("Super+");
        }
        
        // Convertir keycode a nombre de tecla
        binding.push_str(&keycode_to_name(keycode));
        
        Some(binding)
    }
}

/// Modificadores de teclado
#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub super_key: bool,
}

/// Eventos de teclado
#[derive(Debug, Clone)]
pub enum KeyboardEvent {
    Key { keycode: u32, state: KeyState },
    Modifiers { modifiers: Modifiers },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyState {
    Pressed,
    Released,
}

/// Estado del puntero (mouse/touchpad)
pub struct PointerState {
    pub x: f64,
    pub y: f64,
    pub buttons: Vec<u32>,
}

impl PointerState {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            buttons: Vec::new(),
        }
    }

    pub fn process_event(&mut self, event: PointerEvent) {
        match event {
            PointerEvent::Motion { x, y } => {
                self.x = x;
                self.y = y;
            }
            PointerEvent::Button { button, state } => {
                if state == ButtonState::Pressed {
                    self.buttons.push(button);
                } else {
                    self.buttons.retain(|&b| b != button);
                }
            }
            PointerEvent::Axis { axis } => {
                // Manejo de scroll
            }
        }
    }
}

/// Eventos de puntero
#[derive(Debug, Clone)]
pub enum PointerEvent {
    Motion { x: f64, y: f64 },
    Button { button: u32, state: ButtonState },
    Axis { axis: f64 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonState {
    Pressed,
    Released,
}

/// Convierte un keycode a nombre de tecla
fn keycode_to_name(keycode: u32) -> String {
    // Mapeo básico de keycodes a nombres
    // En una implementación real, esto usaría xkbcommon
    match keycode {
        36 => "Return".to_string(),
        65 => "Space".to_string(),
        24 => "Q".to_string(),
        41 => "F".to_string(),
        58 => "M".to_string(),
        57 => "N".to_string(),
        114 => "Right".to_string(),
        113 => "Left".to_string(),
        10..=18 => format!("{}", keycode - 9),  // Teclas numéricas 1-9
        _ => format!("Key{}", keycode),
    }
}

/// Configuración de dispositivos de entrada
pub struct InputConfig {
    /// Aceleración del mouse
    pub pointer_acceleration: f64,
    /// Velocidad de repetición del teclado (caracteres por segundo)
    pub keyboard_repeat_rate: u32,
    /// Delay antes de empezar a repetir (ms)
    pub keyboard_repeat_delay: u32,
    /// Tap-to-click en touchpad
    pub touchpad_tap_to_click: bool,
    /// Scroll natural
    pub touchpad_natural_scroll: bool,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            pointer_acceleration: 0.0,
            keyboard_repeat_rate: 25,
            keyboard_repeat_delay: 600,
            touchpad_tap_to_click: true,
            touchpad_natural_scroll: true,
        }
    }
}
