use reqwest::Client;
use crate::config::AppConfig;
use crate::models::{SavingsRecord, TransactionItem};
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
    target_user_id: &str,
) -> Result<i64, String> {
    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    // Ambil 1 baris terbaru, cukup kolom amount saja
    let url = format!(
        "{}/rest/v1/savings?select=amount&order=created_at.desc&limit=1&user_id=eq.{}",
        config.supabase_url, target_user_id
    );

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

    write_local_log(app_handle, "INFO", "Berhasil mengambil saldo terkini.");
    Ok(saldo)
}

/// Mengambil riwayat transaksi nasabah dari tabel savings,
/// lalu menghitung nominal dan tipe transaksi di sisi backend.
///
/// Mengembalikan maksimal 50 transaksi terbaru sebagai Vec<TransactionItem>,
/// siap dikonsumsi frontend tanpa perlu kalkulasi tambahan.
pub async fn get_savings_history(
    app_handle: &tauri::AppHandle,
    access_token: &str,
    target_user_id: &str,
) -> Result<Vec<TransactionItem>, String> {
    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    let url = format!(
        "{}/rest/v1/savings?select=*&order=created_at.desc&limit=50&user_id=eq.{}",
        config.supabase_url, target_user_id
    );

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

    // Kalkulasi dilakukan di backend: nominal selisih dan tipe transaksi (income/expense)
    let items: Vec<TransactionItem> = records.into_iter().enumerate().map(|(i, rec)| {
        let before  = rec.amount_before.unwrap_or(0);
        let after   = rec.amount.unwrap_or(0);
        let is_income = after >= before;
        let nominal   = (after - before).unsigned_abs() as i64;

        let name = rec.keterangan
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| if is_income { "Setoran".to_string() } else { "Penarikan".to_string() });

        TransactionItem {
            id: rec.id.unwrap_or_else(|| i.to_string()),
            name,
            date: rec.created_at.unwrap_or_default(),
            nominal,
            transaction_type: if is_income { "income".to_string() } else { "expense".to_string() },
        }
    }).collect();

    write_local_log(
        app_handle,
        "INFO",
        &format!("Berhasil mengambil dan menghitung {} riwayat transaksi.", items.len()),
    );
    Ok(items)
}
