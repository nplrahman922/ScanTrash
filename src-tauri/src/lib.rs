use std::sync::Mutex;
use crate::models::AppState;

pub mod api;
pub mod config;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 1. Daftarkan Brankas State di sini agar bisa diakses oleh semua handler
        .manage(AppState {
            access_token: Mutex::new(None),
            refresh_token: Mutex::new(None),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        // 2. Hapus underscore (_) pada app karena sekarang kita menggunakannya
        .setup(|app| {
            let _app_config = config::AppConfig::init();
            
            // 3. Pasang telinga (listener) deeplink di background
            crate::handlers::auth_handler::init_deep_link_listener(app.handle().clone());
            
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        // Daftarkan handler kamu di sini!
        .invoke_handler(tauri::generate_handler![
            handlers::log_handler::create_log_command,
            handlers::local_log_handler::write_local_log_command,
            handlers::local_log_handler::read_local_log_command,
            handlers::pricelist_handler::get_pricelist_command,
            handlers::auth_handler::get_google_auth_url_command,
            handlers::auth_handler::check_auth_status_command,
            handlers::auth_handler::logout_command,
            handlers::profile_handler::get_profile_command,
            handlers::scan_handler::analyze_trash_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
