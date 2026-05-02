use reqwest::Client;
use serde_json::json;
use tauri::AppHandle;
use crate::services::local_log_service::write_local_log;

pub fn get_hf_token() -> String {
    if let Ok(token) = std::env::var("HF_Token") {
        return token;
    }
    let env_content = include_str!("../../.env");
    for line in env_content.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == "HF_Token" {
                return v.trim().to_string();
            }
        }
    }
    String::new()
}

pub async fn analyze_image_with_hf(
    app_handle: &AppHandle,
    image_base64: &str,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String, String> {
    let token = get_hf_token();
    if token.is_empty() {
        write_local_log(app_handle, "ERROR", "HF_Token tidak ditemukan di .env atau environment");
        return Err("Terjadi kesalahan konfigurasi layanan AI.".to_string());
    }

    // 1. Sanitasi Text Prompt (Mencegah Prompt Injection dari control characters)
    let safe_system_prompt: String = system_prompt.chars().filter(|c| !c.is_control() || *c == '\n' || *c == '\t').collect();
    let safe_user_prompt: String = user_prompt.chars().filter(|c| !c.is_control() || *c == '\n' || *c == '\t').collect();

    // 2. Validasi Ukuran & Sanitasi Image Base64 (Mencegah DoS & Payload berbahanya)
    if image_base64.len() > 7_000_000 {
        write_local_log(app_handle, "WARNING", "Request ditolak: Ukuran gambar base64 terlalu besar (DoS prevention)");
        return Err("Ukuran gambar terlalu besar.".to_string());
    }
    
    // Pastikan base64 hanya berisi karakter valid ASCII untuk base64 / data URI
    if !image_base64.is_ascii() {
        write_local_log(app_handle, "WARNING", "Request ditolak: Base64 mengandung karakter non-ASCII (Potensi Payload Injection)");
        return Err("Format gambar tidak valid.".to_string());
    }

    // 3. Validasi Format Prefix
    let allowed_prefixes = [
        "data:image/jpeg;base64,",
        "data:image/png;base64,",
        "data:image/webp;base64,"
    ];

    let has_valid_prefix = allowed_prefixes.iter().any(|&prefix| image_base64.starts_with(prefix));
    
    let image_data = if has_valid_prefix {
        image_base64.to_string()
    } else {
        if image_base64.starts_with("data:image/") {
            write_local_log(app_handle, "WARNING", "Request ditolak: Format gambar tidak diizinkan");
            return Err("Format gambar tidak diizinkan. Gunakan JPG, PNG, atau WEBP.".to_string());
        }
        format!("data:image/jpeg;base64,{}", image_base64)
    };

    // 3. Timeout 30 detik
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap_or_else(|_| Client::new());

    let model_url = "https://router.huggingface.co/v1/chat/completions";
    let combined_prompt = format!("{}\n\n{}", safe_system_prompt, safe_user_prompt);

    let payload = json!({
        "model": "Qwen/Qwen3-VL-235B-A22B-Instruct",
        "messages": [
            {
                "role": "user",
                "content": [
                    {"type": "text", "text": combined_prompt},
                    {"type": "image_url", "image_url": {"url": image_data}}
                ]
            }
        ],
        "max_tokens": 1024,
        "temperature": 0.3
    });

    let resp = client
        .post(model_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim request ke API HF.");
            "Gagal terhubung ke layanan AI.".to_string()
        })?;

    if resp.status().is_client_error() || resp.status().is_server_error() {
        let err_text = resp.text().await.unwrap_or_default();
        write_local_log(app_handle, "ERROR", &format!("HF API Error: {}", err_text));
        return Err("Layanan AI sedang mengalami gangguan.".to_string());
    }

    let result_json: serde_json::Value = resp
        .json()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal parsing JSON dari respons API HF.");
            "Terjadi kesalahan saat membaca respons AI.".to_string()
        })?;

    if let Some(content) = result_json
        .get("choices")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|s| s.as_str())
    {
        Ok(content.to_string())
    } else {
        if let Some(arr) = result_json.as_array() {
            if let Some(obj) = arr.get(0) {
                if let Some(text) = obj.get("generated_text").and_then(|t| t.as_str()) {
                    return Ok(text.to_string());
                }
            }
        }
        write_local_log(app_handle, "ERROR", "Format respons HF tidak dikenali (struktur JSON tidak sesuai).");
        Err("Format respons AI tidak valid atau tidak dikenali.".to_string())
    }
}