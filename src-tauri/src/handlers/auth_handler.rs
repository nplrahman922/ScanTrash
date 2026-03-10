use crate::services::auth_service;
use tauri::{AppHandle, Manager, Emitter};
use crate::models::AppState;
use tauri_plugin_deep_link::DeepLinkExt;

#[tauri::command]
pub fn get_google_auth_url_command() -> String {
    auth_service::generate_google_auth_url()
}

#[tauri::command]
pub fn check_auth_status_command(state: tauri::State<'_, AppState>) -> bool {
    // Buka brankas, cek apakah ada tokennya?
    let token = state.access_token.lock().unwrap();
    token.is_some() // Akan return 'true' kalau token ada, 'false' kalau kosong
}

pub fn init_deep_link_listener(app_handle: AppHandle) {
    // 1. Buat "salinan" dari app_handle khusus untuk dibawa ke dalam closure
    let handle_clone = app_handle.clone();

    app_handle.deep_link().on_open_url(move |event| {
        // Event langsung berisi daftar URL, tidak perlu parse JSON manual lagi!
        for url_obj in event.urls() {
            let url = url_obj.to_string();
            
            println!("🔥 [RUST] DEEPLINK MASUK: {}", url);

            // Cek apakah ini URL balikan dari Supabase
            if url.starts_with("com.users.scantrash://auth") {
                let hash_part = url.split('#').nth(1).unwrap_or("");
                let mut access_token = String::new();
                let mut refresh_token = String::new();

                // Bedah token secara manual
                for pair in hash_part.split('&') {
                    let mut kv = pair.split('=');
                    if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                        if k == "access_token" { access_token = v.to_string(); }
                        if k == "refresh_token" { refresh_token = v.to_string(); }
                    }
                }

                if !access_token.is_empty() {
                    // Kunci token di dalam RAM (Mutex) Rust
                    let state = handle_clone.state::<AppState>();
                    *state.access_token.lock().unwrap() = Some(access_token);
                    *state.refresh_token.lock().unwrap() = Some(refresh_token);
                    
                    // Kirim sinyal hijau ke Vue
                    println!("✅ [RUST] TOKEN BERHASIL DIKUNCI DI RAM!");
                    handle_clone.emit("login-success", "Login tervalidasi di backend!").unwrap();
                }
            }
        }
    });
}