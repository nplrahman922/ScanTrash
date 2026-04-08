use serde::{Deserialize, Serialize};

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

    let config = get_ai_rules()?;
    let rule = config
        .rules
        .iter()
        .find(|r| r.name == "default")
        .ok_or("Rule default not found")?;

    let hf_text_response = crate::api::analyze_image_with_hf(
        &image.photobase64,
        &config.prompt.system,
        &rule.instruction,
    )
    .await?;

    println!("=== HASIL KEMBALIAN AI ASLI ===");
    println!("{}", hf_text_response);
    println!("===============================");

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
    
    // push the last item
    if has_data {
        items.push(current_item);
    }
    
    // Fallback jika parser gagal sepenuhnya
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
    // Sebagai sanity check
    let base64_str = &image.photobase64;
    let clean_str = if let Some(pos) = base64_str.find(',') {
        &base64_str[pos + 1..]
    } else {
        base64_str
    };

    if clean_str.len() > 10_000_000 {
        // Limit 10MB approx
        return false;
    }

    true
}

fn get_ai_rules() -> Result<AiConfig, String> {
    // Compile-time inclusion agar tetap bisa dibaca saat berjalan di Android
    let config_str = include_str!("ai_config.toml");
    toml::from_str(config_str).map_err(|e| format!("Gagal parsing ai_config.toml: {}", e))
}
