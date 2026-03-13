use reqwest::Client;
use crate::config::AppConfig;
use serde_json::Value;

// 1. Service Generator URL Google Auth
pub fn generate_google_auth_url() -> String {
    let config = crate::config::AppConfig::init();

    // Redirect URL harus sesuai scheme deep-link yang didaftarkan.
    // Pastikan di Supabase settings redirect URL juga terdaftar: scantrash://oauth
    format!(
        "{}/auth/v1/authorize?provider=google&prompt=consent&redirect_to=scantrash://oauth",
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

pub fn parse_tokens_from_url(url: &str) -> Option<(String, String)> {
    println!("🔍 [RUST] Parsing URL: {}", url);

    // Sertakan dua format yang mungkin muncul saat redirect:
    // 1) scantrash://oauth?access_token=...&refresh_token=...
    // 2) scantrash://oauth#access_token=...&refresh_token=...
    if !url.starts_with("scantrash://oauth") {
        println!("❌ [RUST] URL tidak dimulai dengan scheme yang benar (scantrash://oauth)");
        return None;
    }

    // Ambil bagian yang berisi token, bisa setelah '?' atau '#'
    let token_part = url.split(['?', '#']).nth(1).unwrap_or("");
    if token_part.is_empty() {
        println!("❌ [RUST] Tidak menemukan token di URL (tidak ada query atau fragment)");
        return None;
    }

    let mut access_token = String::new();
    let mut refresh_token = String::new();

    for pair in token_part.split('&') {
        let mut kv = pair.split('=');
        if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
            if k == "access_token" {
                access_token = v.to_string();
                println!("✅ [RUST] Found access_token");
            }
            if k == "refresh_token" {
                refresh_token = v.to_string();
                println!("✅ [RUST] Found refresh_token");
            }
        }
    }

    if !access_token.is_empty() && !refresh_token.is_empty() {
        println!("✅ [RUST] Token parsing berhasil");
        Some((access_token, refresh_token))
    } else {
        println!("❌ [RUST] Token tidak lengkap - access: {}, refresh: {}", !access_token.is_empty(), !refresh_token.is_empty());
        None
    }
}