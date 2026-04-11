use serde::Serialize;
use tauri::AppHandle;
use reqwest::Client;
use serde_json::json;
use crate::services::ai_services::AIImageInput; // Sesuaikan path jika berbeda

#[derive(Serialize)]
pub struct ScanResult {
    pub trash_type: String,
    pub label_id: String,
    pub material_info: String,
    pub kondisi: String,
    pub kebersihan: String,
    pub estimasi_harga: i32,
}

// 🌐 FUNGSI BARU: Tembak data ke tabel 'scan' Supabase
async fn save_scan_to_supabase(scan_data: &ScanResult) -> Result<(), String> {
    let supabase_url = std::env::var("SUPABASE_URL").unwrap_or_default();
    let supabase_key = std::env::var("SUPABASE_KEY").unwrap_or_default();

    if supabase_url.is_empty() || supabase_key.is_empty() {
        return Err("URL atau Key Supabase kosong di .env!".into());
    }

    let client = Client::new();
    let url = format!("{}/rest/v1/scan", supabase_url);

    // Bikin JSON persis dengan nama kolom di tabel scan kalian
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
        .header("Authorization", format!("Bearer {}", supabase_key))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal") // Biar responnya cepat
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

#[tauri::command]
pub async fn scan_trash(app_handle: AppHandle, image: String) -> Result<ScanResult, String> {
    
    // 1. Bersihkan Data Base64
    let clean_base64 = if image.contains(",") {
        image.split(',').nth(1).unwrap_or("").to_string()
    } else {
        image.clone()
    };

    let ai_input = AIImageInput { photobase64: clean_base64 };

    // 2. PANGGIL MESIN AI (Naufal punya)
    let hasil_ai = crate::services::ai_services::analisa_image(ai_input).await?;

    // 3. Ambil Item Pertama
    let item_pertama = hasil_ai.into_iter().next().ok_or("Data kosong, AI gagal membaca sampah.")?;

    // 4. Ubah Teks Harga ("Rp 4.000") Jadi Angka (4000)
    let cleaned_price: String = item_pertama.estimasi_harga
        .chars()
        .filter(|c| c.is_digit(10))
        .collect();
    let price_int: i32 = cleaned_price.parse().unwrap_or(0);

    // 5. Rakit hasil akhir
    let final_result = ScanResult {
        trash_type: item_pertama.jenis_sampah.clone(),
        label_id: item_pertama.jenis_sampah.to_lowercase().replace(" ", "_"),
        material_info: format!("Jumlah/Berat: {}", item_pertama.berat_jumlah),
        kondisi: item_pertama.keterangan,
        kebersihan: "Sesuai deteksi visual".to_string(), // Default aman
        estimasi_harga: price_int,
    };

    // 🚀 6. SIMPAN KE DATABASE SUPABASE (TABEL SCAN)
    // Trik Hackathon: Kita pakai `let _ =` agar kalau nyimpan ke DB gagal (misal internet putus),
    // aplikasinya TIDAK ERROR dan modal hijau di HP tetap muncul!
    match save_scan_to_supabase(&final_result).await {
        Ok(_) => println!("✅ Berhasil menyimpan riwayat scan ke database!"),
        Err(e) => println!("⚠️ Gagal simpan ke DB (tapi AI tetap jalan): {}", e),
    };

    // 7. Kembalikan ke Frontend Vue
    Ok(final_result)
}