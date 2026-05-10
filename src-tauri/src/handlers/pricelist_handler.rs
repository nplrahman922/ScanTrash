use tauri::AppHandle;
use crate::models::Pricelist;
use crate::services::{pricelist_service, session_service, local_log_service};

#[tauri::command]
pub async fn get_pricelist_command(app_handle: AppHandle) -> Result<Vec<Pricelist>, String> {
    // ✅ FIX Bug 3: Cek session dulu — konsisten dengan semua handler lain
    let (access_opt, _) = session_service::get_session(&app_handle);

    match access_opt {
        Some(token) => {
            match pricelist_service::fetch_all_pricelist(&app_handle, &token).await {
                Ok(data) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!("Berhasil mengambil {} item pricelist.", data.len()),
                    );
                    Ok(data)
                }
                Err(_e) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "ERROR",
                        "Gagal mengambil data pricelist dari server.",
                    );
                    Err("Gagal memuat daftar harga. Silakan coba lagi.".to_string())
                }
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle,
                "WARNING",
                "Akses pricelist ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}
