use tauri::AppHandle;
use crate::services::local_log_service;

#[tauri::command]
pub fn write_local_log_command(app_handle: AppHandle, level: String, message: String) {
    // Frontend mengirim waktu dan pesan, langsung lempar ke Service
    local_log_service::write_local_log(&app_handle, &level, &message);
}

#[tauri::command]
pub fn read_local_log_command(app_handle: tauri::AppHandle) -> Result<String, String> {
    // Langsung panggil service baca yang baru kita buat
    crate::services::local_log_service::read_local_log(&app_handle)
}