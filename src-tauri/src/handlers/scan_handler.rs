use tauri::{AppHandle, State};
use crate::models::AppState;
use crate::services::scan_service::{process_scan_image, ScanResult};

#[tauri::command]
pub async fn scan_trash(
    _app_handle: AppHandle, 
    state: State<'_, AppState>,
    image: String
) -> Result<Vec<ScanResult>, String> { 
    
    // 1. Validasi Ukuran (Mencegah DoS / Payload terlalu besar sebelum diproses)
    if image.len() > 7_000_000 {
        return Err("Payload gambar terlalu besar. Maksimal ~5MB.".into());
    }

    // 2. Ambil User Token
    let user_token = {
        let lock = state.access_token.lock().map_err(|_| "Gagal membuka brankas token")?;
        lock.clone().ok_or("Sesi habis atau user belum login!")?
    };

    // 3. Panggil Service untuk Memproses
    let final_results = process_scan_image(&_app_handle, &user_token, image).await?;

    Ok(final_results)
}