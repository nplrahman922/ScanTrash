use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Deserialize)]
pub struct AiConfig {
    pub prompt: PromptConfig,
    pub rules: Vec<RuleConfig>,
}

#[derive(Debug, Deserialize)]
pub struct PromptConfig {
    pub system: String,
}

#[derive(Debug, Deserialize)]
pub struct RuleConfig {
    pub name: String,
    pub instruction: String,
}

#[derive(Debug, Deserialize)]
pub struct AIImageInput {
    pub photobase64: String,
    pub user_jwt: String, 
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct AIItem {
    pub jenis_sampah: String,
    pub berat_jumlah: String,
    pub estimasi_harga: String,
    pub keterangan: String,
}

// 🌐 FUNGSI PENYELAMAT: Membakar .env ke dalam APK (Sama persis kayak di scan_handler)
fn get_env_var(key: &str) -> String {
    if let Ok(val) = std::env::var(key) {
        return val;
    }
    
    // Asumsi file .env ada di dalam folder src-tauri
    // Kalau error merah pas disave, ganti jadi: "../../../../.env" (naik 4 tingkat)
    let env_content = include_str!("../../../.env"); 
    
    for line in env_content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() { continue; } 
        
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key {
                return v.trim().trim_matches('"').trim_matches('\'').to_string();
            }
        }
    }
    String::new()
}

use crate::services::local_log_service::write_local_log;

pub async fn analisa_image(app_handle: &tauri::AppHandle, image: AIImageInput) -> Result<Vec<AIItem>, String> {
    if !check_size_image(&image) {
        return Err("Ukuran gambar terlalu besar.".into());
    }

    let config = get_ai_rules(app_handle, &image.user_jwt).await?;
    
    let rule = config
        .rules
        .iter()
        .find(|r| r.name == "default")
        .ok_or("Rule default not found")?;

    let hf_text_response = crate::api::analyze_image_with_hf(
        app_handle,
        &image.photobase64,
        &config.prompt.system,
        &rule.instruction,
    )
    .await?;

    write_local_log(app_handle, "INFO", "=== HASIL KEMBALIAN AI ASLI ===");
    write_local_log(app_handle, "INFO", &hf_text_response);
    write_local_log(app_handle, "INFO", "===============================");

    let mut items: Vec<AIItem> = Vec::new();
    let mut current_item = AIItem::default();
    let mut has_data = false;

    for line in hf_text_response.lines() {
        let line = line.trim();
        let lower_line = line.to_lowercase();
        
        if lower_line.starts_with("jenis sampah") {
            if has_data {
                items.push(current_item.clone());
                current_item = AIItem::default();
            }
            if let Some((_, val)) = line.split_once(':') {
                current_item.jenis_sampah = val.trim().to_string();
                has_data = true;
            }
        } else if lower_line.starts_with("jumlah sampah") || lower_line.starts_with("satuan") || lower_line.starts_with("berat") {
            if let Some((_, val)) = line.split_once(':') {
                current_item.berat_jumlah = val.trim().to_string();
                has_data = true;
            }
        } else if lower_line.starts_with("harga") {
            if let Some((_, val)) = line.split_once(':') {
                current_item.estimasi_harga = val.trim().to_string();
                has_data = true;
            }
        } else if lower_line.starts_with("keterangan") {
            if let Some((_, val)) = line.split_once(':') {
                current_item.keterangan = val.trim().to_string();
                has_data = true;
            }
        }
    }
    
    if has_data {
        items.push(current_item);
    }
    
    if items.is_empty() {
        items.push(AIItem {
            jenis_sampah: "Response tidak terformat".to_string(),
            berat_jumlah: "-".to_string(),
            estimasi_harga: "-".to_string(),
            keterangan: hf_text_response,
        });
    }

    Ok(items)
}

fn check_size_image(image: &AIImageInput) -> bool {
    let base64_str = &image.photobase64;
    let clean_str = if let Some(pos) = base64_str.find(',') {
        &base64_str[pos + 1..]
    } else {
        base64_str
    };

    if clean_str.len() > 10_000_000 {
        return false;
    }
    true
}

async fn get_ai_rules(app_handle: &tauri::AppHandle, user_token: &str) -> Result<AiConfig, String> {
    // 👇 SUDAH MEMAKAI get_env_var()
    let supabase_url = get_env_var("SUPABASE_URL");
    let supabase_key = get_env_var("SUPABASE_KEY");

    if supabase_url.is_empty() {
        write_local_log(app_handle, "ERROR", "URL Supabase belum disetting di .env");
        return Err("Terjadi kesalahan konfigurasi server.".to_string());
    }

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    let url_rules = format!("{}/rest/v1/rules?select=content_rules&order=created_at.desc&limit=1", supabase_url);
    let resp_rules = client.get(&url_rules)
        .header("apikey", &supabase_key)
        .header("Authorization", format!("Bearer {}", user_token))
        .send().await.map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim request untuk mengambil rules dari server.");
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    let json_rules: serde_json::Value = resp_rules.json().await.map_err(|_e| {
        write_local_log(app_handle, "ERROR", "Gagal parsing JSON dari respons rules server.");
        "Terjadi kesalahan saat membaca aturan server.".to_string()
    })?;
    
    let mut base_system = String::new();
    let mut instruction = String::new();

    if let Some(rules_obj) = json_rules.as_array().and_then(|arr| arr.get(0)).and_then(|obj| obj.get("content_rules")) {
        base_system = rules_obj.get("system_prompt").and_then(|s| s.as_str()).unwrap_or("").to_string();
        
        if let Some(rule_arr) = rules_obj.get("rules").and_then(|r| r.as_array()) {
            if let Some(first_rule) = rule_arr.get(0) {
                instruction = first_rule.get("instruction").and_then(|s| s.as_str()).unwrap_or("").to_string();
            }
        }
    }

    let url_price = format!("{}/rest/v1/pricelist?select=labels,price", supabase_url);
    let resp_price = client.get(&url_price)
        .header("apikey", &supabase_key)
        .header("Authorization", format!("Bearer {}", user_token)) 
        .send().await.map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim request untuk mengambil pricelist dari server.");
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    let json_price: serde_json::Value = resp_price.json().await.map_err(|_e| {
        write_local_log(app_handle, "ERROR", "Gagal parsing JSON dari respons pricelist server.");
        "Terjadi kesalahan saat memuat daftar harga.".to_string()
    })?;
    
    let mut daftar_harga_teks = String::new();
    if let Some(price_arr) = json_price.as_array() {
        for item in price_arr {
            let nama = item.get("labels").and_then(|s| s.as_str()).unwrap_or("");
            let harga = item.get("price").and_then(|n| n.as_i64()).unwrap_or(0);
            daftar_harga_teks.push_str(&format!("{}: Rp {}/kg\n", nama, harga));
        }
    }

    let final_system_prompt = base_system.replace("{{ DAFTAR_HARGA }}", &daftar_harga_teks);

    Ok(AiConfig {
        prompt: PromptConfig { system: final_system_prompt },
        rules: vec![RuleConfig {
            name: "default".to_string(),
            instruction,
        }],
    })
}