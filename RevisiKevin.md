# Celah Keamanan & bug

1. api.rs
- .map_err(|e| format!("Gagal memanggil API: {}", e))?
    - (Kritis) 
    - Error message bisa contain token jika request gagal
    - Risiko: Token leak di log files atau error reporting
    - Solusi: Log error tanpa detail sensitif; gunakan proper logging framework
    
- pub async fn analyze_image_with_hf(
    image_base64: &str,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String, String>
    - (Tinggi) Celah serangan DDos
    - image_base64 tidak divalidasi (bisa oversized → DoS/memory exhaustion)
    - Prompt tidak disanitasi (bisa prompt injection attack pada AI model)
    - Solusi: Tambah max length validation, sanitasi input
    
- Ok(result_json.to_string())  // Fallback: return raw JSON tanpa parsing
    - (Tinggi) 
    - Jika API response tidak sesuai format, return raw JSON string
    - Bisa expose internal API structure atau unexpected data
    - Solusi: Return error atau sanitized response

- gk ada Timeout rawan crash system
- gk sesuai script dimana format foto bisa berupa png , jpeg , webp di rules_ai

2. scan_handler.rs
- pub async fn scan_trash(
    _app_handle: AppHandle, 
    state: State<'_, AppState>,
    image: String  // ❌ String input tanpa validasi!
)
    - (Tinggi) Celah keamanan DDos
    - image tidak divalidasi (bisa oversized → DoS/memory exhaustion)
    - Solusi: Tambah max length validation, sanitasi input

- return Err(format!("Gagal Insert: {}", err));
    - Error dari DB bisa expose struktur database atau query details
    - Attacker bisa pakai informasi ini untuk SQL injection (jika backend rentan)
    - Solusi: Return error generik tanpa detail

# catatan sebenarnya masih banyak banget bug & code fatal lainnya akan tetapi karena statusnya masih prototype maka masih bisa dimaklumi T_T

# Saran coba test / check / audit ulang code nya jangan langsung serahkan ke AI semua. kalau mau cepat lanjut progres dan aman tanpa harus nunggu revisi numpuk 😤
