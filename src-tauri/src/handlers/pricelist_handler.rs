use tauri::AppHandle;
use crate::models::Pricelist;
use crate::services::{log_service, local_log_service, pricelist_service, profile_service, session_service};

// ─── READ (semua user terautentikasi) ─────────────────────────────────────────

#[tauri::command]
pub async fn get_pricelist_command(app_handle: AppHandle) -> Result<Vec<Pricelist>, String> {
    let (access_opt, _) = session_service::get_session(&app_handle);

    match access_opt {
        Some(token) => {
            match pricelist_service::fetch_all_pricelist(&app_handle, &token).await {
                Ok(data) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!("Berhasil mengambil {} item pricelist.", data.len()),
                    );
                    Ok(data)
                }
                Err(_e) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "ERROR",
                        "Gagal mengambil data pricelist dari server.",
                    );
                    Err("Gagal memuat daftar harga. Silakan coba lagi.".to_string())
                }
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle,
                "WARNING",
                "Akses pricelist ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}

// ─── Helper guard admin ───────────────────────────────────────────────────────

async fn require_admin(app_handle: &AppHandle, token: &str, aksi: &str) -> Result<String, String> {
    let profile = profile_service::get_user_profile(token).await?;
    if profile.role != "admin" {
        local_log_service::write_local_log(
            app_handle,
            "WARNING",
            &format!(
                "Akses {} pricelist ditolak: User '{}' bukan admin.",
                aksi, profile.username
            ),
        );
        return Err("Akses ditolak: Hanya admin yang dapat mengelola katalog harga.".to_string());
    }
    Ok(profile.username)
}

// ─── CREATE ───────────────────────────────────────────────────────────────────

/// Menambahkan item pricelist baru dengan gambar PNG.
///
/// Frontend: invoke("create_pricelist_command", { labels, price, imageBase64 })
/// - `imageBase64`: data PNG dalam base64 tanpa prefix (wajib untuk tambah)
/// - Validasi: hanya PNG, ukuran ≤ 512 KB dilakukan di service
#[tauri::command]
pub async fn create_pricelist_command(
    app_handle: AppHandle,
    labels: String,
    price: i64,
    image_base64: String,
) -> Result<(), String> {
    // Validasi input dasar
    if labels.trim().is_empty() {
        return Err("Nama sampah tidak boleh kosong.".to_string());
    }
    if price <= 0 {
        return Err("Harga harus lebih dari Rp0.".to_string());
    }
    if image_base64.trim().is_empty() {
        return Err("Gambar wajib dipilih untuk item baru.".to_string());
    }

    let (access_opt, _) = session_service::get_session(&app_handle);
    match access_opt {
        Some(token) => {
            let username = require_admin(&app_handle, &token, "tambah").await?;

            // 1. Upload gambar ke Storage → dapatkan public URL
            let img_url = pricelist_service::upload_image(&app_handle, &token, &image_base64).await?;

            // 2. Insert ke tabel pricelist
            match pricelist_service::create_pricelist(&app_handle, &token, &labels, price, &img_url).await {
                Ok(_) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!("Admin '{}' berhasil menambah item pricelist.", username),
                    );
                    // Audit log: detail boleh lengkap
                    let msg = format!(
                        "[PRICELIST:TAMBAH] labels={}, price=Rp{}/kg, admin={}",
                        labels, price, username
                    );
                    let _ = log_service::insert_log_to_supabase("INFO", &msg, &token).await;
                    Ok(())
                }
                Err(e) => {
                    // Jika insert DB gagal, hapus gambar yang sudah terupload (rollback)
                    pricelist_service::delete_image(&app_handle, &token, &img_url).await;
                    Err(e)
                }
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle, "WARNING", "Akses tambah pricelist ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}

// ─── UPDATE ───────────────────────────────────────────────────────────────────

/// Memperbarui item pricelist. Gambar opsional — jika `image_base64` kosong, gambar lama dipertahankan.
///
/// Frontend: invoke("update_pricelist_command", { id, labels, price, imageBase64 })
/// - `imageBase64`: kosong string = tidak ganti gambar; isi = upload gambar baru
#[tauri::command]
pub async fn update_pricelist_command(
    app_handle: AppHandle,
    id: String,
    labels: String,
    price: i64,
    image_base64: String,    // kosong = tidak ganti gambar
    current_img_url: String, // URL gambar saat ini (untuk delete jika ada gambar baru)
) -> Result<(), String> {
    if id.trim().is_empty() {
        return Err("ID item tidak boleh kosong.".to_string());
    }
    if labels.trim().is_empty() {
        return Err("Nama sampah tidak boleh kosong.".to_string());
    }
    if price <= 0 {
        return Err("Harga harus lebih dari Rp0.".to_string());
    }

    let (access_opt, _) = session_service::get_session(&app_handle);
    match access_opt {
        Some(token) => {
            let username = require_admin(&app_handle, &token, "edit").await?;

            // Jika ada gambar baru → upload dulu
            let new_img_url = if !image_base64.trim().is_empty() {
                Some(pricelist_service::upload_image(&app_handle, &token, &image_base64).await?)
            } else {
                None // Tidak ada gambar baru → pertahankan yang lama
            };

            match pricelist_service::update_pricelist(
                &app_handle,
                &token,
                &id,
                &labels,
                price,
                new_img_url.as_deref(),
            )
            .await
            {
                Ok(_) => {
                    // Jika ada gambar baru yang berhasil diupload, hapus gambar lama dari storage
                    if new_img_url.is_some() && !current_img_url.is_empty() {
                        pricelist_service::delete_image(&app_handle, &token, &current_img_url).await;
                    }

                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!("Admin '{}' berhasil memperbarui item pricelist.", username),
                    );
                    let msg = format!(
                        "[PRICELIST:EDIT] id={}, labels={}, price=Rp{}/kg, ganti_gambar={}, admin={}",
                        id, labels, price, new_img_url.is_some(), username
                    );
                    let _ = log_service::insert_log_to_supabase("INFO", &msg, &token).await;
                    Ok(())
                }
                Err(e) => {
                    // Rollback: hapus gambar baru jika DB update gagal
                    if let Some(ref new_url) = new_img_url {
                        pricelist_service::delete_image(&app_handle, &token, new_url).await;
                    }
                    Err(e)
                }
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle, "WARNING", "Akses edit pricelist ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}

// ─── DELETE ───────────────────────────────────────────────────────────────────

/// Menghapus item pricelist dan gambarnya dari Storage.
///
/// Frontend: invoke("delete_pricelist_command", { id, imgUrl })
#[tauri::command]
pub async fn delete_pricelist_command(
    app_handle: AppHandle,
    id: String,
    img_url: String,
) -> Result<(), String> {
    if id.trim().is_empty() {
        return Err("ID item tidak boleh kosong.".to_string());
    }

    let (access_opt, _) = session_service::get_session(&app_handle);
    match access_opt {
        Some(token) => {
            let username = require_admin(&app_handle, &token, "hapus").await?;

            // 1. Hapus dari DB
            pricelist_service::delete_pricelist(&app_handle, &token, &id).await?;

            // 2. Hapus gambar dari Storage (best-effort, tidak gagalkan operasi)
            if !img_url.is_empty() && img_url != "0" {
                pricelist_service::delete_image(&app_handle, &token, &img_url).await;
            }

            local_log_service::write_local_log(
                &app_handle,
                "INFO",
                &format!("Admin '{}' berhasil menghapus item pricelist.", username),
            );
            let msg = format!("[PRICELIST:HAPUS] id={}, admin={}", id, username);
            let _ = log_service::insert_log_to_supabase("INFO", &msg, &token).await;
            Ok(())
        }
        None => {
            local_log_service::write_local_log(
                &app_handle, "WARNING", "Akses hapus pricelist ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}
