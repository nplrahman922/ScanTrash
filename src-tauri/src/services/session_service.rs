use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;
use serde_json::json;
use crate::models::AppState;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn get_session(app_handle: &AppHandle) -> (Option<String>, Option<String>) {
    let state = app_handle.state::<AppState>();
    // AMBIL KUNCI DARI .ENV SAAT KOMPILASI
    let mc = new_magic_crypt!(dotenvy_macro::dotenv!("SESSION_SECRET_KEY"), 256);
    
    let mut access_opt = state.access_token.lock().unwrap().clone();
    let mut refresh_opt = state.refresh_token.lock().unwrap().clone();

    if access_opt.is_none() || refresh_opt.is_none() {
        if let Ok(store) = app_handle.store("session.json") {
            // Ambil Access Token yang terenkripsi
            if let Some(at_val) = store.get("access_token") {
                if let Some(at_encrypted) = at_val.as_str() {
                    // DEKRIPSI DULU SEBELUM DIMASUKKAN KE RAM
                    if let Ok(at_decrypted) = mc.decrypt_base64_to_string(at_encrypted) {
                        access_opt = Some(at_decrypted.clone());
                        *state.access_token.lock().unwrap() = Some(at_decrypted);
                    }
                }
            }
            // Ambil Refresh Token yang terenkripsi
            if let Some(rt_val) = store.get("refresh_token") {
                if let Some(rt_encrypted) = rt_val.as_str() {
                    // DEKRIPSI DULU
                    if let Ok(rt_decrypted) = mc.decrypt_base64_to_string(rt_encrypted) {
                        refresh_opt = Some(rt_decrypted.clone());
                        *state.refresh_token.lock().unwrap() = Some(rt_decrypted);
                    }
                }
            }
        }
    }
    (access_opt, refresh_opt)
}

pub fn save_session(app_handle: &AppHandle, access_token: &str, refresh_token: &str) {
    let state = app_handle.state::<AppState>();
    let mc = new_magic_crypt!(dotenvy_macro::dotenv!("SESSION_SECRET_KEY"), 256);
    
    // Simpan token ASLI di RAM (karena RAM tidak bisa disedot dengan mudah)
    *state.access_token.lock().unwrap() = Some(access_token.to_string());
    *state.refresh_token.lock().unwrap() = Some(refresh_token.to_string());

    // ENKRIPSI sebelum dilempar ke Harddisk / session.json
    let encrypted_access = mc.encrypt_str_to_base64(access_token);
    let encrypted_refresh = mc.encrypt_str_to_base64(refresh_token);

    if let Ok(store) = app_handle.store("session.json") {
        store.set("access_token", json!(encrypted_access));
        store.set("refresh_token", json!(encrypted_refresh));
        let _ = store.save();
    }
}

// 3. Fungsi Menghapus Token (Kosongkan RAM dan Disk)
pub fn clear_session(app_handle: &AppHandle) {
    let state = app_handle.state::<AppState>();
    
    // Hapus RAM
    *state.access_token.lock().unwrap() = None;
    *state.refresh_token.lock().unwrap() = None;

    // Hapus Disk
    if let Ok(store) = app_handle.store("session.json") {
        let _ = store.delete("access_token");
        let _ = store.delete("refresh_token");
        let _ = store.save();
    }
}