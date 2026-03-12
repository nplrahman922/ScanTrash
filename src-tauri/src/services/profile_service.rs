use crate::config::AppConfig;
use crate::models::Profile;
use reqwest::Client;

pub async fn get_user_profile(access_token: &str) -> Result<Profile, String> {
    let config = AppConfig::init();
    let client = Client::new();
    
    // Tembak tabel profiles
    let url = format!("{}/rest/v1/profiles?select=*", config.supabase_url);

    let res = client.get(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        // Trik Supabase: Minta kembalikan 1 Object JSON saja, jangan format Array []
        .header("Accept", "application/vnd.pgrst.object+json") 
        .send()
        .await
        .map_err(|e| format!("Gagal fetch Supabase: {}", e))?;

    if res.status().is_success() {
        let profile: Profile = res.json().await.map_err(|e| format!("JSON Error: {}", e))?;
        Ok(profile)
    } else {
        Err("Profil tidak ditemukan di database".to_string())
    }
}