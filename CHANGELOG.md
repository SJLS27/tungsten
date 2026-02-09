# Changelog

Todos los cambios notables en este proyecto serán documentados en este archivo.

El formato está basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.0.0/),
y este proyecto adhiere a [Semantic Versioning](https://semver.org/lang/es/).

## [No publicado]

### En Progreso
- Implementación completa del compositor Wayland usando Smithay
- Soporte completo para XDG Shell
- Sistema de renderizado con OpenGL
- Gestión de input con libinput

## [0.1.0] - 2026-02-09

### Añadido
- Estructura básica del proyecto
- Sistema de configuración con TOML
- Módulos principales:
  - `config`: Gestión de configuración
  - `state`: Estado del compositor
  - `handlers`: Manejadores de eventos
  - `shell`: Protocolos de Wayland
  - `input`: Gestión de dispositivos de entrada
  - `render`: Motor de renderizado
- Soporte para múltiples espacios de trabajo
- Sistema de atajos de teclado configurables
- Panel superior configurable
- Decoraciones de ventanas del lado del servidor
- Sistema de efectos visuales (sombras, animaciones)
- Script de instalación
- Documentación completa en README
- Archivo de sesión de Wayland
- Licencia MIT

### Características Planeadas
- [ ] Soporte completo de XDG Shell
- [ ] Implementación de Layer Shell
- [ ] XWayland para aplicaciones X11
- [ ] Temas personalizables
- [ ] Workspaces dinámicos
- [ ] Gestos de touchpad
- [ ] Soporte multi-monitor completo
- [ ] Integración con PipeWire para screencasting
- [ ] Gestor de energía
- [ ] Notificaciones de escritorio
- [ ] Screenshots integrados
- [ ] Configuración gráfica (GUI)

### Notas
- Este es el primer release de desarrollo
- Muchas características están aún en progreso
- El proyecto está en estado alpha

---

## Guía de Versionado

- **MAJOR**: Cambios incompatibles en la API o configuración
- **MINOR**: Nueva funcionalidad compatible hacia atrás
- **PATCH**: Correcciones de bugs compatibles hacia atrás
