use crate::models::LogSystem;
use reqwest::Client;
use std::env;

pub async fn insert_log_to_supabase(log_data: LogSystem, jwt_token: &str) -> Result<(), String> {
    let client = Client::new();
    // Ambil variable dari environment (yang sudah di-load oleh config.rs sebelumnya)
    let supabase_url = env::var("SUPABASE_URL").map_err(|_| "SUPABASE_URL not found")?;
    let supabase_key = env::var("SUPABASE_KEY").map_err(|_| "SUPABASE_KEY not found")?;

    // Endpoint REST API bawaan Supabase untuk insert data
    let url = format!("{}/rest/v1/log_system", supabase_url);

    let res = client
        .post(&url)
        .header("apikey", supabase_key)
        .header("Authorization", format!("Bearer {}", jwt_token)) // Kirim token untuk ngelewatin RLS
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal") // Biar responnya cepat, nggak usah balikin data yang di-insert
        .json(&log_data)
        .send()
        .await
        .map_err(|e| format!("Gagal mengirim request: {}", e))?;

    if res.status().is_success() {
        Ok(())
    } else {
        let err_text = res.text().await.unwrap_or_default();
        Err(format!("Supabase error: {}", err_text))
    }
}
