use crate::models::LogSystem;
use crate::services::log_service;

#[tauri::command]
pub async fn create_log_command(
    user_id: String,
    level: String,
    message: String,
    token: String,
) -> Result<String, String> {
    
    // Siapkan struct sesuai format models.rs
    let new_log = LogSystem {
        id: None, // Kosongkan agar UUID di-generate otomatis oleh DB default gen_random_uuid() [cite: 7]
        user_id,
        level,
        message,
        created_at: None, // Kosongkan agar pakai default now() di DB [cite: 7]
    };

    // Panggil service yang tadi kita buat
    match log_service::insert_log_to_supabase(new_log, &token).await {
        Ok(_) => Ok("Log berhasil dicatat!".to_string()),
        Err(e) => Err(e),
    }
}