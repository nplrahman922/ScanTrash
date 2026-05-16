use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// 1. Tabel log_system
#[derive(Debug, Serialize, Deserialize)]
pub struct LogSystem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>, 
    pub user_id: String,
    pub level: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

// 2. Tabel profiles
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub user_id: String,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

// 3. Tabel scan
#[derive(Debug, Serialize, Deserialize)]
pub struct Scan {
    pub user_id: String, // (Lihat catatan penting di bawah soal ini!)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_predict: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

// 4. Tabel pricelist
#[derive(Debug, Serialize, Deserialize)]
pub struct Pricelist {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub labels: String,
    pub price: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

// 5. Tabel savings (Tabungan / Riwayat Transaksi)
#[derive(Debug, Serialize, Deserialize)]
pub struct Savings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub user_id: String,
    pub amount_before: i64,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

// 6. Record transaksi savings (dengan kolom keterangan)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SavingsRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Saldo SEBELUM transaksi ini dilakukan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_before: Option<i64>,
    /// Saldo SETELAH transaksi (= saldo terkini nasabah jika ini baris terbaru)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Catatan / keterangan transaksi
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keterangan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

// 7. TransactionItem — respons riwayat transaksi yang sudah dikalkulasi di backend
/// Struct ini BUKAN representasi langsung tabel DB.
/// Dikirim ke frontend setelah backend menghitung nominal dan menentukan tipe transaksi.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionItem {
    pub id: String,
    /// Label transaksi: isi kolom keterangan, atau "Setoran"/"Penarikan" sebagai fallback
    pub name: String,
    /// Timestamp ISO dari created_at (format presentasi dilakukan di frontend)
    pub date: String,
    /// Selisih absolut antara amount dan amount_before
    pub nominal: i64,
    /// "income" jika saldo naik, "expense" jika saldo turun
    pub transaction_type: String,
}

// 8. Tabel tanggal (Jadwal Setor Nasabah)
#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tanggal: Option<String>,
    pub waktu_buka: String,
    pub lokasi: String,
    pub waktu_tutup: String,
    pub tanggal: String,
}

// Brankas RAM Aplikasi
pub struct AppState {
    pub access_token: Mutex<Option<String>>,
    pub refresh_token: Mutex<Option<String>>,
}