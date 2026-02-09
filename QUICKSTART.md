# Tungsten - Guía de Inicio Rápido

## 📁 Estructura del Proyecto

```
tungsten/
├── src/
│   ├── main.rs          # Punto de entrada principal
│   ├── config.rs        # Sistema de configuración con TOML
│   ├── state.rs         # Estado del compositor (ventanas, workspaces)
│   ├── handlers.rs      # Manejadores de eventos (teclado, mouse, ventanas)
│   ├── shell.rs         # Protocolos Wayland (XDG Shell, Layer Shell)
│   ├── input.rs         # Gestión de dispositivos de entrada
│   └── render.rs        # Motor de renderizado con efectos
│
├── config/
│   └── tungsten.toml    # Configuración por defecto
│
├── Cargo.toml           # Dependencias del proyecto
├── README.md            # Documentación completa
├── CONTRIBUTING.md      # Guía para contribuidores
├── CHANGELOG.md         # Historial de cambios
├── LICENSE              # Licencia MIT
├── install.sh           # Script de instalación
├── tungsten.desktop     # Archivo de sesión Wayland
└── .gitignore          # Archivos a ignorar en git
```

## 🚀 Primeros Pasos

### 1. Instalar Dependencias

**Arch Linux:**
```bash
sudo pacman -S rust wayland wayland-protocols libxkbcommon libinput mesa seatd
```

**Ubuntu/Debian:**
```bash
sudo apt install cargo libwayland-dev libxkbcommon-dev libinput-dev \
                 libudev-dev libdrm-dev libgbm-dev libgl1-mesa-dev libseat-dev
```

**Fedora:**
```bash
sudo dnf install cargo wayland-devel wayland-protocols-devel libxkbcommon-devel \
                 libinput-devel systemd-devel libdrm-devel mesa-libgbm-devel \
                 mesa-libGL-devel libseat-devel
```

### 2. Compilar el Proyecto

```bash
cd tungsten
cargo build --release
```

### 3. Instalar (Opcional)

```bash
# Usar el script de instalación
./install.sh

# O instalar manualmente
sudo install -Dm755 target/release/tungsten /usr/local/bin/tungsten
mkdir -p ~/.config/tungsten
cp config/tungsten.toml ~/.config/tungsten/
sudo cp tungsten.desktop /usr/share/wayland-sessions/
```

### 4. Ejecutar

```bash
# Desde la consola (TTY)
tungsten

# Con configuración personalizada
tungsten --config mi-config.toml

# Con logs de debug
tungsten --log-level debug
```

## 🎨 Características Principales

### Gestión de Ventanas
- ✅ Múltiples espacios de trabajo (4 por defecto)
- ✅ Modos: Tiling, Floating, Maximizado, Stacking
- ✅ Decoraciones del lado del servidor (bordes, barras de título)
- ✅ Gaps configurables entre ventanas

### Panel Superior
- ✅ Panel personalizable estilo XFCE
- ✅ Widgets: menú de aplicaciones, lista de ventanas, reloj, system tray
- ✅ Posición configurable (top, bottom, left, right)
- ✅ Transparencia ajustable

### Efectos Visuales
- ✅ Sombras de ventanas
- ✅ Animaciones (fade in/out, slide)
- ✅ Transparencia de ventanas inactivas
- ✅ Composición con OpenGL

### Atajos de Teclado
- ✅ Completamente configurables
- ✅ Soporta Super, Ctrl, Alt, Shift
- ✅ Bindings para workspaces, ventanas, aplicaciones

## ⚙️ Configuración Básica

Edita `~/.config/tungsten/tungsten.toml`:

```toml
[general]
animations = true
animation_duration = 200

[keybindings]
terminal = "Super+Return"
launcher = "Super+Space"
close_window = "Super+Q"

[window_management]
border_width = 2
gaps_inner = 8
gaps_outer = 4

[panel]
enabled = true
height = 32
position = "top"
```

## 🔧 Próximos Pasos para Desarrollo

### Implementación Pendiente

El proyecto actualmente tiene la estructura y arquitectura completa, pero necesita:

1. **Implementación del compositor Smithay**
   - Inicializar el backend (DRM, libinput)
   - Crear el loop de eventos con calloop
   - Implementar los handlers de Smithay

2. **XDG Shell completo**
   - Manejo de superficies toplevel
   - Popups y subventanas
   - Gestión de estados (maximizado, fullscreen)

3. **Renderizado con OpenGL**
   - Inicializar contexto GL
   - Renderizar texturas de ventanas
   - Implementar efectos (sombras, blur)

4. **Input handling**
   - Integración con libinput
   - Distribución de eventos a ventanas
   - Manejo de focus

### Archivos Clave para Extender

- `src/main.rs`: Función `run_compositor()` necesita implementación
- `src/shell.rs`: Implementar handlers de XDG Shell
- `src/render.rs`: Implementar renderizado con OpenGL
- `src/input.rs`: Integrar con libinput de Smithay

### Referencias Útiles

```rust
// Ejemplo de inicialización básica con Smithay
use smithay::backend::drm::DrmDevice;
use smithay::backend::libinput::LibinputInputBackend;
use smithay::reexports::calloop::EventLoop;

// Ver documentación de Smithay para ejemplos completos
// https://smithay.github.io/smithay/
```

## 📖 Recursos

- **README.md**: Documentación completa del proyecto
- **CONTRIBUTING.md**: Guía para contribuidores
- **CHANGELOG.md**: Historial de versiones
- [Smithay Docs](https://smithay.github.io/smithay/)
- [Wayland Book](https://wayland-book.com/)

## 🐛 Depuración

```bash
# Logs detallados
RUST_LOG=tungsten=debug cargo run

# Logs de Smithay también
RUST_LOG=tungsten=debug,smithay=debug cargo run

# Solo un módulo específico
RUST_LOG=tungsten::render=trace cargo run
```

## 💡 Tips

1. **Empieza simple**: Primero haz que funcione un compositor básico sin efectos
2. **Usa Smithay examples**: El repositorio de Smithay tiene ejemplos excelentes
3. **Debugging visual**: Añade logs en el render loop para entender el flujo
4. **Testing**: Prueba primero con `weston-terminal` o aplicaciones Wayland simples

---

¡Buena suerte con tu window manager! 🚀
