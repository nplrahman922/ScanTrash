use crate::config::AppConfig;
use crate::models::Pricelist;
use crate::services::local_log_service::write_local_log;
use reqwest::Client;

pub async fn fetch_all_pricelist(
    app_handle: &tauri::AppHandle,
    access_token: &str,
) -> Result<Vec<Pricelist>, String> {
    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    let url = format!("{}/rest/v1/pricelist?select=*", config.supabase_url);

    let res = client
        .get(&url)
        .header("apikey", &config.supabase_key)
        // ✅ FIX Bug 2: Pakai user JWT, bukan anon key — konsisten dengan semua service lain
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|_e| {
            // ✅ FIX Bug 4: Log error internal, kirim pesan generik ke frontend
            write_local_log(app_handle, "ERROR", "Gagal mengirim request ke server untuk mengambil pricelist.");
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if res.status().is_success() {
        let data = res
            .json::<Vec<Pricelist>>()
            .await
            .map_err(|_e| {
                write_local_log(app_handle, "ERROR", "Gagal parsing JSON respons pricelist dari server.");
                "Terjadi kesalahan saat membaca data pricelist.".to_string()
            })?;
        Ok(data)
    } else {
        write_local_log(app_handle, "ERROR", "Server menolak request pricelist (HTTP error).");
        Err("Gagal mengambil data pricelist dari server.".to_string())
    }
}
