use tauri::AppHandle;
use crate::models::Profile;
use crate::services::{profile_service, session_service, local_log_service}; 

#[tauri::command]
pub async fn get_profile_command(app_handle: AppHandle) -> Result<Profile, String> {
    // 1. Ambil token dari "Gudang" (session_service)
    let (access_opt, _) = session_service::get_session(&app_handle);
    
    // 2. Evaluasi token dan tangkap hasil dari Service untuk dicatat di Log
    match access_opt {
        Some(token) => {
            // Tunggu hasil dari profil_service
            match profile_service::get_user_profile(&token).await {
                Ok(profile) => {
                    // [SUKSES] Catat log, lalu kembalikan profil ke Frontend
                    local_log_service::write_local_log(&app_handle, "INFO", &format!("Berhasil menarik data profil untuk user: {}", profile.username));
                    Ok(profile)
                },
                Err(e) => {
                    // [ERROR] Catat log error dari Supabase, lalu lempar pesan error ke Frontend
                    local_log_service::write_local_log(&app_handle, "ERROR", &format!("Gagal menarik profil: {}", e));
                    Err(e)
                }
            }
        },
        None => {
            // [DITOLAK] Token kosong, catat log peringatan
            local_log_service::write_local_log(&app_handle, "WARNING", "Akses profil ditolak: Tidak ada sesi aktif di penyimpanan lokal.");
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}