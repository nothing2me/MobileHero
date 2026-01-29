mod websocket;
mod keyboard;
mod config;

use std::sync::Arc;
use tokio::sync::Mutex;

pub use config::Config;
pub use websocket::ServerState;

use tauri::Manager; // Need Manager trait for checking app state if needed, but path() is on AppHandle

// Helper to get config path
fn get_config_path(app: &tauri::AppHandle) -> std::path::PathBuf {
    // For Tauri v2, we can use:
    // app.path().app_config_dir() -> Result<PathBuf>
    
    // Note: handling unwraps carefully.
    match app.path().app_config_dir() {
        Ok(dir) => dir.join("config.json"),
        Err(_) => {
            // Fallback to local execution dir if app_config_dir fails (e.g. unexpected environment)
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("config.json")
        }
    }
}

#[tauri::command]
async fn start_server(
    state: tauri::State<'_, Arc<Mutex<ServerState>>>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let mut server = state.lock().await;
    
    if server.is_running {
        return Err("Server already running".to_string());
    }
    
    let config_path = get_config_path(&app);
    let config = config::load_config(Some(config_path.clone()));
    let port = config.port;

    // Start the WebSocket server in a background task
    let state_clone = Arc::clone(&state.inner());
    let app_clone = app.clone();
    
    tokio::spawn(async move {
        // We pass the path or handle to run_server?
        // run_server takes app handle, so it can resolve the path itself inside.
        if let Err(e) = websocket::run_server(state_clone, app_clone, port).await {
            eprintln!("Server error: {}", e);
        }
    });
    
    server.is_running = true;
    server.should_stop = false; // Reset shutdown flag
    
    // Get local IP
    let ip = local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    
    Ok(format!("{}:{}", ip, port))
}

#[tauri::command]
async fn stop_server(state: tauri::State<'_, Arc<Mutex<ServerState>>>) -> Result<(), String> {
    let mut server = state.lock().await;
    server.is_running = false;
    server.should_stop = true;
    
    // Signal all connections to close
    let _ = server.shutdown_tx.send(());
    
    Ok(())
}

#[tauri::command]
fn get_local_ip() -> String {
    local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string())
}

#[tauri::command]
fn get_config(app: tauri::AppHandle) -> Config {
    let path = get_config_path(&app);
    config::load_config(Some(path))
}

#[tauri::command]
fn save_config(app: tauri::AppHandle, config: Config) -> Result<(), String> {
    let path = get_config_path(&app);
    config::save_config(&config, path).map_err(|e| e.to_string())
}

#[tauri::command]
fn generate_qr_code(data: String) -> Result<String, String> {
    use qrcode::QrCode;
    use image::Luma;
    use image::ImageEncoder;
    use base64::Engine;
    
    let code = QrCode::new(data.as_bytes()).map_err(|e| e.to_string())?;
    let image = code.render::<Luma<u8>>().build();
    
    let mut png_data: Vec<u8> = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
    encoder.write_image(
        image.as_raw(),
        image.width(),
        image.height(),
        image::ExtendedColorType::L8,
    ).map_err(|e: image::ImageError| e.to_string())?;
    
    let base64 = base64::engine::general_purpose::STANDARD.encode(&png_data);
    Ok(format!("data:image/png;base64,{}", base64))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let server_state = Arc::new(Mutex::new(ServerState::new()));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(server_state)
        .invoke_handler(tauri::generate_handler![
            start_server,
            stop_server,
            get_local_ip,
            get_config,
            save_config,
            generate_qr_code,
        ])
        .setup(|_app| {
            // Initialize the tokio runtime for async operations
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            std::thread::spawn(move || {
                rt.block_on(async {
                    // Keep runtime alive
                    loop {
                        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
                    }
                });
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
