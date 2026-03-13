use crate::models::LogSystem;
use crate::config::AppConfig;
use crate::services::auth_service; // <-- Wajib panggil ini untuk dapat user_id
use reqwest::Client;

// Perhatikan parameternya sekarang jauh lebih sederhana!
pub async fn insert_log_to_supabase(level: &str, message: &str, jwt_token: &str) -> Result<(), String> {
    
    // 1. Service otomatis mencari user_id dari token
    let user_id = auth_service::get_user_id(jwt_token).await?;

    // 2. Service merakit data otomatis (id dan created_at diisi None)
    let log_data = LogSystem {
        id: None,
        user_id,
        level: level.to_string(),
        message: message.to_string(),
        created_at: None,
    };

    let client = Client::new();
    let config = AppConfig::init();
    
    let url = format!("{}/rest/v1/log_system", config.supabase_url);

    let res = client
        .post(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", jwt_token)) 
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal") 
        .json(&log_data) // Data yang dirakit otomatis tadi dikirim ke sini
        .send()
        .await
        .map_err(|e| format!("Gagal mengirim request: {}", e))?;

    if res.status().is_success() {
        println!("✅ [RUST] Log berhasil dicatat di Supabase!");
        Ok(())
    } else {
        println!("❌ [RUST] Gagal mencatat log di Supabase!");
        let err_text = res.text().await.unwrap_or_default();
        Err(format!("Supabase error: {}", err_text))
    }
}