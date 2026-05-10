use serde::Serialize;
use reqwest::Client;
use serde_json::json;
use tauri::AppHandle;

use crate::services::ai_services::AIImageInput;
use crate::services::local_log_service::write_local_log;

#[derive(Serialize)]
pub struct ScanResult {
    pub trash_type: String,
    pub label_id: String,
    pub material_info: String,
    pub kondisi: String,
    pub kebersihan: String,
    pub estimasi_harga: i32,
}

fn get_env_var(key: &str) -> String {
    if let Ok(val) = std::env::var(key) { return val; }
    let env_content = include_str!("../../../.env"); 
    for line in env_content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() { continue; } 
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key { return v.trim().trim_matches('"').trim_matches('\'').to_string(); }
        }
    }
    String::new()
}

pub async fn save_scan_to_supabase(app_handle: &AppHandle, scan_data: &[ScanResult], user_token: &str) -> Result<(), String> {
    let supabase_url = get_env_var("SUPABASE_URL");
    let supabase_key = get_env_var("SUPABASE_KEY");

    if supabase_url.is_empty() || supabase_key.is_empty() {
        write_local_log(app_handle, "ERROR", "URL atau Key Supabase kosong di .env!");
        return Err("Terjadi kesalahan konfigurasi server.".into());
    }

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap_or_else(|_| Client::new());
        
    let url = format!("{}/rest/v1/scan", supabase_url);

    let payload = json!(scan_data.iter().map(|s| json!({
        "trash_type": s.trash_type,
        "label_id": s.label_id,
        "material_info": s.material_info,
        "kondisi": s.kondisi,
        "kebersihan": s.kebersihan,
        "estimasi_harga": s.estimasi_harga
    })).collect::<Vec<_>>());

    let resp = client.post(&url)
        .header("apikey", &supabase_key)
        .header("Authorization", format!("Bearer {}", user_token)) 
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal")
        .json(&payload)
        .send()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Koneksi ke server database gagal saat menyimpan scan.");
            "Gagal menghubungi server database.".to_string()
        })?;

    if !resp.status().is_success() {
        let _err = resp.text().await.unwrap_or_default();
        write_local_log(app_handle, "ERROR", "Server database menolak data scan (HTTP error).");
        return Err("Gagal menyimpan data scan ke sistem.".into());
    }
    Ok(())
}

pub async fn process_scan_image(
    app_handle: &AppHandle,
    user_token: &str,
    image: String,
) -> Result<Vec<ScanResult>, String> {
    
    let clean_base64 = if image.contains(",") {
        image.split(',').nth(1).unwrap_or("").to_string()
    } else {
        image.clone()
    };

    let ai_input = AIImageInput { 
        photobase64: clean_base64,
        user_jwt: user_token.to_string(),
    };

    let hasil_ai = crate::services::ai_services::analisa_image(app_handle, ai_input).await?;

    if hasil_ai.is_empty() {
        return Err("Data kosong, AI gagal membaca sampah.".into());
    }

    let mut final_results = Vec::new();

    for item in hasil_ai {
        let cleaned_price: String = item.estimasi_harga.chars().filter(|c| c.is_digit(10)).collect();
        let price_int: i32 = cleaned_price.parse().unwrap_or(0);

        final_results.push(ScanResult {
            trash_type: item.jenis_sampah.clone(),
            label_id: item.jenis_sampah.to_lowercase().replace(" ", "_"),
            material_info: format!("Jumlah/Berat: {}", item.berat_jumlah),
            kondisi: item.keterangan,
            kebersihan: "Sesuai deteksi visual".to_string(),
            estimasi_harga: price_int,
        });
    }

    match save_scan_to_supabase(app_handle, &final_results, user_token).await {
        Ok(_) => write_local_log(app_handle, "INFO", &format!("Berhasil menyimpan {} objek ke database!", final_results.len())),
        Err(e) => return Err(e),
    };

    Ok(final_results)
}
