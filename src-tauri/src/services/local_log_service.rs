use tauri::Manager; // Manager tetap dibiarkan karena dibutuhkan untuk fungsi .path()
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::read_to_string;
use chrono::Local;

pub fn write_local_log(app_handle: &tauri::AppHandle, level: &str, message: &str) {
    if let Ok(mut path) = app_handle.path().app_local_data_dir() {
        path.push("scantrash_local.log");

        // --- PROTEKSI LOG GENDUT ---
        if path.exists() {
            if let Ok(metadata) = std::fs::metadata(&path) {
                // Jika ukuran file > 1MB (1.048.576 bytes)
                if metadata.len() > 1_000_000 {
                    // Hapus file lama agar tidak membebani HP
                    let _ = std::fs::remove_file(&path);
                }
            }
        }
        // ---------------------------

        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}] [{}] {}\n", now, level.trim(), message);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("Gagal membuka file log");

        let _ = file.write_all(log_entry.as_bytes());
    }
}

pub fn read_local_log(app_handle: &tauri::AppHandle) -> Result<String, String> {
    // 1. Dapatkan jalur folder lokal aplikasi
    if let Ok(mut path) = app_handle.path().app_local_data_dir() {
        path.push("scantrash_local.log");

        // 2. Cek apakah filenya benar-benar ada
        if !path.exists() {
            return Ok("Belum ada data log tersimpan.".to_string());
        }

        // 3. Baca file dan kirim hasilnya
        match read_to_string(&path) {
            Ok(content) => Ok(content),
            Err(e) => Err(format!("Gagal membaca log: {}", e)),
        }
    } else {
        Err("Gagal mengakses sistem folder Android.".to_string())
    }
}