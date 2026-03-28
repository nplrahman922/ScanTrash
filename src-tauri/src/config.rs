pub struct AppConfig {
    pub supabase_url: String,
    pub supabase_key: String,
}

impl AppConfig {
    pub fn init() -> Self {
        AppConfig {
            // dotenv!() akan menyedot nilai dari file .env SAAT proses kompilasi (build)
            supabase_url: dotenvy_macro::dotenv!("SUPABASE_URL").to_string(),
            supabase_key: dotenvy_macro::dotenv!("SUPABASE_KEY").to_string(),
        }
    }
}