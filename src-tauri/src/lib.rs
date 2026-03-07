pub mod config;
pub mod models;
pub mod api;
pub mod handlers;
pub mod middleware;
pub mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            let _app_config = config::AppConfig::init();
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        // Daftarkan handler kamu di sini!
        .invoke_handler(tauri::generate_handler![
            handlers::log_handler::create_log_command,
            handlers::pricelist_handler::get_pricelist_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}