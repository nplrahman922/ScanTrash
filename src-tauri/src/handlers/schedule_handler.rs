use tauri::AppHandle;
use crate::models::Schedule;
use crate::services::{schedule_service, session_service, local_log_service};

#[tauri::command]
pub async fn get_schedules_command(app_handle: AppHandle) -> Result<Vec<Schedule>, String> {
    // 1. Cek sesi aktif
    let (access_opt, _) = session_service::get_session(&app_handle);

    match access_opt {
        Some(token) => {
            // 2. Panggil service untuk mengambil data jadwal
            match schedule_service::fetch_all_schedules(&app_handle, &token).await {
                Ok(data) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!("Berhasil mengambil {} data jadwal setor.", data.len()),
                    );
                    Ok(data)
                }
                Err(_e) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "ERROR",
                        "Gagal mengambil data jadwal setor dari server.",
                    );
                    Err("Gagal memuat jadwal. Silakan coba lagi.".to_string())
                }
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle,
                "WARNING",
                "Akses jadwal setor ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}
