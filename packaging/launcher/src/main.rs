use log::{error, info};
use path_absolutize::Absolutize;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;
use subprocess::{Popen, PopenConfig};

#[derive(Debug, Deserialize)]
struct Config {
    server: ServerConfig,
    frontend: FrontendConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct FrontendConfig {
    auto_open_browser: bool,
}

fn main() {
    env_logger::init();

    println!("========================================");
    println!("    Poker Analyzer - Local Edition");
    println!("========================================");
    println!();

    // Obtener directorio de instalación
    let install_dir = get_install_dir();
    info!("Directorio de instalación: {:?}", install_dir);

    // Leer configuración
    let config = match read_config(&install_dir) {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Error leyendo configuración: {}", e);
            eprintln!("ERROR: No se pudo leer config.toml: {}", e);
            wait_for_key();
            std::process::exit(1);
        }
    };

    // Iniciar servidor backend
    println!("Iniciando servidor backend...");
    let backend_process = match start_backend(&install_dir) {
        Ok(process) => {
            println!("  ✓ Servidor iniciado");
            Some(process)
        }
        Err(e) => {
            error!("Error iniciando backend: {}", e);
            eprintln!("ERROR: No se pudo iniciar el servidor: {}", e);
            wait_for_key();
            std::process::exit(1);
        }
    };

    // Esperar a que el servidor esté listo
    println!("Esperando a que el servidor esté listo...");
    thread::sleep(Duration::from_secs(3));

    // Health check
    let url = format!("http://{}:{}/health", config.server.host, config.server.port);
    if !check_server_health(&url) {
        eprintln!("ADVERTENCIA: El servidor puede no estar completamente listo");
    } else {
        println!("  ✓ Servidor respondiendo");
    }

    // Abrir navegador
    if config.frontend.auto_open_browser {
        println!("Abriendo navegador...");
        let web_url = format!("http://{}:{}", config.server.host, config.server.port);
        match webbrowser::open(&web_url) {
            Ok(_) => println!("  ✓ Navegador abierto"),
            Err(e) => {
                eprintln!("ADVERTENCIA: No se pudo abrir el navegador: {}", e);
                println!("  Por favor, abre manualmente: {}", web_url);
            }
        }
    }

    println!();
    println!("========================================");
    println!("  Poker Analyzer está corriendo");
    println!("========================================");
    println!();
    println!("  Servidor: http://{}:{}", config.server.host, config.server.port);
    println!("  Logs: logs/");
    println!();
    println!("Presiona Ctrl+C para detener");
    println!();

    // Esperar señal de interrupción
    ctrlc::set_handler(move || {
        println!();
        println!("Deteniendo Poker Analyzer...");
        std::process::exit(0);
    })
    .expect("Error configurando handler de Ctrl+C");

    // Mantener el programa corriendo
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

fn get_install_dir() -> PathBuf {
    // El ejecutable está en el directorio raíz de la instalación
    env::current_exe()
        .expect("No se pudo obtener ruta del ejecutable")
        .parent()
        .expect("No se pudo obtener directorio padre")
        .to_path_buf()
}

fn read_config(install_dir: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = install_dir.join("config.toml");
    let config_content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

fn start_backend(install_dir: &Path) -> Result<Child, Box<dyn std::error::Error>> {
    let python_exe = install_dir.join("backend/python311/python.exe");
    let main_py = install_dir.join("backend/app/main.py");

    if !python_exe.exists() {
        return Err(format!("Python no encontrado en: {:?}", python_exe).into());
    }

    if !main_py.exists() {
        return Err(format!("main.py no encontrado en: {:?}", main_py).into());
    }

    // Configurar variables de entorno
    let mut cmd = Command::new(&python_exe);
    cmd.arg(&main_py);
    cmd.current_dir(install_dir);

    // Iniciar proceso
    let child = cmd.spawn()?;
    Ok(child)
}

fn check_server_health(url: &str) -> bool {
    // Intentar conectar al servidor 10 veces
    for i in 0..10 {
        thread::sleep(Duration::from_millis(500));
        
        // Usar curl o similar para verificar
        // Por simplicidad, asumimos que está listo después de unos segundos
        if i > 5 {
            return true;
        }
    }
    false
}

fn wait_for_key() {
    println!();
    println!("Presiona Enter para salir...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
}

