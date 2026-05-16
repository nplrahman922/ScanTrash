use crate::config::AppConfig;
use crate::models::{AdminDashboard, AktivitasItem, NasabahItem, Profile, SavingsRecord};
use crate::services::{local_log_service, log_service, savings_service};
use reqwest::Client;
use std::collections::HashMap;
use tauri::AppHandle;

/// Mengambil daftar nasabah (role = 'users') dari tabel profiles,
/// lalu melengkapi setiap nasabah dengan saldo terkininya dari tabel savings.
///
/// Parameter `keyword` digunakan untuk filter pencarian berdasarkan username
/// (case-insensitive, partial match). Kirim string kosong untuk mengambil semua nasabah.
///
/// Return: Vec<NasabahItem> berisi profil + saldo, atau error string.
pub async fn get_nasabah_list(
    app_handle: &AppHandle,
    access_token: &str,
    keyword: &str,
) -> Result<Vec<NasabahItem>, String> {
    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    // Filter: hanya role = 'user' (nilai aktual di DB — BUKAN 'users')
    // Jika keyword tidak kosong, tambahkan filter ilike untuk pencarian username
    let url = if keyword.is_empty() {
        format!(
            "{}/rest/v1/profiles?select=user_id,username,email,photo_url,role&role=eq.user&order=username.asc",
            config.supabase_url
        )
    } else {
        format!(
            "{}/rest/v1/profiles?select=user_id,username,email,photo_url,role&role=eq.user&username=ilike.*{}*&order=username.asc",
            config.supabase_url,
            keyword
        )
    };

    let resp = client
        .get(&url)
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Prefer", "count=none")
        .send()
        .await
        .map_err(|_e| {
            local_log_service::write_local_log(
                app_handle,
                "ERROR",
                "Gagal mengirim request ke server untuk mengambil daftar nasabah.",
            );
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if !resp.status().is_success() {
        local_log_service::write_local_log(
            app_handle,
            "ERROR",
            "Server menolak request daftar nasabah.",
        );
        return Err("Gagal mengambil daftar nasabah dari server.".to_string());
    }

    let profiles: Vec<Profile> = resp.json().await.map_err(|_e| {
        local_log_service::write_local_log(
            app_handle,
            "ERROR",
            "Gagal parsing JSON respons daftar nasabah dari server.",
        );
        "Terjadi kesalahan saat membaca data nasabah.".to_string()
    })?;

    local_log_service::write_local_log(
        app_handle,
        "INFO",
        &format!(
            "Ditemukan {} nasabah. Mengambil saldo masing-masing...",
            profiles.len()
        ),
    );

    // Untuk setiap nasabah, ambil saldo terkininya dengan reuse savings_service
    let mut hasil: Vec<NasabahItem> = Vec::new();
    for profil in profiles {
        let saldo = savings_service::get_current_balance(app_handle, access_token, &profil.user_id)
            .await
            .unwrap_or(0); // Jika gagal ambil saldo, anggap 0 (tidak blokir seluruh list)

        hasil.push(NasabahItem {
            user_id: profil.user_id,
            username: profil.username,
            email: profil.email,
            photo_url: profil.photo_url,
            saldo,
        });
    }

    local_log_service::write_local_log(
        app_handle,
        "INFO",
        &format!(
            "Berhasil menyusun daftar {} nasabah beserta saldo terkini.",
            hasil.len()
        ),
    );

    Ok(hasil)
}

/// Membuat transaksi setoran untuk nasabah tertentu.
///
/// Alur:
/// 1. Ambil saldo terkini nasabah sebagai `amount_before`
/// 2. Hitung `amount` = amount_before + nominal
/// 3. INSERT record baru ke tabel savings
///
/// Parameter:
/// - `target_user_id`: UUID nasabah yang akan disetor
/// - `nominal`: jumlah Rupiah yang disetor (harus > 0)
/// - `keterangan`: deskripsi transaksi (bisa kosong → default "Setoran")
pub async fn create_setoran(
    app_handle: &AppHandle,
    access_token: &str,
    target_user_id: &str,
    nominal: i64,
    keterangan: &str,
) -> Result<(), String> {
    // === Step 1: Ambil saldo sekarang sebagai amount_before ===
    let amount_before = savings_service::get_current_balance(
        app_handle,
        access_token,
        target_user_id,
    )
    .await
    .unwrap_or(0);

    let amount_after = amount_before + nominal;
    let ket = if keterangan.trim().is_empty() {
        "Setoran"
    } else {
        keterangan
    };

    // === Step 2: INSERT ke tabel savings ===
    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    let url = format!("{}/rest/v1/savings", config.supabase_url);

    let body = serde_json::json!({
        "user_id":      target_user_id,
        "amount_before": amount_before,
        "amount":        amount_after,
        "keterangan":    ket,
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
            local_log_service::write_local_log(
                app_handle,
                "ERROR",
                "Gagal mengirim request setoran ke server.",
            );
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if !resp.status().is_success() {
        local_log_service::write_local_log(
            app_handle,
            "ERROR",
            &format!(
                "Server menolak request setoran (HTTP {}).",
                resp.status().as_u16()
            ),
        );
        return Err("Gagal menyimpan setoran ke server.".to_string());
    }

    // Local log: generic, tanpa detail finansial (sesuai kebijakan keamanan)
    local_log_service::write_local_log(
        app_handle,
        "INFO",
        "Setoran berhasil dicatat ke database.",
    );

    // Audit log ke Supabase: boleh detail untuk keperluan audit trail
    let audit_msg = format!(
        "[SETORAN] nasabah={}, nominal=Rp{}, keterangan='{}', saldo_baru=Rp{}.",
        target_user_id, nominal, ket, amount_after
    );
    let _ = log_service::insert_log_to_supabase("INFO", &audit_msg, access_token).await;

    Ok(())
}

/// Membuat transaksi penarikan saldo untuk nasabah tertentu.
///
/// Perbedaan dari setoran:
/// - `amount` = amount_before - nominal (saldo berkurang)
/// - Validasi: nominal tidak boleh melebihi saldo terkini
///
/// Return: Err jika saldo tidak cukup atau request gagal.
pub async fn create_penarikan(
    app_handle: &AppHandle,
    access_token: &str,
    target_user_id: &str,
    nominal: i64,
    keterangan: &str,
) -> Result<(), String> {
    // === Step 1: Ambil saldo sekarang ===
    let amount_before = savings_service::get_current_balance(
        app_handle,
        access_token,
        target_user_id,
    )
    .await
    .unwrap_or(0);

    // === Validasi: saldo harus mencukupi ===
    if nominal > amount_before {
    // Local log: generic, tanpa nominal (sesuai kebijakan keamanan)
        local_log_service::write_local_log(
            app_handle,
            "WARNING",
            "Penarikan ditolak: saldo nasabah tidak mencukupi.",
        );
        return Err(format!(
            "Saldo nasabah tidak mencukupi. Saldo saat ini: Rp{}.",
            amount_before.to_string()
                .chars()
                .rev()
                .enumerate()
                .map(|(i, c)| if i > 0 && i % 3 == 0 { format!(".{}", c) } else { c.to_string() })
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<String>()
        ));
    }

    let amount_after = amount_before - nominal;
    let ket = if keterangan.trim().is_empty() {
        "Penarikan"
    } else {
        keterangan
    };

    // === Step 2: INSERT ke tabel savings ===
    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    let url = format!("{}/rest/v1/savings", config.supabase_url);

    let body = serde_json::json!({
        "user_id":       target_user_id,
        "amount_before": amount_before,
        "amount":        amount_after,
        "keterangan":    ket,
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
            local_log_service::write_local_log(
                app_handle,
                "ERROR",
                "Gagal mengirim request penarikan ke server.",
            );
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if !resp.status().is_success() {
        local_log_service::write_local_log(
            app_handle,
            "ERROR",
            &format!(
                "Server menolak request penarikan (HTTP {}).",
                resp.status().as_u16()
            ),
        );
        return Err("Gagal menyimpan penarikan ke server.".to_string());
    }

    // Local log: generic, tanpa detail finansial
    local_log_service::write_local_log(
        app_handle,
        "INFO",
        "Penarikan berhasil dicatat ke database.",
    );

    // Audit log ke Supabase: boleh detail untuk keperluan audit trail
    let audit_msg = format!(
        "[PENARIKAN] nasabah={}, nominal=Rp{}, keterangan='{}', saldo_baru=Rp{}.",
        target_user_id, nominal, ket, amount_after
    );
    let _ = log_service::insert_log_to_supabase("INFO", &audit_msg, access_token).await;

    Ok(())
}

/// Mengambil data ringkasan untuk dashboard admin:
/// - total_saldo: jumlah saldo semua nasabah
/// - total_nasabah: jumlah nasabah (role = 'users')
/// - aktivitas_terbaru: 10 transaksi terbaru dari SEMUA nasabah
///
/// Strategi query (efisien, tanpa N+1 untuk aktivitas):
/// 1. Reuse get_nasabah_list("") → dapat total_nasabah + total_saldo + HashMap username
/// 2. Satu query langsung ke savings tanpa filter user_id → 10 aktivitas terbaru
///    Username di-lookup dari HashMap, zero query tambahan per baris.
pub async fn get_admin_dashboard(
    app_handle: &AppHandle,
    access_token: &str,
) -> Result<AdminDashboard, String> {
    // === Step 1: Ambil semua nasabah (reuse fungsi yang sudah ada) ===
    let nasabah_list = get_nasabah_list(app_handle, access_token, "").await?;

    let total_nasabah = nasabah_list.len() as i64;
    let total_saldo: i64 = nasabah_list.iter().map(|n| n.saldo).sum();

    // Bangun HashMap untuk lookup username per user_id tanpa query tambahan
    let username_map: HashMap<String, String> = nasabah_list
        .into_iter()
        .map(|n| (n.user_id, n.username))
        .collect();

    // === Step 2: Query savings langsung tanpa filter user_id ===
    // Satu request untuk semua aktivitas terbaru lintas nasabah
    let config = AppConfig::init();
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    let url = format!(
        "{}/rest/v1/savings?select=*&order=created_at.desc&limit=10",
        config.supabase_url
    );

    let resp = client
        .get(&url)
        // RLS dihandle DB dengan policy is_admin — anon key + JWT admin sudah cukup
        .header("apikey", &config.supabase_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|_e| {
            local_log_service::write_local_log(
                app_handle,
                "ERROR",
                "Gagal mengirim request aktivitas terbaru ke server.",
            );
            "Koneksi gagal. Pastikan internet Anda aktif.".to_string()
        })?;

    if !resp.status().is_success() {
        local_log_service::write_local_log(
            app_handle,
            "ERROR",
            "Server menolak request aktivitas terbaru (HTTP error).",
        );
        return Err("Gagal mengambil aktivitas terbaru dari server.".to_string());
    }

    let records: Vec<SavingsRecord> = resp.json().await.map_err(|_e| {
        local_log_service::write_local_log(
            app_handle,
            "ERROR",
            "Gagal parsing JSON aktivitas terbaru dari server.",
        );
        "Terjadi kesalahan saat membaca data aktivitas.".to_string()
    })?;

    // === Step 3: Map SavingsRecord → AktivitasItem ===
    // Logika kalkulasi income/expense sama dengan get_savings_history()
    let aktivitas_terbaru: Vec<AktivitasItem> = records
        .into_iter()
        .enumerate()
        .map(|(i, rec)| {
            let before = rec.amount_before.unwrap_or(0);
            let after = rec.amount.unwrap_or(0);
            let is_income = after >= before;
            let nominal = (after - before).unsigned_abs() as i64;

            let uid = rec.user_id.clone().unwrap_or_default();
            // Lookup username dari HashMap — tidak ada query tambahan
            let username = username_map
                .get(&uid)
                .cloned()
                .unwrap_or_else(|| "Nasabah".to_string());

            let keterangan = rec.keterangan.filter(|s| !s.is_empty()).unwrap_or_else(|| {
                if is_income {
                    "Setoran".to_string()
                } else {
                    "Penarikan".to_string()
                }
            });

            AktivitasItem {
                id: rec.id.unwrap_or_else(|| i.to_string()),
                username,
                user_id: uid,
                date: rec.created_at.unwrap_or_default(),
                keterangan,
                nominal,
                transaction_type: if is_income {
                    "income".to_string()
                } else {
                    "expense".to_string()
                },
            }
        })
        .collect();

    local_log_service::write_local_log(
        app_handle,
        "INFO",
        &format!(
            "Dashboard admin berhasil dimuat: {} nasabah, total saldo Rp{}, {} aktivitas terbaru.",
            total_nasabah,
            total_saldo,
            aktivitas_terbaru.len()
        ),
    );

    Ok(AdminDashboard {
        total_saldo,
        total_nasabah,
        aktivitas_terbaru,
    })
}
