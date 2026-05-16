use crate::models::{AdminDashboard, NasabahItem};
use crate::services::{admin_service, local_log_service, profile_service, session_service};
use tauri::AppHandle;

/// Command untuk mengambil daftar nasabah beserta saldo terkini.
/// Hanya dapat diakses oleh user dengan role "admin".
///
/// Parameter `keyword`: string pencarian berdasarkan username (case-insensitive).
/// Kirim string kosong ("") untuk mendapatkan semua nasabah tanpa filter.
///
/// Return: Vec<NasabahItem>, atau error string.
/// Frontend: invoke("get_nasabah_list_command", { keyword: "" }) → NasabahItem[]
#[tauri::command]
pub async fn get_nasabah_list_command(
    app_handle: AppHandle,
    keyword: String,
) -> Result<Vec<NasabahItem>, String> {
    // 1. Ambil sesi aktif
    let (access_opt, _) = session_service::get_session(&app_handle);

    match access_opt {
        Some(token) => {
            // 2. Verifikasi role admin — tolak jika bukan admin
            let profile = profile_service::get_user_profile(&token).await?;
            if profile.role != "admin" {
                local_log_service::write_local_log(
                    &app_handle,
                    "WARNING",
                    &format!(
                        "Akses daftar nasabah ditolak: User '{}' bukan admin.",
                        profile.username
                    ),
                );
                return Err(
                    "Akses ditolak: Hanya admin yang dapat mengakses fitur ini.".to_string()
                );
            }

            // 3. Panggil service untuk ambil data
            let keyword_log = if keyword.is_empty() {
                "semua nasabah".to_string()
            } else {
                format!("keyword '{}'", keyword)
            };

            match admin_service::get_nasabah_list(&app_handle, &token, &keyword).await {
                Ok(list) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!(
                            "Admin '{}' berhasil mengambil {} nasabah ({}).",
                            profile.username,
                            list.len(),
                            keyword_log
                        ),
                    );
                    Ok(list)
                }
                Err(_e) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "ERROR",
                        &format!(
                            "Admin '{}' gagal mengambil daftar nasabah ({}).",
                            profile.username, keyword_log
                        ),
                    );
                    Err("Gagal memuat daftar nasabah. Silakan coba lagi.".to_string())
                }
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle,
                "WARNING",
                "Akses daftar nasabah ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}

/// Command untuk mengambil data dashboard admin:
/// total saldo, total nasabah, dan 10 aktivitas transaksi terbaru lintas nasabah.
///
/// Tidak memerlukan parameter — semua data diambil otomatis dari token admin.
/// Frontend: invoke("get_admin_dashboard_command") → AdminDashboard
#[tauri::command]
pub async fn get_admin_dashboard_command(app_handle: AppHandle) -> Result<AdminDashboard, String> {
    // 1. Ambil sesi aktif
    let (access_opt, _) = session_service::get_session(&app_handle);

    match access_opt {
        Some(token) => {
            // 2. Verifikasi role admin
            let profile = profile_service::get_user_profile(&token).await?;
            if profile.role != "admin" {
                local_log_service::write_local_log(
                    &app_handle,
                    "WARNING",
                    &format!(
                        "Akses dashboard admin ditolak: User '{}' bukan admin.",
                        profile.username
                    ),
                );
                return Err(
                    "Akses ditolak: Hanya admin yang dapat mengakses fitur ini.".to_string()
                );
            }

            // 3. Ambil data dashboard
            match admin_service::get_admin_dashboard(&app_handle, &token).await {
                Ok(dashboard) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!("Admin '{}' berhasil memuat dashboard.", profile.username),
                    );
                    Ok(dashboard)
                }
                Err(_e) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "ERROR",
                        &format!("Admin '{}' gagal memuat dashboard.", profile.username),
                    );
                    Err("Gagal memuat data dashboard. Silakan coba lagi.".to_string())
                }
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle,
                "WARNING",
                "Akses dashboard admin ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}

/// Command untuk membuat transaksi setoran oleh admin.
///
/// Parameter:
/// - `target_user_id`: UUID nasabah penerima setoran
/// - `nominal`: jumlah Rupiah yang disetor (> 0)
/// - `keterangan`: catatan transaksi (kosong → "Setoran")
///
/// Frontend: invoke("create_setoran_command", { targetUserId, nominal, keterangan })
#[tauri::command]
pub async fn create_setoran_command(
    app_handle: AppHandle,
    target_user_id: String,
    nominal: i64,
    keterangan: String,
) -> Result<(), String> {
    // 1. Validasi input dasar sebelum request ke server
    if target_user_id.trim().is_empty() {
        return Err("Pilih nasabah terlebih dahulu.".to_string());
    }
    if nominal <= 0 {
        return Err("Nominal setoran harus lebih dari Rp0.".to_string());
    }

    // 2. Ambil sesi aktif
    let (access_opt, _) = session_service::get_session(&app_handle);

    match access_opt {
        Some(token) => {
            // 3. Guard: hanya admin
            let profile = profile_service::get_user_profile(&token).await?;
            if profile.role != "admin" {
                local_log_service::write_local_log(
                    &app_handle,
                    "WARNING",
                    &format!(
                        "Akses setoran ditolak: User '{}' bukan admin.",
                        profile.username
                    ),
                );
                return Err(
                    "Akses ditolak: Hanya admin yang dapat melakukan setoran.".to_string(),
                );
            }

            // 4. Proses setoran
            match admin_service::create_setoran(
                &app_handle,
                &token,
                &target_user_id,
                nominal,
                &keterangan,
            )
            .await
            {
                Ok(_) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!(
                            "Admin '{}' berhasil melakukan setoran untuk nasabah.",
                            profile.username
                        ),
                    );
                    Ok(())
                }
                Err(_e) => {
                    Err("Gagal menyimpan setoran. Silakan coba lagi.".to_string())
                }
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle,
                "WARNING",
                "Akses setoran ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}

/// Command untuk membuat transaksi penarikan saldo oleh admin.
///
/// Parameter:
/// - `target_user_id`: UUID nasabah
/// - `nominal`: jumlah Rupiah yang ditarik (harus > 0 dan <= saldo nasabah)
/// - `keterangan`: catatan transaksi (kosong → "Penarikan")
///
/// Frontend: invoke("create_penarikan_command", { targetUserId, nominal, keterangan })
#[tauri::command]
pub async fn create_penarikan_command(
    app_handle: AppHandle,
    target_user_id: String,
    nominal: i64,
    keterangan: String,
) -> Result<(), String> {
    // 1. Validasi input dasar
    if target_user_id.trim().is_empty() {
        return Err("Pilih nasabah terlebih dahulu.".to_string());
    }
    if nominal <= 0 {
        return Err("Jumlah penarikan harus lebih dari Rp0.".to_string());
    }

    // 2. Ambil sesi
    let (access_opt, _) = session_service::get_session(&app_handle);

    match access_opt {
        Some(token) => {
            // 3. Guard: hanya admin
            let profile = profile_service::get_user_profile(&token).await?;
            if profile.role != "admin" {
                local_log_service::write_local_log(
                    &app_handle,
                    "WARNING",
                    &format!(
                        "Akses penarikan ditolak: User '{}' bukan admin.",
                        profile.username
                    ),
                );
                return Err(
                    "Akses ditolak: Hanya admin yang dapat melakukan penarikan.".to_string(),
                );
            }

            // 4. Proses penarikan (termasuk validasi saldo di service)
            match admin_service::create_penarikan(
                &app_handle,
                &token,
                &target_user_id,
                nominal,
                &keterangan,
            )
            .await
            {
                Ok(_) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!(
                            "Admin '{}' berhasil melakukan penarikan untuk nasabah.",
                            profile.username
                        ),
                    );
                    Ok(())
                }
                Err(e) => {
                    // Teruskan pesan error spesifik (misal saldo tidak cukup) ke frontend
                    Err(e)
                }
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle,
                "WARNING",
                "Akses penarikan ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}
