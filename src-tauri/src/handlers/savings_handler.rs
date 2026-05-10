use tauri::AppHandle;
use crate::models::SavingsRecord;
use crate::services::{savings_service, session_service, local_log_service};

/// Command untuk mengambil saldo terkini nasabah yang sedang login.
///
/// Return: saldo terkini (i64 dalam satuan Rupiah), atau error string.
/// Frontend: invoke("get_balance_command") → number
#[tauri::command]
pub async fn get_balance_command(
    app_handle: AppHandle,
    target_user_id: Option<String>,
) -> Result<i64, String> {
    let (access_opt, _) = session_service::get_session(&app_handle);

    match access_opt {
        Some(token) => {
            match savings_service::get_current_balance(&app_handle, &token, target_user_id.as_deref()).await {
                Ok(saldo) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!("Saldo berhasil dikirim ke frontend: Rp {}", saldo),
                    );
                    Ok(saldo)
                }
                Err(_e) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "ERROR",
                        "Gagal mengambil saldo dari server.",
                    );
                    Err("Gagal memuat saldo. Silakan coba lagi.".to_string())
                }
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle,
                "WARNING",
                "Akses saldo ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}

/// Command untuk mengambil riwayat transaksi nasabah yang sedang login.
///
/// Return: Vec<SavingsRecord> (maks 50 transaksi terbaru), diurutkan dari terbaru.
/// Frontend: invoke("get_savings_history_command") → array of { id, user_id, amount_before, amount, keterangan, created_at }
#[tauri::command]
pub async fn get_savings_history_command(
    app_handle: AppHandle,
    target_user_id: Option<String>,
) -> Result<Vec<SavingsRecord>, String> {
    let (access_opt, _) = session_service::get_session(&app_handle);

    match access_opt {
        Some(token) => {
            match savings_service::get_savings_history(&app_handle, &token, target_user_id.as_deref()).await {
                Ok(records) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        &format!("Riwayat transaksi ({} data) berhasil dikirim ke frontend.", records.len()),
                    );
                    Ok(records)
                }
                Err(_e) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "ERROR",
                        "Gagal mengambil riwayat transaksi dari server.",
                    );
                    Err("Gagal memuat riwayat transaksi. Silakan coba lagi.".to_string())
                }
            }
        }
        None => {
            local_log_service::write_local_log(
                &app_handle,
                "WARNING",
                "Akses riwayat transaksi ditolak: Tidak ada sesi aktif.",
            );
            Err("Akses ditolak: Tidak ada sesi aktif.".to_string())
        }
    }
}
