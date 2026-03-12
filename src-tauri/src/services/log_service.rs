use crate::models::LogSystem;
use crate::config::AppConfig; // <-- PANGGIL CONFIG SENTRAL KITA
use reqwest::Client;

pub async fn insert_log_to_supabase(log_data: LogSystem, jwt_token: &str) -> Result<(), String> {
    let client = Client::new();
    
    // Gunakan AppConfig, JANGAN pakai std::env untuk aplikasi mobile!
    let config = AppConfig::init();

    // Endpoint REST API bawaan Supabase untuk insert data
    let url = format!("{}/rest/v1/log_system", config.supabase_url);

    let res = client
        .post(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", jwt_token)) // Kirim token untuk ngelewatin RLS
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal") // Biar responnya cepat
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