use crate::models::AuthResponse;
use crate::services::auth_service;

#[tauri::command]
pub async fn login_email_command(email: String, password: String) -> Result<AuthResponse, String> {
    auth_service::login_with_email(&email, &password).await
}

#[tauri::command]
pub fn get_google_auth_url_command() -> String {
    auth_service::generate_google_auth_url()
}
