use reqwest::Client;
use crate::config::AppConfig;
use serde_json::Value;

// 1. Service Generator URL Google Auth
pub fn generate_google_auth_url() -> String {
    let config = crate::config::AppConfig::init();
    
    // Tanpa /callback
    format!(
        "{}/auth/v1/authorize?provider=google&prompt=consent&redirect_to=com.users.scantrash://auth", 
        config.supabase_url
    )
}

// 2. Fungsi untuk "Ping" (Cek apakah access_token masih hidup)
pub async fn validate_token(access_token: &str) -> bool {
    let config = AppConfig::init();
    let client = Client::new();
    let url = format!("{}/auth/v1/user", config.supabase_url);

    // Kita tembak endpoint /user bawaan Supabase
    let res = client.get(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await;

    match res {
        Ok(response) => response.status().is_success(), // True jika 200 OK
        Err(_) => false,
    }
}

// 3. Fungsi untuk menukar refresh_token dengan access_token baru
pub async fn refresh_access_token(refresh_token: &str) -> Result<(String, String), String> {
    let config = AppConfig::init();
    let client = Client::new();
    let url = format!("{}/auth/v1/token?grant_type=refresh_token", config.supabase_url);

    let res = client.post(&url)
        .header("apikey", &config.supabase_key)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({ "refresh_token": refresh_token }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let body: Value = res.json().await.map_err(|e| e.to_string())?;
        
        // Ambil token baru dari response JSON Supabase
        let new_access = body["access_token"].as_str().unwrap_or("").to_string();
        let new_refresh = body["refresh_token"].as_str().unwrap_or("").to_string();

        if !new_access.is_empty() && !new_refresh.is_empty() {
            Ok((new_access, new_refresh))
        } else {
            Err("Format token tidak valid dari Supabase".to_string())
        }
    } else {
        Err("Refresh token ditolak (Mungkin sudah basi/terpakai)".to_string())
    }
}

// Fungsi untuk mematikan sesi di server Supabase
pub async fn logout_from_server(access_token: &str) {
    let config = AppConfig::init();
    let client = Client::new();
    let url = format!("{}/auth/v1/logout", config.supabase_url);

    // Kita kirim request POST kosong, yang penting ada Header Authorization-nya
    let _ = client.post(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await;
    
    // Kita abaikan responsenya (entah sukses atau gagal di server, 
    // yang penting aplikasi kita tetap membuang token di HP)
}

pub async fn get_user_id(access_token: &str) -> Result<String, String> {
    let config = crate::config::AppConfig::init();
    let client = reqwest::Client::new();
    let url = format!("{}/auth/v1/user", config.supabase_url);

    let res = client.get(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|e| format!("Gagal menghubungi Supabase: {}", e))?;

    if res.status().is_success() {
        let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        
        // Ambil ID dari response JSON
        if let Some(id) = body["id"].as_str() {
            return Ok(id.to_string());
        }
    }
    
    Err("Sesi tidak valid atau User ID tidak ditemukan".to_string())
}