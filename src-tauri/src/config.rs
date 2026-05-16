pub struct AppConfig {
    pub supabase_url: String,
    pub supabase_key: String,
}

impl AppConfig {
    pub fn init() -> Self {
        AppConfig {
            // dotenv!() membaca nilai dari file .env saat proses kompilasi (build)
            supabase_url: dotenvy_macro::dotenv!("SUPABASE_URL").to_string(),
            supabase_key: dotenvy_macro::dotenv!("SUPABASE_KEY").to_string(),
        }
    }
}
