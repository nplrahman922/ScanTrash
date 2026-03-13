use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;
use serde_json::json;
use crate::models::AppState;

// 1. Fungsi Mengambil Token (Cek RAM, lalu cek Disk)
pub fn get_session(app_handle: &AppHandle) -> (Option<String>, Option<String>) {
    let state = app_handle.state::<AppState>();
    
    let mut access_opt = state.access_token.lock().unwrap().clone();
    let mut refresh_opt = state.refresh_token.lock().unwrap().clone();

    // Kalau RAM kosong, bongkar Disk
    if access_opt.is_none() || refresh_opt.is_none() {
        if let Ok(store) = app_handle.store("session.json") {
            if let Some(at_val) = store.get("access_token") {
                if let Some(at_str) = at_val.as_str() {
                    access_opt = Some(at_str.to_string());
                    *state.access_token.lock().unwrap() = Some(at_str.to_string());
                }
            }
            if let Some(rt_val) = store.get("refresh_token") {
                if let Some(rt_str) = rt_val.as_str() {
                    refresh_opt = Some(rt_str.to_string());
                    *state.refresh_token.lock().unwrap() = Some(rt_str.to_string());
                }
            }
        }
    }
    (access_opt, refresh_opt)
}

// 2. Fungsi Menyimpan Token (Ke RAM dan Disk)
pub fn save_session(app_handle: &AppHandle, access_token: &str, refresh_token: &str) {
    let state = app_handle.state::<AppState>();
    
    // Simpan RAM
    *state.access_token.lock().unwrap() = Some(access_token.to_string());
    *state.refresh_token.lock().unwrap() = Some(refresh_token.to_string());

    // Simpan Disk
    if let Ok(store) = app_handle.store("session.json") {
        store.set("access_token", json!(access_token));
        store.set("refresh_token", json!(refresh_token));
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