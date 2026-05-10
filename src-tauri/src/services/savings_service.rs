use reqwest::Client;
use crate::config::AppConfig;
use crate::models::SavingsRecord;
use crate::services::local_log_service::write_local_log;

/// Mengambil saldo terkini nasabah dari tabel savings.
///
/// Saldo terkini = nilai kolom `amount` dari baris paling baru
/// (amount = saldo SETELAH transaksi terakhir dilakukan).
///
/// Return: saldo terkini dalam bentuk i64, atau 0 jika belum ada transaksi.
pub async fn get_current_balance(
    app_handle: &tauri::AppHandle,
    access_token: &str,
    target_user_id: Option<&str>,
) -> Result<i64, String> {
    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    // Ambil 1 baris terbaru, cukup kolom amount saja
    let mut url = format!(
        "{}/rest/v1/savings?select=amount&order=created_at.desc&limit=1",
        config.supabase_url
    );

    // Jika Admin mengirimkan ID spesifik, filter berdasarkan user_id tersebut
    if let Some(uid) = target_user_id {
        url = format!("{}&user_id=eq.{}", url, uid);
    }

    let resp = client
        .get(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim request ke server untuk mengambil saldo.");
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if !resp.status().is_success() {
        write_local_log(app_handle, "ERROR", "Server menolak request saldo (HTTP error).");
        return Err("Gagal mengambil data saldo dari server.".to_string());
    }

    let json: serde_json::Value = resp.json().await.map_err(|_e| {
        write_local_log(app_handle, "ERROR", "Gagal parsing JSON respons saldo dari server.");
        "Terjadi kesalahan saat membaca data saldo.".to_string()
    })?;

    // Jika array kosong, nasabah belum punya transaksi → saldo 0
    let saldo = json
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|obj| obj.get("amount"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    write_local_log(app_handle, "INFO", &format!("Saldo terkini berhasil diambil: Rp {}", saldo));
    Ok(saldo)
}

/// Mengambil riwayat transaksi nasabah dari tabel savings.
///
/// Mengembalikan maksimal 50 transaksi terbaru, diurutkan dari yang paling baru.
/// Setiap baris berisi: id, user_id, amount_before, amount, keterangan, created_at.
pub async fn get_savings_history(
    app_handle: &tauri::AppHandle,
    access_token: &str,
    target_user_id: Option<&str>,
) -> Result<Vec<SavingsRecord>, String> {
    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    let mut url = format!(
        "{}/rest/v1/savings?select=*&order=created_at.desc&limit=50",
        config.supabase_url
    );

    // Jika Admin mengirimkan ID spesifik, filter berdasarkan user_id tersebut
    if let Some(uid) = target_user_id {
        url = format!("{}&user_id=eq.{}", url, uid);
    }

    let resp = client
        .get(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim request ke server untuk mengambil riwayat transaksi.");
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if !resp.status().is_success() {
        write_local_log(app_handle, "ERROR", "Server menolak request riwayat transaksi (HTTP error).");
        return Err("Gagal mengambil riwayat transaksi dari server.".to_string());
    }

    let records: Vec<SavingsRecord> = resp.json().await.map_err(|_e| {
        write_local_log(app_handle, "ERROR", "Gagal parsing JSON respons riwayat transaksi dari server.");
        "Terjadi kesalahan saat membaca riwayat transaksi.".to_string()
    })?;

    write_local_log(
        app_handle,
        "INFO",
        &format!("Berhasil mengambil {} riwayat transaksi.", records.len()),
    );
    Ok(records)
}
