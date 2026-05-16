use tauri::AppHandle;
use crate::models::Schedule;
use crate::services::{log_service, local_log_service, profile_service, schedule_service, session_service};

// ─── READ (semua user terautentikasi) ────────────────────────────────────────

#[tauri::command]
pub async fn get_schedules_command(app_handle: AppHandle) -> Result<Vec<Schedule>, String> {
    let (access_opt, _) = session_service::get_session(&app_handle);

    match access_opt {
        Some(token) => {
            match schedule_service::fetch_all_schedules(&app_handle, &token).await {
                Ok(data) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!("Berhasil mengambil {} data jadwal setor.", data.len()),
                    );
                    Ok(data)
                }
                Err(_e) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "ERROR",
                        "Gagal mengambil data jadwal setor dari server.",
                    );
                    Err("Gagal memuat jadwal. Silakan coba lagi.".to_string())
                }
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle,
                "WARNING",
                "Akses jadwal setor ditolak: Tidak ada sesi aktif.",
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
                "Akses {} jadwal ditolak: User '{}' bukan admin.",
                aksi, profile.username
            ),
        );
        return Err("Akses ditolak: Hanya admin yang dapat mengelola jadwal.".to_string());
    }
    Ok(profile.username)
}

// ─── CREATE ───────────────────────────────────────────────────────────────────

/// Menambahkan jadwal baru. Hanya admin.
///
/// Frontend: invoke("create_schedule_command", { tanggal, waktuBuka, waktuTutup, lokasi })
/// Format waktu: "HH:MM" (backend menambahkan ":00+08" untuk WITA)
#[tauri::command]
pub async fn create_schedule_command(
    app_handle: AppHandle,
    tanggal: String,
    waktu_buka: String,
    waktu_tutup: String,
    lokasi: String,
) -> Result<(), String> {
    if tanggal.trim().is_empty() || waktu_buka.trim().is_empty()
        || waktu_tutup.trim().is_empty() || lokasi.trim().is_empty()
    {
        return Err("Semua field wajib diisi.".to_string());
    }

    let (access_opt, _) = session_service::get_session(&app_handle);
    match access_opt {
        Some(token) => {
            let username = require_admin(&app_handle, &token, "tambah").await?;

            match schedule_service::create_schedule(
                &app_handle, &token, &tanggal, &waktu_buka, &waktu_tutup, &lokasi,
            )
            .await
            {
                Ok(_) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!("Admin '{}' berhasil menambah jadwal.", username),
                    );
                    // Audit log ke Supabase (detail boleh lengkap)
                    let msg = format!(
                        "[JADWAL:TAMBAH] tanggal={}, lokasi={}, buka={}, tutup={}",
                        tanggal, lokasi, waktu_buka, waktu_tutup
                    );
                    let _ = log_service::insert_log_to_supabase("INFO", &msg, &token).await;
                    Ok(())
                }
                Err(_) => Err("Gagal menyimpan jadwal. Silakan coba lagi.".to_string()),
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle, "WARNING", "Akses tambah jadwal ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}

// ─── UPDATE ───────────────────────────────────────────────────────────────────

/// Memperbarui jadwal yang sudah ada. Hanya admin.
///
/// Frontend: invoke("update_schedule_command", { idTanggal, tanggal, waktuBuka, waktuTutup, lokasi })
#[tauri::command]
pub async fn update_schedule_command(
    app_handle: AppHandle,
    id_tanggal: String,
    tanggal: String,
    waktu_buka: String,
    waktu_tutup: String,
    lokasi: String,
) -> Result<(), String> {
    if id_tanggal.trim().is_empty() || tanggal.trim().is_empty()
        || waktu_buka.trim().is_empty() || waktu_tutup.trim().is_empty()
        || lokasi.trim().is_empty()
    {
        return Err("Semua field wajib diisi.".to_string());
    }

    let (access_opt, _) = session_service::get_session(&app_handle);
    match access_opt {
        Some(token) => {
            let username = require_admin(&app_handle, &token, "edit").await?;

            match schedule_service::update_schedule(
                &app_handle, &token, &id_tanggal, &tanggal, &waktu_buka, &waktu_tutup, &lokasi,
            )
            .await
            {
                Ok(_) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!("Admin '{}' berhasil memperbarui jadwal.", username),
                    );
                    let msg = format!(
                        "[JADWAL:EDIT] id={}, tanggal={}, lokasi={}, buka={}, tutup={}",
                        id_tanggal, tanggal, lokasi, waktu_buka, waktu_tutup
                    );
                    let _ = log_service::insert_log_to_supabase("INFO", &msg, &token).await;
                    Ok(())
                }
                Err(_) => Err("Gagal memperbarui jadwal. Silakan coba lagi.".to_string()),
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle, "WARNING", "Akses edit jadwal ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}

// ─── DELETE ───────────────────────────────────────────────────────────────────

/// Menghapus jadwal berdasarkan UUID. Hanya admin.
///
/// Frontend: invoke("delete_schedule_command", { idTanggal })
#[tauri::command]
pub async fn delete_schedule_command(
    app_handle: AppHandle,
    id_tanggal: String,
) -> Result<(), String> {
    if id_tanggal.trim().is_empty() {
        return Err("ID jadwal tidak boleh kosong.".to_string());
    }

    let (access_opt, _) = session_service::get_session(&app_handle);
    match access_opt {
        Some(token) => {
            let username = require_admin(&app_handle, &token, "hapus").await?;

            match schedule_service::delete_schedule(&app_handle, &token, &id_tanggal).await {
                Ok(_) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!("Admin '{}' berhasil menghapus jadwal.", username),
                    );
                    let msg = format!("[JADWAL:HAPUS] id={}", id_tanggal);
                    let _ = log_service::insert_log_to_supabase("INFO", &msg, &token).await;
                    Ok(())
                }
                Err(_) => Err("Gagal menghapus jadwal. Silakan coba lagi.".to_string()),
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle, "WARNING", "Akses hapus jadwal ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}
