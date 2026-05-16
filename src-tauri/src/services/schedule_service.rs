use crate::config::AppConfig;
use crate::models::Schedule;
use crate::services::local_log_service::write_local_log;
use reqwest::Client;

pub async fn fetch_all_schedules(
    app_handle: &tauri::AppHandle,
    access_token: &str,
) -> Result<Vec<Schedule>, String> {
    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    // Mengambil data dari tabel `tanggal` (diurutkan berdasarkan tanggal desc = terbaru dulu)
    let url = format!("{}/rest/v1/tanggal?select=*&order=tanggal.asc", config.supabase_url);

    let res = client
        .get(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim request ke server untuk mengambil jadwal.");
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if res.status().is_success() {
        let data = res
            .json::<Vec<Schedule>>()
            .await
            .map_err(|_e| {
                write_local_log(app_handle, "ERROR", "Gagal parsing JSON respons jadwal dari server.");
                "Terjadi kesalahan saat membaca data jadwal.".to_string()
            })?;
        Ok(data)
    } else {
        write_local_log(app_handle, "ERROR", "Server menolak request jadwal (HTTP error).");
        Err("Gagal mengambil data jadwal dari server.".to_string())
    }
}

/// Menambahkan jadwal baru ke tabel tanggal.
///
/// Parameter `waktu_buka` dan `waktu_tutup` diterima dalam format "HH:MM" dari frontend,
/// lalu dikonversi ke "HH:MM:00+08" (WITA = UTC+8) sebelum dikirim ke Supabase.
pub async fn create_schedule(
    app_handle: &tauri::AppHandle,
    access_token: &str,
    tanggal: &str,
    waktu_buka: &str,
    waktu_tutup: &str,
    lokasi: &str,
) -> Result<(), String> {
    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    let url = format!("{}/rest/v1/tanggal", config.supabase_url);

    // Konversi "HH:MM" → "HH:MM:00+08" untuk format PostgreSQL time with time zone (WITA)
    let buka_fmt = format!("{}:00+08", waktu_buka);
    let tutup_fmt = format!("{}:00+08", waktu_tutup);

    let body = serde_json::json!({
        "tanggal":     tanggal,
        "waktu_buka":  buka_fmt,
        "waktu_tutup": tutup_fmt,
        "lokasi":      lokasi,
    });

    let resp = client
        .post(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal")
        .json(&body)
        .send()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim request tambah jadwal ke server.");
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if !resp.status().is_success() {
        write_local_log(
            app_handle,
            "ERROR",
            &format!("Server menolak request tambah jadwal (HTTP {}).", resp.status().as_u16()),
        );
        return Err("Gagal menyimpan jadwal ke server.".to_string());
    }

    write_local_log(app_handle, "INFO", "Jadwal berhasil ditambahkan.");
    Ok(())
}

/// Memperbarui jadwal yang sudah ada berdasarkan `id_tanggal` (UUID).
pub async fn update_schedule(
    app_handle: &tauri::AppHandle,
    access_token: &str,
    id_tanggal: &str,
    tanggal: &str,
    waktu_buka: &str,
    waktu_tutup: &str,
    lokasi: &str,
) -> Result<(), String> {
    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    let url = format!(
        "{}/rest/v1/tanggal?id_tanggal=eq.{}",
        config.supabase_url, id_tanggal
    );

    let buka_fmt = format!("{}:00+08", waktu_buka);
    let tutup_fmt = format!("{}:00+08", waktu_tutup);

    let body = serde_json::json!({
        "tanggal":     tanggal,
        "waktu_buka":  buka_fmt,
        "waktu_tutup": tutup_fmt,
        "lokasi":      lokasi,
    });

    let resp = client
        .patch(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal")
        .json(&body)
        .send()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim request edit jadwal ke server.");
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if !resp.status().is_success() {
        write_local_log(
            app_handle,
            "ERROR",
            &format!("Server menolak request edit jadwal (HTTP {}).", resp.status().as_u16()),
        );
        return Err("Gagal memperbarui jadwal di server.".to_string());
    }

    write_local_log(app_handle, "INFO", "Jadwal berhasil diperbarui.");
    Ok(())
}

/// Menghapus jadwal berdasarkan `id_tanggal` (UUID).
pub async fn delete_schedule(
    app_handle: &tauri::AppHandle,
    access_token: &str,
    id_tanggal: &str,
) -> Result<(), String> {
    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    let url = format!(
        "{}/rest/v1/tanggal?id_tanggal=eq.{}",
        config.supabase_url, id_tanggal
    );

    let resp = client
        .delete(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Prefer", "return=minimal")
        .send()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim request hapus jadwal ke server.");
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if !resp.status().is_success() {
        write_local_log(
            app_handle,
            "ERROR",
            &format!("Server menolak request hapus jadwal (HTTP {}).", resp.status().as_u16()),
        );
        return Err("Gagal menghapus jadwal dari server.".to_string());
    }

    write_local_log(app_handle, "INFO", "Jadwal berhasil dihapus.");
    Ok(())
}
