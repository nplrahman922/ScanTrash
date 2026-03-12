use crate::services::auth_service;
use tauri::{AppHandle, Manager, Emitter};
use crate::models::AppState;
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_store::StoreExt;
use serde_json::json;

#[tauri::command]
pub fn get_google_auth_url_command() -> String {
    auth_service::generate_google_auth_url()
}

#[tauri::command]
pub async fn check_auth_status_command(
    app_handle: tauri::AppHandle, // Tambahkan parameter ini untuk akses Disk!
    state: tauri::State<'_, AppState>
) -> Result<bool, String> {
    
    // 1. Coba ambil dari RAM dulu
    let (mut access_opt, mut refresh_opt) = {
        let at = state.access_token.lock().unwrap().clone();
        let rt = state.refresh_token.lock().unwrap().clone();
        (at, rt)
    };

    // 2. Kalo RAM kosong (karena aplikasi baru dibuka), BONGKAR DISK!
    if access_opt.is_none() || refresh_opt.is_none() {
        if let Ok(store) = app_handle.store("session.json") {
            // Coba baca token dari session.json
            if let Some(at_val) = store.get("access_token") {
                if let Some(at_str) = at_val.as_str() {
                    access_opt = Some(at_str.to_string());
                    *state.access_token.lock().unwrap() = Some(at_str.to_string()); // Pindahkan ke RAM
                }
            }
            if let Some(rt_val) = store.get("refresh_token") {
                if let Some(rt_str) = rt_val.as_str() {
                    refresh_opt = Some(rt_str.to_string());
                    *state.refresh_token.lock().unwrap() = Some(rt_str.to_string()); // Pindahkan ke RAM
                }
            }
        }
    }

    // 3. Kalau di RAM dan di Disk tetap kosong = Beneran belum login
    if access_opt.is_none() || refresh_opt.is_none() {
        return Ok(false);
    }

    let access_token = access_opt.unwrap();
    let refresh_token = refresh_opt.unwrap();

    // 4. Ping Supabase
    let is_valid = crate::services::auth_service::validate_token(&access_token).await;
    // dont forget to remove!!!!!!!!!!!!!!!
    println!("\n=====================================================");
    println!("🔑 [DEBUG] ACCESS TOKEN BARU:");
    println!("{}\n", access_token);
    println!("🔄 [DEBUG] REFRESH TOKEN BARU:");
    println!("{}", refresh_token);
    println!("=====================================================\n");
    
    if is_valid {
        println!("✅ [RUST] Token divalidasi dan masih hidup!");
        return Ok(true); 
    }

    println!("🔄 [RUST] Token Expired! Sedang mencoba Auto-Refresh...");
    
    match crate::services::auth_service::refresh_access_token(&refresh_token).await {
        Ok((new_access, new_refresh)) => {
            // Tukar berhasil! Update RAM
            *state.access_token.lock().unwrap() = Some(new_access.clone());
            *state.refresh_token.lock().unwrap() = Some(new_refresh.clone());
            
            // UPDATE DISK JUGA!
            if let Ok(store) = app_handle.store("session.json") {
                store.set("access_token", json!(new_access));
                store.set("refresh_token", json!(new_refresh));
                let _ = store.save();
            }

            println!("✅ [RUST] Auto-Refresh Sukses! Brankas RAM & Disk diperbarui.");
            Ok(true) 
        }
        Err(err) => {
            println!("❌ [RUST] Auto-Refresh Gagal: {}. Menghapus data...", err);
            
            // Hapus RAM
            *state.access_token.lock().unwrap() = None;
            *state.refresh_token.lock().unwrap() = None;
            
            // HAPUS DISK (Biar besok gak error lagi)
            if let Ok(store) = app_handle.store("session.json") {
                let _ = store.delete("access_token");
                let _ = store.delete("refresh_token");
                let _ = store.save();
            }
            
            Ok(false) 
        }
    }
}

pub fn init_deep_link_listener(app_handle: AppHandle) {
    let handle_clone = app_handle.clone();

    app_handle.deep_link().on_open_url(move |event| {
        for url_obj in event.urls() {
            let url = url_obj.to_string();
            println!("🔥 [RUST] DEEPLINK MASUK: {}", url);

            if url.starts_with("com.users.scantrash://auth") {
                let hash_part = url.split('#').nth(1).unwrap_or("");
                let mut access_token = String::new();
                let mut refresh_token = String::new();

                for pair in hash_part.split('&') {
                    let mut kv = pair.split('=');
                    if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                        if k == "access_token" { access_token = v.to_string(); }
                        if k == "refresh_token" { refresh_token = v.to_string(); }
                    }
                }

                if !access_token.is_empty() {
                    // KITA CLONE DATA UNTUK DIBAWA KE BACKGROUND TASK
                    
                    // dont forget to remove!!!!!!!!!!!!!!!
                    println!("\n=====================================================");
                    println!("🔑 [DEBUG] ACCESS TOKEN BARU:");
                    println!("{}\n", access_token);
                    println!("🔄 [DEBUG] REFRESH TOKEN BARU:");
                    println!("{}", refresh_token);
                    println!("=====================================================\n");

                    let bg_handle = handle_clone.clone();
                    let bg_access = access_token.clone();
                    let bg_refresh = refresh_token.clone();

                    // LEMPAR TUGAS BERAT KE BACKGROUND THREAD AGAR ANDROID TIDAK MARAH!
                    tauri::async_runtime::spawn(async move {
                        // 1. Simpan ke RAM
                        let state = bg_handle.state::<AppState>();
                        *state.access_token.lock().unwrap() = Some(bg_access.clone());
                        *state.refresh_token.lock().unwrap() = Some(bg_refresh.clone());    

                        // 2. Simpan ke Disk secara aman di luar Main Thread
                        if let Ok(store) = bg_handle.store("session.json") {
                            store.set("access_token", json!(bg_access));
                            store.set("refresh_token", json!(bg_refresh));
                            let _ = store.save(); 
                            println!("💾 [RUST] Token berhasil diamankan ke Persistent Storage!");
                        }

                        // 3. Kirim sinyal ke Vue
                        if let Err(e) = bg_handle.emit("login-success", "Login tervalidasi di backend!") {
                            println!("⚠️ [RUST] UI belum siap menerima sinyal... Error: {}", e);
                        }
                    });
                }
            }
        }
    });
}

#[tauri::command]
pub async fn logout_command(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    
    // 1. Ambil token lama dari RAM sebelum dihapus (untuk lapor ke server)
    let access_token = state.access_token.lock().unwrap().clone();

    // 2. Kosongkan Brankas RAM
    *state.access_token.lock().unwrap() = None;
    *state.refresh_token.lock().unwrap() = None;

    // 3. Sapu Bersih Disk (Persistent Storage)
    if let Ok(store) = app_handle.store("session.json") {
        let _ = store.delete("access_token");
        let _ = store.delete("refresh_token");
        let _ = store.save(); // Wajib di-save agar benar-benar terhapus dari memori HP
        println!("🗑️ [RUST] Token berhasil dihapus dari Disk!");
    }

    // 4. Lapor ke Supabase untuk menghanguskan token di server
    if let Some(token) = access_token {
        crate::services::auth_service::logout_from_server(&token).await;
        println!("🔌 [RUST] Sesi dimatikan dari server Supabase.");
    }

    Ok(())
}