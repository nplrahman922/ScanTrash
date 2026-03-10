// 2. Service Generator URL Google Auth
pub fn generate_google_auth_url() -> String {
    let config = crate::config::AppConfig::init();
    
    // Tanpa /callback
    format!(
        "{}/auth/v1/authorize?provider=google&prompt=consent&redirect_to=com.users.scantrash://auth", 
        config.supabase_url
    )
}
