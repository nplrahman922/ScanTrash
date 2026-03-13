use tauri::{Emitter};
use tauri_plugin_deep_link::DeepLinkExt;
use crate::services::{auth_service, session_service, log_service};

#[tauri::command]
pub fn get_google_auth_url_command() -> String {
    auth_service::generate_google_auth_url()
}

// Perhatikan: Kita BAHKAN TIDAK PERLU lagi parameter 'state: tauri::State' di sini!
#[tauri::command]
pub async fn check_auth_status_command(app_handle: tauri::AppHandle) -> Result<bool, String> {
    
    // 1. Minta sesi ke Gudang
    let (access_opt, refresh_opt) = session_service::get_session(&app_handle);

    if access_opt.is_none() || refresh_opt.is_none() {
        return Ok(false);
    }

    let access_token = access_opt.unwrap();
    let refresh_token = refresh_opt.unwrap();

    // 2. Ping Supabase
    if auth_service::validate_token(&access_token).await {
        println!("✅ [RUST] Token divalidasi dan masih hidup!");
        return Ok(true); 
    }

    println!("🔄 [RUST] Token Expired! Sedang mencoba Auto-Refresh...");
    
    // 3. Auto Refresh jika expired
    match auth_service::refresh_access_token(&refresh_token).await {
        Ok((new_access, new_refresh)) => {
            // Suruh Gudang simpan token baru
            session_service::save_session(&app_handle, &new_access, &new_refresh);
            println!("✅ [RUST] Auto-Refresh Sukses! Brankas RAM & Disk diperbarui.");
            
            // Print Debug (Bisa dihapus nanti)
            println!("\n🔑 [DEBUG] ACCESS TOKEN BARU:\n{}\n", new_access);
            
            Ok(true) 
        }
        Err(err) => {
            println!("❌ [RUST] Auto-Refresh Gagal: {}. Menghapus data...", err);
            session_service::clear_session(&app_handle); // Bersihkan Gudang
            Ok(false) 
        }
    }
}

pub fn init_deep_link_listener(app_handle: tauri::AppHandle) {
    let handle_clone = app_handle.clone();

    app_handle.deep_link().on_open_url(move |event| {
        for url_obj in event.urls() {
            let url = url_obj.to_string();
            println!("🔥 [RUST] DEEPLINK MASUK: {}", url);

            // 1. Serahkan tugas membedah URL ke Service!
            if let Some((access_token, refresh_token)) = auth_service::parse_tokens_from_url(&url) {
                
                // Print Debug
                println!("\n🔑 [DEBUG] ACCESS TOKEN BARU:\n{}\n", access_token);

                let bg_handle = handle_clone.clone();

                // 2. Lempar ke Background Thread
                tauri::async_runtime::spawn(async move {
                    // Simpan pakai Service
                    session_service::save_session(&bg_handle, &access_token, &refresh_token);
                    println!("💾 [RUST] Token berhasil diamankan ke Persistent Storage!");
                    
                    let _ = log_service::insert_log_to_supabase("INFO", "User berhasil login ke aplikasi melalui Google OAuth.", &access_token).await;

                    // Kirim sinyal ke Vue
                    if let Err(e) = bg_handle.emit("login-success", "Login tervalidasi di backend!") {
                        println!("⚠️ [RUST] UI belum siap menerima sinyal... Error: {}", e);
                    }
                });
            }
        }
    });
}

#[tauri::command]
pub async fn logout_command(app_handle: tauri::AppHandle) -> Result<(), String> {
    // 1. Ambil token lama sebelum dihapus
    let (access_opt, _) = session_service::get_session(&app_handle);

    if let Some(token) = &access_opt {
    
    // [LOG SYSTEM] Cukup 1 baris ini saja! Service yang akan mengurus sisanya.
    let _ = log_service::insert_log_to_supabase("INFO", "User secara sadar melakukan logout dari perangkat.", token).await;

    auth_service::logout_from_server(token).await;
    println!("🔌 [RUST] Sesi dimatikan dari server Supabase.");
    }

    // 4. Suruh gudang sapu bersih
    session_service::clear_session(&app_handle);
    println!("🗑️ [RUST] Token berhasil dihapus dari Disk dan RAM!");

    Ok(())
}