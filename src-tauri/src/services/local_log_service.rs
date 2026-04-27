use tauri::{AppHandle, Manager};
use std::fs::{OpenOptions, create_dir_all, read_to_string};
use std::io::Write;
use chrono::Local;

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
            // Tulis pesan ke dalam file (dengan timestamp)
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
            let log_entry = format!("[{}] [{}] {}\n", timestamp, level, message);
            let _ = file.write_all(log_entry.as_bytes());
            
            // Print ke terminal (hanya untuk memudahkanmu saat development)
            println!("💾 [LOCAL LOG] {}", log_entry.trim());
        } else {
            println!("❌ [LOCAL LOG] Gagal membuka file log di HP!");
        }
    }
}

pub fn read_local_log(app_handle: &AppHandle) -> Result<String, String> {
    if let Ok(mut path) = app_handle.path().app_local_data_dir() {
        path.push("scantrash_local.log");
        if path.exists() {
            match read_to_string(&path) {
                Ok(content) => Ok(content),
                Err(e) => Err(format!("Gagal membaca file log: {}", e)),
            }
        } else {
            Ok(String::new())
        }
    } else {
        Err("Gagal mendapatkan direktori data aplikasi".to_string())
    }
}