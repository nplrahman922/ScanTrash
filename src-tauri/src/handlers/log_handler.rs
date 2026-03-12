use crate::models::{LogSystem, AppState}; // Pastikan AppState di-import
use crate::services::{log_service, auth_service}; // Import auth_service juga

#[tauri::command]
pub async fn create_log_command(
    level: String,
    message: String,
    state: tauri::State<'_, AppState>, // Ambil brankas RAM langsung dari Tauri!
) -> Result<String, String> {
    
    // 1. BUKA BRANKAS RAM: Ambil token secara diam-diam
    let access_token = {
        let guard = state.access_token.lock().unwrap();
        match &*guard {
            Some(t) => t.clone(),
            None => return Err("Akses ditolak: User belum login!".to_string()),
        }
    };

    // 2. SOLUSI DARI TIMMU: Minta User ID ke Supabase menggunakan token tersebut
    let user_id = match auth_service::get_user_id(&access_token).await {
        Ok(id) => id,
        Err(e) => return Err(e),
    };

    // 3. Siapkan struct (UUID dan Waktu otomatis diisi oleh database)
    let new_log = LogSystem {
        id: None, 
        user_id,     // <-- ID yang valid dan 100% ada di tabel profiles!
        level,
        message,
        created_at: None, 
    };

    // 4. Panggil service untuk melakukan INSERT
    match log_service::insert_log_to_supabase(new_log, &access_token).await {
        Ok(_) => Ok("Log berhasil dicatat!".to_string()),
        Err(e) => Err(e),
    }
}