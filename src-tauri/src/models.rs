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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pricelist {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub labels: String,
    pub price: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub img_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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

// 8b. NasabahItem — respons daftar nasabah untuk fitur admin
/// Berisi data profil nasabah (role=user) digabung dengan saldo terkini.
/// Dikirim ke frontend setelah backend mengambil profil + kalkulasi saldo.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NasabahItem {
    pub user_id: String,
    pub username: String,
    /// Email digunakan sebagai pengganti nomor HP (tidak ada kolom phone di tabel profiles)
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Saldo terkini nasabah (Rupiah), 0 jika belum ada transaksi
    pub saldo: i64,
}

// 9. AktivitasItem — satu entri aktivitas transaksi lintas nasabah (untuk admin dashboard)
/// Mirip dengan TransactionItem tapi dilengkapi username nasabah pelaku transaksi.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AktivitasItem {
    pub id: String,
    /// Nama nasabah pelaku transaksi (di-lookup dari HashMap profil, bukan query per baris)
    pub username: String,
    /// UUID nasabah — berguna untuk navigasi ke halaman detail nasabah
    pub user_id: String,
    /// Timestamp ISO dari created_at
    pub date: String,
    /// Keterangan transaksi atau label default ("Setoran" / "Penarikan")
    pub keterangan: String,
    /// Selisih absolut antara amount dan amount_before
    pub nominal: i64,
    /// "income" jika saldo naik, "expense" jika saldo turun
    pub transaction_type: String,
}

// 10. AdminDashboard — respons lengkap halaman dashboard admin
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminDashboard {
    /// Total saldo semua nasabah (Rupiah)
    pub total_saldo: i64,
    /// Jumlah nasabah (role = 'users')
    pub total_nasabah: i64,
    /// 10 transaksi terbaru dari seluruh nasabah, diurutkan dari yang paling baru
    pub aktivitas_terbaru: Vec<AktivitasItem>,
}

// 8. Tabel tanggal (Jadwal Setor Nasabah)
#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_tanggal: Option<String>,
    pub waktu_buka: String,
    pub lokasi: String,
    pub waktu_tutup: String,
    pub tanggal: String,
}

// 8b. Payload untuk tambah/edit jadwal (tanpa id, karena UUID auto-generated)
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleInput {
    pub tanggal: String,    // "YYYY-MM-DD"
    pub waktu_buka: String, // "HH:MM:00+08" (WITA)
    pub waktu_tutup: String,// "HH:MM:00+08" (WITA)
    pub lokasi: String,
}

// Brankas RAM Aplikasi
pub struct AppState {
    pub access_token: Mutex<Option<String>>,
    pub refresh_token: Mutex<Option<String>>,
}