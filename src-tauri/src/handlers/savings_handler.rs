use tauri::AppHandle;
use crate::models::TransactionItem;
use crate::services::{savings_service, session_service, local_log_service, profile_service};

/// Command untuk mengambil saldo terkini nasabah yang sedang login.
///
/// Return: saldo terkini (i64 dalam satuan Rupiah), atau error string.
/// Frontend: invoke("get_balance_command") → number
#[tauri::command]
pub async fn get_balance_command(
    app_handle: AppHandle,
    target_user_id: String,
) -> Result<i64, String> {
    let (access_opt, _) = session_service::get_session(&app_handle);

    match access_opt {
        Some(token) => {
            let profile = profile_service::get_user_profile(&token).await?;
            if profile.role != "admin" && profile.user_id != target_user_id {
                local_log_service::write_local_log(
                    &app_handle,
                    "WARNING",
                    "Akses saldo ditolak: Percobaan mengakses data pengguna lain.",
                );
                return Err("Akses ditolak: Anda tidak dapat mengakses data pengguna lain.".to_string());
            }

            match savings_service::get_current_balance(&app_handle, &token, &target_user_id).await {
                Ok(saldo) => {
                    local_log_service::write_local_log(
                        &app_handle,
                        "INFO",
                        "Berhasil mengambil saldo terkini.",
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
/// Return: Vec<TransactionItem> (maks 50 transaksi terbaru), sudah berisi kalkulasi
/// nominal dan tipe transaksi (income/expense) dari backend.
/// Frontend: invoke("get_savings_history_command") → array of { id, name, date, nominal, transaction_type }
#[tauri::command]
pub async fn get_savings_history_command(
    app_handle: AppHandle,
    target_user_id: String,
) -> Result<Vec<TransactionItem>, String> {
    let (access_opt, _) = session_service::get_session(&app_handle);

    match access_opt {
        Some(token) => {
            let profile = profile_service::get_user_profile(&token).await?;
            if profile.role != "admin" && profile.user_id != target_user_id {
                local_log_service::write_local_log(
                    &app_handle,
                    "WARNING",
                    "Akses riwayat ditolak: Percobaan mengakses data pengguna lain.",
                );
                return Err("Akses ditolak: Anda tidak dapat mengakses data pengguna lain.".to_string());
            }

            match savings_service::get_savings_history(&app_handle, &token, &target_user_id).await {
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
