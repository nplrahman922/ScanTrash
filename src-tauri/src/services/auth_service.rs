use crate::config::AppConfig;
use crate::models::AuthResponse;
use reqwest::Client;
use serde_json::json;

// 1. Service Login Email & Password
pub async fn login_with_email(email: &str, password: &str) -> Result<AuthResponse, String> {
    let client = Client::new();
    let config = AppConfig::init();

    let url = format!("{}/auth/v1/token?grant_type=password", config.supabase_url);

    let res = client
        .post(&url)
        .header("apikey", &config.supabase_key)
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .send()
        .await
        .map_err(|e| format!("Request gagal: {}", e))?;

    if res.status().is_success() {
        let data = res
            .json::<AuthResponse>()
            .await
            .map_err(|e| e.to_string())?;
        Ok(data)
    } else {
        let err_text = res.text().await.unwrap_or_default();
        Err(format!("Login gagal: {}", err_text))
    }
}

// 2. Service Generator URL Google Auth
pub fn generate_google_auth_url() -> String {
    let config = crate::config::AppConfig::init();
    
    // Tanpa /callback
    format!(
        "{}/auth/v1/authorize?provider=google&prompt=consent&redirect_to=com.users.scantrash://auth", 
        config.supabase_url
    )
}
