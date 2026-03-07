use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogSystem {
    pub id: Option<String>, // Option karena kalau insert baru, id otomatis di-generate DB
    pub user_id: String,    // Sesuai dengan user_id uuid di tabel log_system 
    pub level: String,      // Sesuai dengan level TEXT 
    pub message: String,    // Sesuai dengan message TEXT 
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pricelist {
    pub id: String,
    pub labels: String,
    pub price: i64, 
    pub created_at: String,
}

// Kamu bisa tambahkan struct untuk Profiles, Scan, Savings, dll di bawahnya nanti.