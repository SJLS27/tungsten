# Contribuyendo a Tungsten

¡Gracias por tu interés en contribuir a Tungsten! Este documento proporciona pautas y mejores prácticas para contribuir al proyecto.

## 🎯 Código de Conducta

Este proyecto se adhiere a un código de conducta. Al participar, se espera que mantengas este código. Por favor, reporta comportamientos inaceptables a los maintainers del proyecto.

## 🚀 Cómo Contribuir

### Reportar Bugs

Los bugs se rastrean como issues de GitHub. Cuando crees un issue de bug, incluye:

- **Título descriptivo y claro**
- **Pasos exactos para reproducir el problema**
- **Comportamiento esperado vs. comportamiento actual**
- **Información del sistema** (distribución Linux, versión de Wayland, etc.)
- **Logs relevantes** (ejecuta `tungsten --log-level debug`)

### Sugerir Mejoras

Las sugerencias de mejora también se rastrean como issues. Incluye:

- **Título descriptivo**
- **Descripción detallada de la funcionalidad sugerida**
- **Por qué esta mejora sería útil**
- **Ejemplos de uso** si es posible

### Pull Requests

1. **Fork el repositorio** y crea tu rama desde `main`
2. **Escribe código** siguiendo las pautas de estilo
3. **Agrega tests** si aplica
4. **Actualiza la documentación** si cambias APIs
5. **Asegúrate de que el código compile** sin warnings
6. **Ejecuta** `cargo fmt` y `cargo clippy`
7. **Escribe un commit message** descriptivo
8. **Abre un Pull Request**

## 📝 Pautas de Código

### Estilo de Rust

- Seguimos las [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Usa `cargo fmt` para formatear el código
- Usa `cargo clippy` para linting
- Todos los `pub` items deben tener documentación
- Preferir expresiones sobre statements cuando sea idiomático

### Documentación

```rust
/// Descripción breve de lo que hace la función.
///
/// Descripción más detallada si es necesario, incluyendo
/// comportamientos especiales o efectos secundarios.
///
/// # Argumentos
///
/// * `arg1` - Descripción del argumento
/// * `arg2` - Descripción del argumento
///
/// # Ejemplos
///
/// ```
/// let resultado = funcion(arg1, arg2);
/// assert_eq!(resultado, valor_esperado);
/// ```
///
/// # Errores
///
/// Esta función retornará un error si...
pub fn funcion(arg1: Type1, arg2: Type2) -> Result<Type3, Error> {
    // implementación
}
```

### Mensajes de Commit

Seguimos [Conventional Commits](https://www.conventionalcommits.org/):

```
<tipo>(<ámbito>): <descripción>

[cuerpo opcional]

[footer(s) opcional(es)]
```

Tipos:
- `feat`: Nueva característica
- `fix`: Corrección de bug
- `docs`: Solo cambios en documentación
- `style`: Cambios que no afectan el significado del código
- `refactor`: Cambio de código que no corrige un bug ni añade una característica
- `perf`: Cambio que mejora el rendimiento
- `test`: Añadir o corregir tests
- `chore`: Cambios en el proceso de build o herramientas auxiliares

Ejemplos:
```
feat(panel): añadir widget de batería

fix(input): corregir manejo de modificadores de teclado

docs(readme): actualizar instrucciones de instalación
```

## 🏗️ Estructura del Proyecto

```
tungsten/
├── src/
│   ├── main.rs          # Punto de entrada
│   ├── config.rs        # Sistema de configuración
│   ├── state.rs         # Estado del compositor
│   ├── handlers.rs      # Manejadores de eventos
│   ├── shell.rs         # Protocolos Wayland
│   ├── input.rs         # Gestión de entrada
│   └── render.rs        # Motor de renderizado
├── config/              # Configuraciones de ejemplo
├── docs/                # Documentación adicional
└── tests/               # Tests de integración
```

## 🧪 Testing

```bash
# Ejecutar todos los tests
cargo test

# Ejecutar tests con output
cargo test -- --nocapture

# Ejecutar un test específico
cargo test nombre_del_test
```

## 🔍 Debugging

Para depurar Tungsten:

```bash
# Ejecutar con logs de debug
RUST_LOG=debug cargo run

# Ejecutar con logs de trace (muy verbose)
RUST_LOG=trace cargo run

# Ejecutar con logs específicos de módulo
RUST_LOG=tungsten::render=debug cargo run
```

## 📚 Recursos Útiles

### Wayland y Compositors
- [Wayland Book](https://wayland-book.com/)
- [Smithay Documentation](https://smithay.github.io/smithay/)
- [Wayland Protocol](https://wayland.freedesktop.org/docs/html/)

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## 🎓 Primeros Pasos para Contribuidores

Si eres nuevo en Tungsten, busca issues etiquetados con:
- `good first issue`: Tareas ideales para comenzar
- `help wanted`: Áreas donde necesitamos ayuda
- `documentation`: Mejoras en documentación

## 📞 Contacto

¿Tienes preguntas? No dudes en:
- Abrir un issue en GitHub
- Unirte a nuestras discusiones
- Contactar a los maintainers

## 🙏 Reconocimiento

Todos los contribuidores serán reconocidos en nuestro README y releases.

---

¡Gracias nuevamente por contribuir a Tungsten! 🎉
