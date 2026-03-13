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
    pub created_at: Option<String>,
}

// 5. Tabel savings (Tabungan)
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

// Brankas RAM Aplikasi
pub struct AppState {
    pub access_token: Mutex<Option<String>>,
    pub refresh_token: Mutex<Option<String>>,
}