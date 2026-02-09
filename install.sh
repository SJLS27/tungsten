#!/bin/bash
# Script de instalación de Tungsten Window Manager

set -e

echo "==================================="
echo "  Instalación de Tungsten WM"
echo "==================================="
echo ""

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' 

# Verificar si estamos ejecutando como root
if [ "$EUID" -eq 0 ]; then 
    echo -e "${RED}Error: No ejecutes este script como root${NC}"
    exit 1
fi

# Función para imprimir con color
print_status() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Verificar dependencias
print_status "Verificando dependencias..."

MISSING_DEPS=()

check_command() {
    if ! command -v $1 &> /dev/null; then
        MISSING_DEPS+=("$1")
    fi
}

check_command cargo
check_command rustc

if [ ${#MISSING_DEPS[@]} -ne 0 ]; then
    print_error "Faltan las siguientes dependencias: ${MISSING_DEPS[*]}"
    echo "Por favor, instala Rust desde https://rustup.rs/"
    exit 1
fi

print_success "Dependencias de Rust encontradas"

# Compilar el proyecto
print_status "Compilando Tungsten en modo release..."
cargo build --release

if [ $? -ne 0 ]; then
    print_error "Error al compilar Tungsten"
    exit 1
fi

print_success "Compilación exitosa"

# Instalar el binario
print_status "Instalando binario en /usr/local/bin..."
sudo install -Dm755 target/release/tungsten /usr/local/bin/tungsten
print_success "Binario instalado"

# Crear directorio de configuración
CONFIG_DIR="$HOME/.config/tungsten"
print_status "Creando directorio de configuración en $CONFIG_DIR..."
mkdir -p "$CONFIG_DIR"

# Copiar configuración por defecto si no existe
if [ ! -f "$CONFIG_DIR/tungsten.toml" ]; then
    print_status "Copiando configuración por defecto..."
    cp config/tungsten.toml "$CONFIG_DIR/"
    print_success "Configuración instalada"
else
    print_status "Configuración existente encontrada, no se sobrescribirá"
fi

# Instalar archivo de sesión
print_status "Instalando archivo de sesión de Wayland..."
sudo install -Dm644 tungsten.desktop /usr/share/wayland-sessions/tungsten.desktop
print_success "Archivo de sesión instalado"

# Verificar permisos de seatd
print_status "Verificando permisos para seatd..."
if groups | grep -q 'seat'; then
    print_success "Usuario ya está en el grupo 'seat'"
else
    print_status "Agregando usuario al grupo 'seat'..."
    sudo usermod -aG seat $USER
    echo -e "${BLUE}NOTA:${NC} Deberás cerrar sesión y volver a entrar para que los cambios de grupo tengan efecto"
fi

echo ""
echo "==================================="
print_success "¡Instalación completada!"
echo "==================================="
echo ""
echo "Puedes iniciar Tungsten de las siguientes formas:"
echo "  1. Desde tu gestor de inicio de sesión (GDM, SDDM, etc.)"
echo "  2. Desde la consola ejecutando: tungsten"
echo ""
echo "Configuración: $CONFIG_DIR/tungsten.toml"
echo ""
echo "Para más información, consulta el README.md"
echo ""
