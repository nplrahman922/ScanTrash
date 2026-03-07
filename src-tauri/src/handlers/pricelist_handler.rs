use crate::models::Pricelist;
use crate::services::pricelist_service;

#[tauri::command]
// Perhatikan: parameternya sekarang kosong ()
pub async fn get_pricelist_command() -> Result<Vec<Pricelist>, String> {
    pricelist_service::fetch_all_pricelist().await
}
