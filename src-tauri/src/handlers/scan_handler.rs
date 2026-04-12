use serde::Serialize;
use tokio::time::{sleep, Duration};
use tauri::AppHandle;

#[derive(Serialize)]
pub struct ScanResult {
    status: String,
    trash_type: String,
    label_id: String,
    kelayakan: String,
    material_info: String,
    kondisi: String,
    kebersihan: String,
    estimasi_harga: i32,
}

#[tauri::command]
pub async fn analyze_trash_command(
    app_handle: AppHandle, 
    image_base64: String // ⬅️ INI TAMBAHAN BARUNYA: Rust siap menerima gambar
) -> Result<ScanResult, String> {
    
    // 1. Cek apakah Frontend benar-benar mengirim gambar
    if image_base64.is_empty() {
        print!("❌ Gambar kosong diterima di Rust.");
        crate::services::local_log_service::write_local_log(&app_handle, "WARNING", "Gagal menerima gambar.");
        return Err("Gagal: Gambar kosong atau tidak terbaca.".to_string());
        
    }

    // Catat log ukuran gambar (hanya untuk debugging)
    print!("✅ Menerima gambar dengan ukuran {} bytes", image_base64.len());
    crate::services::local_log_service::write_local_log(
        &app_handle, 
        "INFO", 
        &format!("Menerima gambar dari kamera. Ukuran data: {} bytes", image_base64.len())
    );

    // 2. SIMULASI AI
    // Nanti di sini gambar (image_base64) dikirim ke API OpenAI / Nopal
    sleep(Duration::from_secs(3)).await;

    // 3. Kembalikan Mock Data
    Ok(ScanResult {
        status: "success".to_string(),
        trash_type: "Botol Plastik PET".to_string(),
        label_id: "plastic_pet".to_string(),
        kelayakan: "Layak Ditabung".to_string(),
        material_info: "Plastik PET, ukuran standar.".to_string(),
        kondisi: "Utuh namun sudah diremas untuk menghemat ruang.".to_string(),
        kebersihan: "Sangat bersih, tidak ada kontaminasi.".to_string(),
        estimasi_harga: 4000,
    })
}