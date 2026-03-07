pub mod api;
pub mod config;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|_app| {
            let _app_config = config::AppConfig::init();
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        // Daftarkan handler kamu di sini!
        .invoke_handler(tauri::generate_handler![
            handlers::log_handler::create_log_command,
            handlers::pricelist_handler::get_pricelist_command,
            handlers::auth_handler::login_email_command,
            handlers::auth_handler::get_google_auth_url_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
