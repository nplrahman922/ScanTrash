use tauri::{AppHandle, Manager};
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;

pub fn write_local_log(app_handle: &AppHandle, level: &str, message: &str) {
    // 1. Dapatkan jalur aman khusus aplikasi kita (App Local Data Directory)
    if let Ok(mut path) = app_handle.path().app_local_data_dir() {
        
        // 2. Pastikan foldernya ada (kalau belum ada, buat foldernya)
        if !path.exists() {
            let _ = create_dir_all(&path);
        }

        // 3. Tentukan nama file log-nya
        path.push("scantrash_local.log");

        // 4. Buka file dengan mode "Append" (tambah ke baris bawah, jangan timpa yang lama)
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            // Tulis pesan ke dalam file
            let log_entry = format!("[{}] {}\n", level, message);
            let _ = file.write_all(log_entry.as_bytes());
            
            // Print ke terminal (hanya untuk memudahkanmu saat development)
            println!("💾 [LOCAL LOG] {}", log_entry.trim());
        } else {
            println!("❌ [LOCAL LOG] Gagal membuka file log di HP!");
        }
    }
}