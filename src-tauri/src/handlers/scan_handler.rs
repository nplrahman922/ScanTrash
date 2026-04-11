use serde::Serialize;
use tauri::{AppHandle, State};
use reqwest::Client;
use serde_json::json;
use crate::services::ai_services::AIImageInput; 
use crate::models::AppState;

#[derive(Serialize)]
pub struct ScanResult {
    pub trash_type: String,
    pub label_id: String,
    pub material_info: String,
    pub kondisi: String,
    pub kebersihan: String,
    pub estimasi_harga: i32,
}

// 🌐 FUNGSI PENYELAMAT: Membakar .env ke dalam APK Android (WAJIB ADA!)
fn get_env_var(key: &str) -> String {
    if let Ok(val) = std::env::var(key) {
        return val;
    }
    
    // Path ke .env (Naik 3 tingkat dari folder handlers)
    let env_content = include_str!("../../../.env"); 
    
    for line in env_content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() { continue; } // Lewati komentar
        
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key {
                // Bersihkan spasi dan tanda kutip jika ada
                return v.trim().trim_matches('"').trim_matches('\'').to_string();
            }
        }
    }
    String::new()
}

// 🌐 FUNGSI BARU: Tembak data ke tabel 'scan' Supabase
async fn save_scan_to_supabase(scan_data: &ScanResult, user_token: &str) -> Result<(), String> {
    // 👇 SUDAH MEMAKAI get_env_var AGAR JALAN DI ANDROID
    let supabase_url = get_env_var("SUPABASE_URL");
    let supabase_key = get_env_var("SUPABASE_KEY");

    if supabase_url.is_empty() || supabase_key.is_empty() {
        return Err("URL atau Key Supabase kosong di .env!".into());
    }

    let client = Client::new();
    let url = format!("{}/rest/v1/scan", supabase_url);

    let payload = json!({
        "trash_type": scan_data.trash_type,
        "label_id": scan_data.label_id,
        "material_info": scan_data.material_info,
        "kondisi": scan_data.kondisi,
        "kebersihan": scan_data.kebersihan,
        "estimasi_harga": scan_data.estimasi_harga
    });

    let resp = client.post(&url)
        .header("apikey", &supabase_key)
        // 🔥 UBAH DI SINI: Sekarang kita pakai JWT milik User (user_token)
        .header("Authorization", format!("Bearer {}", user_token)) 
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Koneksi DB gagal: {}", e))?;

    if !resp.status().is_success() {
        let err = resp.text().await.unwrap_or_default();
        return Err(format!("Gagal Insert: {}", err));
    }

    Ok(())
}

// 👇 WARNING RUST DISELESAIKAN: app_handle diubah menjadi _app_handle
#[tauri::command]
pub async fn scan_trash(
    _app_handle: AppHandle, 
    state: State<'_, AppState>, // 👈 Suntikkan brankas AppState ke sini
    image: String
) -> Result<ScanResult, String> {
    
    // 🔐 1. AMBIL TOKEN DARI BRANKAS (APP STATE)
    let user_token = {
        let lock = state.access_token.lock().map_err(|_| "Gagal membuka brankas token")?;
        lock.clone().ok_or("Sesi habis atau user belum login!")?
    };

    // 2. Bersihkan Data Base64
    let clean_base64 = if image.contains(",") {
        image.split(',').nth(1).unwrap_or("").to_string()
    } else {
        image.clone()
    };

    // 3. Masukkan token ke input AI
    let ai_input = AIImageInput { 
        photobase64: clean_base64,
        user_jwt: user_token.clone(), // Pakai token dari brankas
    };

    // 4. PANGGIL MESIN AI
    let hasil_ai = crate::services::ai_services::analisa_image(ai_input).await?;

    let item_pertama = hasil_ai.into_iter().next().ok_or("Data kosong, AI gagal membaca sampah.")?;

    let cleaned_price: String = item_pertama.estimasi_harga.chars().filter(|c| c.is_digit(10)).collect();
    let price_int: i32 = cleaned_price.parse().unwrap_or(0);

    let final_result = ScanResult {
        trash_type: item_pertama.jenis_sampah.clone(),
        label_id: item_pertama.jenis_sampah.to_lowercase().replace(" ", "_"),
        material_info: format!("Jumlah/Berat: {}", item_pertama.berat_jumlah),
        kondisi: item_pertama.keterangan,
        kebersihan: "Sesuai deteksi visual".to_string(), 
        estimasi_harga: price_int,
    };

    // 🚀 5. SIMPAN KE DATABASE SUPABASE (Pakai token dari brankas)
    match save_scan_to_supabase(&final_result, &user_token).await {
        Ok(_) => println!("✅ Berhasil menyimpan riwayat scan ke database!"),
        Err(e) => println!("⚠️ Gagal simpan ke DB: {}", e),
    };

    Ok(final_result)
}