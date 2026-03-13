use crate::models::AppState;
use crate::services::log_service;

#[tauri::command]
pub async fn create_log_command(level: String, message: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    
    let access_token = {
        let guard = state.access_token.lock().unwrap();
        match &*guard {
            Some(t) => t.clone(),
            None => return Err("Akses ditolak: User belum login!".to_string()),
        }
    };

    // LANGSUNG LEMPAR KE SERVICE! Pendek banget kan?
    match log_service::insert_log_to_supabase(&level, &message, &access_token).await {
        Ok(_) => Ok("Log berhasil dicatat!".to_string()),
        Err(e) => Err(e),
    }
}