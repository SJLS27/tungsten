use clap::Parser;
use slog::{Drain, Logger};
use std::path::PathBuf;

mod config;
mod state;
mod handlers;
mod shell;
mod input;
mod render;

use config::TungstenConfig;
use state::TungstenState;

#[derive(Parser, Debug)]
#[command(name = "tungsten")]
#[command(about = "Tungsten - Un compositor Wayland ligero inspirado en XFCE", long_about = None)]
struct Args {
    /// Archivo de configuración
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Nivel de log (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Ejecutar en modo debug
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    // Configurar el logger
    let log_level = match args.log_level.as_str() {
        "trace" => slog::Level::Trace,
        "debug" => slog::Level::Debug,
        "info" => slog::Level::Info,
        "warn" => slog::Level::Warning,
        "error" => slog::Level::Error,
        _ => slog::Level::Info,
    };

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let drain = drain.filter_level(log_level).fuse();
    let log = Logger::root(drain, slog::o!());

    slog::info!(log, "Iniciando Tungsten Window Manager");
    slog::info!(log, "Versión: {}", env!("CARGO_PKG_VERSION"));

    // Cargar configuración
    let config_path = args.config.unwrap_or_else(|| {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("tungsten");
        path.push("tungsten.toml");
        path
    });

    let config = match TungstenConfig::load(&config_path) {
        Ok(cfg) => {
            slog::info!(log, "Configuración cargada desde: {:?}", config_path);
            cfg
        }
        Err(e) => {
            slog::warn!(log, "No se pudo cargar la configuración: {}. Usando valores por defecto.", e);
            TungstenConfig::default()
        }
    };

    slog::info!(log, "Espacios de trabajo: {}", config.workspaces.count);
    slog::info!(log, "Panel habilitado: {}", config.panel.enabled);
    slog::info!(log, "Animaciones: {}", config.general.animations);

    // Inicializar el compositor
    match run_compositor(log.clone(), config, args.debug) {
        Ok(_) => {
            slog::info!(log, "Tungsten finalizado correctamente");
        }
        Err(e) => {
            slog::error!(log, "Error ejecutando el compositor: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_compositor(
    log: Logger,
    config: TungstenConfig,
    debug: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    slog::info!(log, "Inicializando compositor Wayland");

    // Aquí iría la inicialización completa del compositor usando Smithay
    // Por ahora, mostramos la estructura básica
    
    slog::info!(log, "Compositor inicializado");
    slog::info!(log, "Esperando conexiones de clientes Wayland...");
    
    // Loop principal del compositor
    // let mut event_loop = calloop::EventLoop::try_new()?;
    // let state = TungstenState::new(log.clone(), config);
    
    // event_loop.run(None, &mut state, |_| {})?;

    Ok(())
}
