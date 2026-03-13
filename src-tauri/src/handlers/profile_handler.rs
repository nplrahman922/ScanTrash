use tauri::AppHandle;
use crate::models::Profile;
use crate::services::{profile_service, session_service};

#[tauri::command]
pub async fn get_profile_command(app_handle: AppHandle) -> Result<Profile, String> {
    // 1. Ambil token dari "Gudang" (session_service)
    let (access_opt, _) = session_service::get_session(&app_handle);
    
    // 2. Kalau token ada, cari profilnya. Kalau kosong, tolak.
    match access_opt {
        Some(token) => profile_service::get_user_profile(&token).await,
        None => Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
    }
}