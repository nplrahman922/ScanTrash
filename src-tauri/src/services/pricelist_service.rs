use crate::config::AppConfig;
use crate::models::Pricelist;
use reqwest::Client;

pub async fn fetch_all_pricelist() -> Result<Vec<Pricelist>, String> {
    let client = Client::new();
    let config = AppConfig::init(); // Mengambil config dari config.rs

    let url = format!("{}/rest/v1/pricelist?select=*", config.supabase_url);

    // Supabase butuh 2 header dasar ini untuk akses anonim/public:
    // 1. apikey: Berisi anon key
    // 2. Authorization: Berisi "Bearer <anon_key>"
    let res = client
        .get(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", config.supabase_key))
        .send()
        .await
        .map_err(|e| format!("Gagal mengirim request: {}", e))?;

    if res.status().is_success() {
        let data = res
            .json::<Vec<Pricelist>>()
            .await
            .map_err(|e| e.to_string())?;
        Ok(data)
    } else {
        let err_text = res.text().await.unwrap_or_default();
        Err(format!("Supabase error: {}", err_text))
    }
}
