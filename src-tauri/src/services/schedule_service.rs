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

    // Mengambil data dari tabel `tanggal` (diurutkan berdasarkan tanggal)
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
