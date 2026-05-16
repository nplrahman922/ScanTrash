use base64::{Engine as _, engine::general_purpose::STANDARD};
use crate::config::AppConfig;
use crate::models::Pricelist;
use crate::services::local_log_service::write_local_log;
use chrono::Utc;
use reqwest::Client;

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Ekstrak filename dari URL gambar.
/// Contoh: "https://.../photo_pricelist/abc.png" → Some("abc.png")
fn extract_filename(img_url: &str) -> Option<String> {
    img_url
        .split("/photo_pricelist/")
        .nth(1)
        .map(|s| s.to_string())
}

fn make_client() -> Client {
    Client::builder()
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .unwrap_or_else(|_| Client::new())
}

// ─── READ ─────────────────────────────────────────────────────────────────────

pub async fn fetch_all_pricelist(
    app_handle: &tauri::AppHandle,
    access_token: &str,
) -> Result<Vec<Pricelist>, String> {
    let config = AppConfig::init();
    let client = make_client();

    let url = format!("{}/rest/v1/pricelist?select=*&order=labels.asc", config.supabase_url);

    let res = client
        .get(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim request ke server untuk mengambil pricelist.");
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if res.status().is_success() {
        let data = res
            .json::<Vec<Pricelist>>()
            .await
            .map_err(|_e| {
                write_local_log(app_handle, "ERROR", "Gagal parsing JSON respons pricelist dari server.");
                "Terjadi kesalahan saat membaca data pricelist.".to_string()
            })?;
        Ok(data)
    } else {
        write_local_log(app_handle, "ERROR", "Server menolak request pricelist (HTTP error).");
        Err("Gagal mengambil data pricelist dari server.".to_string())
    }
}

// ─── IMAGE UPLOAD ─────────────────────────────────────────────────────────────

/// Upload gambar PNG ke Supabase Storage bucket `photo_pricelist`.
///
/// Parameter:
/// - `image_base64`: data gambar dalam format base64 (tanpa prefix data:image/png;base64,)
///
/// Return: Public URL gambar yang bisa langsung dipakai sebagai `img_url`
pub async fn upload_image(
    app_handle: &tauri::AppHandle,
    access_token: &str,
    image_base64: &str,
) -> Result<String, String> {
    // 1. Decode base64 → bytes
    let bytes = STANDARD
        .decode(image_base64)
        .map_err(|_| "Format gambar tidak valid (base64 decode gagal).".to_string())?;

    // 2. Validasi: harus PNG (magic bytes: 89 50 4E 47 0D 0A 1A 0A)
    if bytes.len() < 8 || bytes[0..8] != [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
        write_local_log(app_handle, "WARNING", "Upload gambar ditolak: bukan format PNG.");
        return Err("File harus berformat PNG.".to_string());
    }

    // 3. Validasi ukuran: ≤ 512 KB
    const MAX_SIZE: usize = 512 * 1024;
    if bytes.len() > MAX_SIZE {
        write_local_log(app_handle, "WARNING", "Upload gambar ditolak: ukuran melebihi 512 KB.");
        return Err(format!(
            "Ukuran gambar melebihi batas 512 KB (saat ini: {} KB).",
            bytes.len() / 1024
        ));
    }

    // 4. Generate nama file unik menggunakan timestamp
    let filename = format!("pricelist_{}.png", Utc::now().timestamp_millis());

    let config = AppConfig::init();
    let client = make_client();

    // 5. Upload ke Supabase Storage
    let url = format!(
        "{}/storage/v1/object/photo_pricelist/{}",
        config.supabase_url, filename
    );

    let resp = client
        .post(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "image/png")
        .body(bytes)
        .send()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim gambar ke Supabase Storage.");
            "Koneksi gagal saat upload gambar. Coba lagi.".to_string()
        })?;

    if !resp.status().is_success() {
        write_local_log(
            app_handle,
            "ERROR",
            &format!("Storage menolak upload gambar (HTTP {}).", resp.status().as_u16()),
        );
        return Err("Gagal menyimpan gambar ke server.".to_string());
    }

    // 6. Buat public URL
    let public_url = format!(
        "{}/storage/v1/object/public/photo_pricelist/{}",
        config.supabase_url, filename
    );

    write_local_log(app_handle, "INFO", "Gambar pricelist berhasil diupload.");
    Ok(public_url)
}

// ─── IMAGE DELETE ─────────────────────────────────────────────────────────────

/// Hapus gambar dari Storage berdasarkan URL lengkap.
/// Best-effort: gagal tidak membatalkan operasi DB.
pub async fn delete_image(
    app_handle: &tauri::AppHandle,
    access_token: &str,
    img_url: &str,
) {
    let Some(filename) = extract_filename(img_url) else {
        write_local_log(app_handle, "WARNING", "Gagal ekstrak filename dari img_url, skip delete storage.");
        return;
    };

    let config = AppConfig::init();
    let client = make_client();

    // Supabase Storage delete: DELETE /storage/v1/object/{bucket}/{path}
    let url = format!(
        "{}/storage/v1/object/photo_pricelist/{}",
        config.supabase_url, filename
    );

    match client
        .delete(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {
            write_local_log(app_handle, "INFO", "Gambar lama berhasil dihapus dari storage.");
        }
        Ok(resp) => {
            write_local_log(
                app_handle,
                "WARNING",
                &format!("Gagal hapus gambar lama dari storage (HTTP {}).", resp.status().as_u16()),
            );
        }
        Err(_) => {
            write_local_log(app_handle, "WARNING", "Koneksi gagal saat hapus gambar dari storage.");
        }
    }
}

// ─── CREATE ───────────────────────────────────────────────────────────────────

pub async fn create_pricelist(
    app_handle: &tauri::AppHandle,
    access_token: &str,
    labels: &str,
    price: i64,
    img_url: &str,
) -> Result<(), String> {
    let config = AppConfig::init();
    let client = make_client();

    let url = format!("{}/rest/v1/pricelist", config.supabase_url);

    let body = serde_json::json!({
        "labels":  labels,
        "price":   price,
        "img_url": img_url,
    });

    let resp = client
        .post(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal")
        .json(&body)
        .send()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim request tambah pricelist.");
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if !resp.status().is_success() {
        write_local_log(
            app_handle,
            "ERROR",
            &format!("Server menolak request tambah pricelist (HTTP {}).", resp.status().as_u16()),
        );
        return Err("Gagal menyimpan item ke server.".to_string());
    }

    write_local_log(app_handle, "INFO", "Item pricelist berhasil ditambahkan.");
    Ok(())
}

// ─── UPDATE ───────────────────────────────────────────────────────────────────

/// Update pricelist. `new_img_url` = None berarti pertahankan gambar lama (tidak update img_url).
pub async fn update_pricelist(
    app_handle: &tauri::AppHandle,
    access_token: &str,
    id: &str,
    labels: &str,
    price: i64,
    new_img_url: Option<&str>,
) -> Result<(), String> {
    let config = AppConfig::init();
    let client = make_client();

    let url = format!(
        "{}/rest/v1/pricelist?id=eq.{}",
        config.supabase_url, id
    );

    // Bangun body: selalu update labels dan price; update img_url hanya jika ada gambar baru
    let body = if let Some(img) = new_img_url {
        serde_json::json!({
            "labels":  labels,
            "price":   price,
            "img_url": img,
        })
    } else {
        serde_json::json!({
            "labels": labels,
            "price":  price,
        })
    };

    let resp = client
        .patch(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal")
        .json(&body)
        .send()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim request edit pricelist.");
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if !resp.status().is_success() {
        write_local_log(
            app_handle,
            "ERROR",
            &format!("Server menolak request edit pricelist (HTTP {}).", resp.status().as_u16()),
        );
        return Err("Gagal memperbarui item di server.".to_string());
    }

    write_local_log(app_handle, "INFO", "Item pricelist berhasil diperbarui.");
    Ok(())
}

// ─── DELETE ───────────────────────────────────────────────────────────────────

pub async fn delete_pricelist(
    app_handle: &tauri::AppHandle,
    access_token: &str,
    id: &str,
) -> Result<(), String> {
    let config = AppConfig::init();
    let client = make_client();

    let url = format!(
        "{}/rest/v1/pricelist?id=eq.{}",
        config.supabase_url, id
    );

    let resp = client
        .delete(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Prefer", "return=minimal")
        .send()
        .await
        .map_err(|_e| {
            write_local_log(app_handle, "ERROR", "Gagal mengirim request hapus pricelist.");
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if !resp.status().is_success() {
        write_local_log(
            app_handle,
            "ERROR",
            &format!("Server menolak request hapus pricelist (HTTP {}).", resp.status().as_u16()),
        );
        return Err("Gagal menghapus item dari server.".to_string());
    }

    write_local_log(app_handle, "INFO", "Item pricelist berhasil dihapus.");
    Ok(())
}
