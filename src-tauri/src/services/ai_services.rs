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
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct AIItem {
    pub jenis_sampah: String,
    pub berat_jumlah: String,
    pub estimasi_harga: String,
    pub keterangan: String,
}

#[tauri::command]
pub async fn analisa_image(image: AIImageInput) -> Result<Vec<AIItem>, String> {
    if !check_size_image(&image) {
        return Err("Ukuran gambar terlalu besar.".into());
    }

    // [UBAHAN MINOR]: Tambah .await karena get_ai_rules sekarang ngambil dari internet
    let config = get_ai_rules().await?;
    
    let rule = config
        .rules
        .iter()
        .find(|r| r.name == "default")
        .ok_or("Rule default not found")?;

    // [UBAHAN JALUR]: Sesuaikan dengan letak file api.rs barumu
    let hf_text_response = crate::api::analyze_image_with_hf(
        &image.photobase64,
        &config.prompt.system,
        &rule.instruction,
    )
    .await?;

    println!("=== HASIL KEMBALIAN AI ASLI ===");
    println!("{}", hf_text_response);
    println!("===============================");

    // KODE PARSER NAUFAL DI BAWAH INI TIDAK DISENTUH SAMA SEKALI
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

// 🌐 FUNGSI BARU: Merakit data sesuai struktur JSON dan Tabel baru
async fn get_ai_rules() -> Result<AiConfig, String> {
    let supabase_url = std::env::var("SUPABASE_URL").unwrap_or_else(|_| "".to_string());
    let supabase_key = std::env::var("SUPABASE_KEY").unwrap_or_else(|_| "".to_string());

    if supabase_url.is_empty() {
        return Err("URL Supabase belum disetting di .env".to_string());
    }

    let client = Client::new();

    // 1. Tarik Aturan Utama dari JSONB 'content_rules'
    let url_rules = format!("{}/rest/v1/rules?select=content_rules&order=created_at.desc&limit=1", supabase_url);
    let resp_rules = client.get(&url_rules)
        .header("apikey", &supabase_key)
        .header("Authorization", format!("Bearer {}", supabase_key))
        .send().await.map_err(|e| format!("Gagal ambil rules: {}", e))?;

    let json_rules: serde_json::Value = resp_rules.json().await.map_err(|e| format!("Gagal parse rules: {}", e))?;
    
    let mut base_system = String::new();
    let mut instruction = String::new();

    // MENYESUAIKAN DENGAN STRUKTUR JSON BARU MILIKMU
    if let Some(rules_obj) = json_rules.as_array().and_then(|arr| arr.get(0)).and_then(|obj| obj.get("content_rules")) {
        // Ambil system_prompt
        base_system = rules_obj.get("system_prompt").and_then(|s| s.as_str()).unwrap_or("").to_string();
        
        // Ambil instruction yang ada di dalam array "rules" index ke-0
        if let Some(rule_arr) = rules_obj.get("rules").and_then(|r| r.as_array()) {
            if let Some(first_rule) = rule_arr.get(0) {
                instruction = first_rule.get("instruction").and_then(|s| s.as_str()).unwrap_or("").to_string();
            }
        }
    }

    // 2. Tarik Daftar Harga dari tabel `pricelist` (Kolom: labels, price)
    let url_price = format!("{}/rest/v1/pricelist?select=labels,price", supabase_url);
    let resp_price = client.get(&url_price)
        .header("apikey", &supabase_key)
        .header("Authorization", format!("Bearer {}", supabase_key))
        .send().await.map_err(|e| format!("Gagal ambil pricelist: {}", e))?;

    let json_price: serde_json::Value = resp_price.json().await.map_err(|e| format!("Gagal parse pricelist: {}", e))?;
    
    let mut daftar_harga_teks = String::new();
    if let Some(price_arr) = json_price.as_array() {
        for item in price_arr {
            // MENYESUAIKAN NAMA KOLOM BARU
            let nama = item.get("labels").and_then(|s| s.as_str()).unwrap_or("");
            let harga = item.get("price").and_then(|n| n.as_i64()).unwrap_or(0);
            
            daftar_harga_teks.push_str(&format!("{}: Rp {}/kg\n", nama, harga));
        }
    }

    // 3. SUNTIKKAN DAFTAR HARGA KE DALAM PLACEHOLDER {{ DAFTAR_HARGA }}
    let final_system_prompt = base_system.replace("{{ DAFTAR_HARGA }}", &daftar_harga_teks);

    // 4. Return format struct AiConfig
    Ok(AiConfig {
        prompt: PromptConfig { system: final_system_prompt },
        rules: vec![RuleConfig {
            name: "default".to_string(),
            instruction,
        }],
    })
}