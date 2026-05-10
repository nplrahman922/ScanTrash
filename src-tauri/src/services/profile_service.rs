use crate::config::AppConfig;
use crate::models::Profile;
use crate::services::auth_service; // Import auth_service untuk mendapatkan user_id
use reqwest::Client;

pub async fn get_user_profile(access_token: &str) -> Result<Profile, String> {
    // 1. Dapatkan user_id dari token (MIMIC logika timmu: supabase.auth.getUser)
    let user_id = auth_service::get_user_id(access_token).await?;

    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());
    
    // 2. Tambahkan filter ?user_id=eq.{user_id} ke URL (MIMIC: .eq('user_id', user.id))
    // Kolom ini merujuk pada tabel profiles yang memiliki kolom user_id uuid [cite: 16]
    let url = format!(
        "{}/rest/v1/profiles?select=*&user_id=eq.{}", 
        config.supabase_url, user_id
    );

    let res = client.get(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        // Memaksa Supabase mengembalikan format Objek Tunggal (MIMIC: .single())
        .header("Accept", "application/vnd.pgrst.object+json") 
        .send()
        .await
        .map_err(|_e| "Koneksi ke server gagal saat mengambil profil.".to_string())?;

    if res.status().is_success() {
        let profile: Profile = res.json().await.map_err(|_e| "Format profil dari server tidak sesuai.".to_string())?;
        Ok(profile)
    } else {
        Err("Gagal mengambil profil. Sesi mungkin tidak valid.".to_string())
    }
}