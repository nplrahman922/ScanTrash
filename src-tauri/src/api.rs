use reqwest::Client;
use serde_json::json;

pub fn get_hf_token() -> String {
    // Membaca dari environment variables jika tersedia
    if let Ok(token) = std::env::var("HF_Token") {
        return token;
    }

    // Fallback: Untuk Android/build, kita embed isi .env saat compile time
    // Ini memastikan nilai HF_Token tetap terbaca di Android tanpa perlu akses filesystem runtime
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
    image_base64: &str,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String, String> {
    let token = get_hf_token();
    if token.is_empty() {
        return Err("HF_Token tidak ditemukan di .env atau environment".to_string());
    }

    let client = Client::new();

    // Menggunakan Endpoint Hugging Face Router API dengan provider Cohere (aya-vision-32b)
    let model_url = "https://router.huggingface.co/v1/chat/completions";

    // Pastikan base64 memiliki prefiks data URI yang valid
    let image_data = if image_base64.starts_with("data:image") {
        image_base64.to_string()
    } else {
        format!("data:image/jpeg;base64,{}", image_base64)
    };

    let payload = json!({
        "model": "Qwen/Qwen3-VL-235B-A22B-Instruct",
        "messages": [
            {
                "role": "system",
                "content": system_prompt
            },
            {
                "role": "user",
                "content": [
                    {"type": "text", "text": user_prompt},
                    {"type": "image_url", "image_url": {"url": image_data}}
                ]
            }
        ],
        "max_tokens": 1024
    });

    let resp = client
        .post(model_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Gagal memanggil API: {}", e))?;

    if resp.status().is_client_error() || resp.status().is_server_error() {
        let err_text = resp.text().await.unwrap_or_default();
        return Err(format!("HF API Error: {}", err_text));
    }

    let result_json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Gagal parsing JSON: {}", e))?;

    // Parsing response format Chat Completion
    if let Some(content) = result_json
        .get("choices")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|s| s.as_str())
    {
        Ok(content.to_string())
    } else {
        // Fallback jika HF mengembalikan format generated_text / array
        if let Some(arr) = result_json.as_array() {
            if let Some(obj) = arr.get(0) {
                if let Some(text) = obj.get("generated_text").and_then(|t| t.as_str()) {
                    return Ok(text.to_string());
                }
            }
        }
        Ok(result_json.to_string())
    }
}
