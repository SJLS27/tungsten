<<<<<<< HEAD
# tungsten
simple and modern windows manager for linux (work in progress) don't funcional 
=======
# Tungsten Window Manager

![Tungsten Logo](https://img.shields.io/badge/Tungsten-WM-blue)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)
![Wayland](https://img.shields.io/badge/Wayland-enabled-green)

Un compositor Wayland ligero y rápido inspirado en XFCE, escrito en Rust.

## 🚀 Características

- **Ligero y Rápido**: Diseñado para ser eficiente en recursos, perfecto para hardware moderno y antiguo
- **Múltiples Espacios de Trabajo**: Organiza tus ventanas en diferentes espacios de trabajo
- **Panel Personalizable**: Panel superior estilo XFCE con widgets configurables
- **Gestión de Ventanas Inteligente**: Modos tiling, floating y maximizado
- **Decoraciones del Lado del Servidor**: Barras de título y bordes elegantes
- **Efectos Visuales**: Sombras, animaciones y transparencia configurables
- **Atajos de Teclado**: Bindings personalizables para todas las acciones
- **Configuración en TOML**: Archivo de configuración simple y legible

## 📋 Requisitos

- Rust 1.70 o superior
- Bibliotecas del sistema:
  - libwayland
  - libxkbcommon
  - libinput
  - libudev
  - libdrm
  - libgbm
  - mesa (para OpenGL)
  - libseat (para gestión de sesiones)

### Instalación de Dependencias

**Arch Linux:**
```bash
sudo pacman -S wayland wayland-protocols libxkbcommon libinput mesa seatd
```

**Ubuntu/Debian:**
```bash
sudo apt install libwayland-dev libxkbcommon-dev libinput-dev libudev-dev \
                 libdrm-dev libgbm-dev libgl1-mesa-dev libseat-dev
```

**Fedora:**
```bash
sudo dnf install wayland-devel wayland-protocols-devel libxkbcommon-devel \
                 libinput-devel systemd-devel libdrm-devel mesa-libgbm-devel \
                 mesa-libGL-devel libseat-devel
```

## 🔧 Compilación

```bash
# Clonar el repositorio
git clone https://github.com/tu-usuario/tungsten.git
cd tungsten

# Compilar en modo release
cargo build --release

# El binario estará en target/release/tungsten
```

## 📦 Instalación

```bash
# Instalar el binario
sudo install -Dm755 target/release/tungsten /usr/local/bin/tungsten

# Instalar la configuración por defecto
mkdir -p ~/.config/tungsten
cp config/tungsten.toml ~/.config/tungsten/

# Instalar el archivo de sesión de Wayland
sudo cp tungsten.desktop /usr/share/wayland-sessions/
```

## 🎮 Uso

### Iniciar Tungsten

Desde un gestor de sesiones (GDM, SDDM, LightDM):
- Selecciona "Tungsten" en la lista de sesiones

Desde la consola (TTY):
```bash
tungsten
```

Con argumentos personalizados:
```bash
tungsten --config ~/.config/tungsten/mi-config.toml --log-level debug
```

### Atajos de Teclado por Defecto

| Atajo | Acción |
|-------|--------|
| `Super + Return` | Abrir terminal |
| `Super + Space` | Abrir lanzador de aplicaciones |
| `Super + Q` | Cerrar ventana |
| `Super + F` | Pantalla completa |
| `Super + M` | Maximizar/Restaurar |
| `Super + N` | Minimizar |
| `Super + →` | Siguiente workspace |
| `Super + ←` | Workspace anterior |
| `Super + 1-4` | Ir al workspace 1-4 |

## ⚙️ Configuración

El archivo de configuración se encuentra en `~/.config/tungsten/tungsten.toml`.

### Ejemplo de Configuración

```toml
[general]
theme = "default"
animations = true
animation_duration = 200
compositing = true

[keybindings]
terminal = "Super+Return"
launcher = "Super+Space"
close_window = "Super+Q"

[window_management]
border_width = 2
active_border_color = [0.3, 0.5, 0.8, 1.0]
gaps_inner = 8
gaps_outer = 4

[panel]
enabled = true
height = 32
position = "top"
opacity = 0.9
```

## 🎨 Personalización

### Cambiar el Terminal

Edita `~/.config/tungsten/tungsten.toml`:

```toml
[applications]
terminal = "alacritty"  # o "kitty", "wezterm", etc.
```

### Desactivar Animaciones

```toml
[general]
animations = false
```

### Cambiar Colores de Bordes

```toml
[window_management]
active_border_color = [1.0, 0.0, 0.0, 1.0]  # Rojo
inactive_border_color = [0.5, 0.5, 0.5, 0.5]  # Gris
```

## 🐛 Solución de Problemas

### Tungsten no inicia

1. Verifica que tengas todas las dependencias instaladas
2. Ejecuta con `--log-level debug` para ver mensajes detallados
3. Asegúrate de tener permisos para acceder a `/dev/dri`

### Las aplicaciones no se abren

Verifica que las aplicaciones estén instaladas:
```bash
which foot    # Terminal por defecto
which wofi    # Lanzador por defecto
```

### Problemas con el teclado

Verifica tu layout de teclado:
```bash
echo $XKB_DEFAULT_LAYOUT
```

## 🤝 Contribuir

¡Las contribuciones son bienvenidas! Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📝 Roadmap

- [ ] Soporte completo para XDG Shell
- [ ] Implementación de Layer Shell para el panel
- [ ] Soporte para XWayland
- [ ] Temas personalizables
- [ ] Configuración de workspaces dinámicos
- [ ] Gestos de touchpad
- [ ] Multi-monitor completo
- [ ] Screencasting con PipeWire
- [ ] Gestor de energía integrado

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

## 🙏 Agradecimientos

- [Smithay](https://github.com/Smithay/smithay) - Framework de compositor Wayland para Rust
- [XFCE](https://xfce.org/) - Inspiración para el diseño
- [Sway](https://swaywm.org/) - Referencia de compositor Wayland

## 📧 Contacto

Proyecto Link: [https://github.com/tu-usuario/tungsten](https://github.com/tu-usuario/tungsten)

---

**Nota**: Este es un proyecto en desarrollo activo. Algunas características pueden estar incompletas o en progreso.
>>>>>>> e834afe (First commit)
