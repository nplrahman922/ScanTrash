# 🌱 EcoScan — Dokumentasi Teknis AI (Versi Penjelasan)

> Ditulis oleh Antigravity AI sebagai penjelas pendamping `CatatanPengembang.md` yang ditulis EnBee.
> Dokumen ini menjelaskan kode yang ada baris per baris, lengkap dengan diagram alur.

---

## 📐 1. Arsitektur Besar Proyek

Proyek ini dibangun di atas fondasi **Tauri V2**, sebuah framework yang memungkinkan kita membangun aplikasi Android/Desktop menggunakan dua bahasa sekaligus: **Vue 3 (TypeScript)** untuk tampilan, dan **Rust** untuk logika backend yang berat dan aman.

Keduanya tidak berbicara langsung—mereka berkomunikasi lewat jembatan khusus milik Tauri yang disebut **IPC Bridge (Invoke)**.

```
┌─────────────────────────────────────────────────────────────┐
│                      ANDROID APP                            │
│                                                             │
│  ┌─────────────────────┐        ┌─────────────────────────┐ │
│  │   FRONTEND (Vue 3)  │  IPC   │   BACKEND (Rust/Tauri)  │ │
│  │   App.vue           │◄──────►│   lib.rs                │ │
│  │   TypeScript        │ invoke │   ai_services.rs        │ │
│  │   style.css         │        │   api.rs                │ │
│  └─────────────────────┘        └──────────┬──────────────┘ │
│                                            │ HTTPS reqwest  │
└────────────────────────────────────────────┼────────────────┘
                                             │
                                             ▼
                               ┌─────────────────────────┐
                               │   Hugging Face Cloud     │
                               │   router.huggingface.co  │
                               │   Model: Qwen3-VL-235B   │
                               └─────────────────────────┘
```

---

## 🔄 2. Sequence Diagram — Alur Utama (Happy Path)

Diagram berikut menunjukkan alur lengkap dari saat pengguna menyentuh layar hingga hasil harga muncul.

```
Pengguna        App.vue (Vue 3)      ai_services.rs (Rust)   api.rs (Rust)    HuggingFace API
   │                 │                       │                    │                  │
   │  Ketuk layar    │                       │                    │                  │
   │───────────────► │                       │                    │                  │
   │                 │ Buka kamera / galeri  │                    │                  │
   │◄─────────────── │                       │                    │                  │
   │                 │                       │                    │                  │
   │  Pilih foto     │                       │                    │                  │
   │───────────────► │                       │                    │                  │
   │                 │ FileReader.readAsDataURL()                 │                  │
   │                 │ (konversi foto → Base64 string)            │                  │
   │                 │                       │                    │                  │
   │                 │ invoke("analisa_image")│                    │                  │
   │                 │──────────────────────►│                    │                  │
   │                 │                       │ check_size_image() │                  │
   │                 │                       │ (cek ukuran < 10MB)│                  │
   │                 │                       │                    │                  │
   │                 │                       │ get_ai_rules()     │                  │
   │                 │                       │ (baca ai_config.toml via include_str) │
   │                 │                       │                    │                  │
   │                 │                       │ analyze_image_with_hf()               │
   │                 │                       │───────────────────►│                  │
   │                 │                       │                    │ get_hf_token()   │
   │                 │                       │                    │ (cek env var     │
   │                 │                       │                    │  fallback ke .env│
   │                 │                       │                    │  yang di-embed)  │
   │                 │                       │                    │                  │
   │                 │                       │                    │ POST /v1/chat/completions
   │                 │                       │                    │─────────────────►│
   │                 │                       │                    │   [model, system_prompt,
   │                 │                       │                    │    user_prompt, image_b64]
   │                 │                       │                    │                  │
   │                 │                       │                    │◄─────────────────│
   │                 │                       │                    │  Raw text response│
   │                 │                       │                    │  (choices[0].message.content)
   │                 │                       │◄───────────────────│                  │
   │                 │                       │  hf_text_response  │                  │
   │                 │                       │                    │                  │
   │                 │                       │ [Parser Loop]      │                  │
   │                 │                       │ baca per baris:    │                  │
   │                 │                       │ "Jenis Sampah : ..." → AIItem.jenis  │
   │                 │                       │ "Jumlah Sampah: ..." → AIItem.berat  │
   │                 │                       │ "Harga         : ..." → AIItem.harga │
   │                 │                       │ "Keterangan    : ..." → AIItem.ket.  │
   │                 │                       │ (baris "Perhitungan" diabaikan!)      │
   │                 │                       │                    │                  │
   │                 │◄──────────────────────│                    │                  │
   │                 │  Vec<AIItem> (array hasil)                 │                  │
   │                 │                       │                    │                  │
   │ Tampilkan hasil │                       │                    │                  │
   │◄─────────────── │                       │                    │                  │
   │  Kartu per item │                       │                    │                  │
```

---

## 📂 3. Penjelasan Per File Kode

### 3.1 `src/App.vue` — Wajah Aplikasi

File ini adalah **satu-satunya komponen UI** di proyek ini. Ini jadi titik pertama yang berinteraksi dengan pengguna.

**Bagian `<script setup>`** — Logika TypeScript:

```typescript
// Antarmuka TypeScript yang merepresentasikan 1 buah objek sampah
// Harus PERSIS cocok dengan struct AIItem di Rust
interface AIItem {
  jenis_sampah: string;   // ← nama sampah
  berat_jumlah: string;   // ← jumlah yang terdeteksi
  estimasi_harga: string; // ← hasil harga akhir
  keterangan: string;     // ← deskripsi kondisi fisik
}
```

Fungsi paling krusial di file ini adalah `onFileChange()`. Inilah titik masuk data gambar:

```typescript
// Saat pengguna memilih foto, fungsi ini berjalan
async function onFileChange(event: Event) {
  // ...
  const reader = new FileReader();
  reader.onload = async (e) => {
    const base64Str = e.target?.result as string; // ← foto diubah jadi teks base64
    imagePreview.value = base64Str;               // ← langsung ditampilkan di layar
    
    isProcessing.value = true; // ← aktifkan spinner loading
    
    // TITIK KUNCI: Kirim data ke Rust via Tauri IPC
    const response = await invoke<AIItem[]>("analisa_image", { 
      image: { photobase64: base64Str } 
    });
    
    results.value = response || [];
    isProcessing.value = false;
  };
  reader.readAsDataURL(file); // ← mulai konversi foto ke Base64
}
```

**Poin desain penting:** `capture="environment"` pada tag `<input>` memastikan kamera belakang HP yang terbuka secara default saat dipakai di Android.

---

### 3.2 `src-tauri/src/lib.rs` — Pintu Masuk Backend Rust

File ini adalah **entry point** aplikasi Tauri. Fungsinya seperti `main.go` atau `index.js` di framework lain: mendaftarkan semua command Rust agar bisa dipanggil dari Vue.

```rust
// Mendaftarkan command "analisa_image" ke Tauri
// Tanpa baris ini, invoke("analisa_image") dari Vue akan gagal
.invoke_handler(tauri::generate_handler![greet, ai_services::analisa_image])
```

`greet` adalah command bawaan Tauri yang tidak dipakai, bisa dihapus di production.

---

### 3.3 `src-tauri/src/ai_services.rs` — Otak Pemrosesan

File ini adalah **jantung logika backend**. Dia mengorkestrasikan semua langkah: validasi → baca config → panggil AI → parse hasil.

#### Struct Data

```rust
// Struct ini adalah "cetakan" untuk 1 buah sampah yang terdeteksi
// Harus IDENTIK dengan interface AIItem di App.vue (TypeScript)
pub struct AIItem {
    pub jenis_sampah: String,  // ← "Kardus", "Botol Bir Kecil", dst.
    pub berat_jumlah: String,  // ← "10", "8", dst.
    pub estimasi_harga: String,// ← "Rp 200", "Rp 5000", dst.
    pub keterangan: String,    // ← "kondisi bersih, ±250 gram"
}
```

#### Fungsi `analisa_image` (Command Utama)

```
Masuk: AIImageInput { photobase64: String }
  │
  ├── [1] check_size_image() ← Tolak jika > 10MB
  │
  ├── [2] get_ai_rules()     ← Baca ai_config.toml (embedded saat compile)
  │         └── system_prompt (daftar harga + panduan)
  │         └── rule.instruction (format output + perhitungan)
  │
  ├── [3] analyze_image_with_hf() ← Panggil API Hugging Face
  │         └── Terima raw text (teks mentah dari AI)
  │
  └── [4] Parser Loop ← Urai teks per-baris jadi Vec<AIItem>
```

#### Parser Text-to-Struct (Baris 61-95)

Ini adalah bagian yang paling kritis dan paling rapuh. Parser membaca teks AI **baris per baris** dan memetakan baris yang diawali kata kunci tertentu ke field `AIItem`:

```
Teks Masuk dari AI:
┌────────────────────────────────────────────────────────┐
│ Jenis Sampah  : Botol Bir Kecil                        │ → jenis_sampah
│ Jumlah Sampah : 8                                      │ → berat_jumlah
│ Perhitungan   : 8 x (250/1000) x 100 = 200            │ → DIABAIKAN ✓
│ Harga         : Rp 200                                 │ → estimasi_harga
│ Keterangan    : kondisi bersih, ±250 gram              │ → keterangan
└────────────────────────────────────────────────────────┘
```

> ⚠️ **Kelemahan Desain Saat Ini:** Parser tidak bisa menangani baris `Keterangan` yang multi-baris. Jika model AI memecah keterangan menjadi beberapa baris, hanya baris pertama yang ikut `keterangan :` yang akan tersimpan.

---

### 3.4 `src-tauri/src/api.rs` — Penghubung ke Dunia Luar

File ini bertanggung jawab atas **satu hal saja**: komunikasi dengan API Hugging Face. Ini adalah implementasi prinsip 1 function 1 job yang EnBee tekankan.

#### Fungsi `get_hf_token()` — Strategi Dual-Layer Token

```
Coba baca dari Environment Variable "HF_Token"
        │
        ├── BERHASIL → kembalikan token (cocok untuk development desktop)
        │
        └── GAGAL → baca dari .env yang di-embed saat compile
                    (cocok untuk Android, karena Android tidak punya
                     akses filesystem runtime ke project folder)
```

Teknik `include_str!("../../.env")` membakar isi file `.env` langsung ke dalam *binary* aplikasi saat proses `cargo build`. Artinya **token tetap terbawa** meski aplikasi diinstall di HP orang lain.

> ⚠️ **Catatan Keamanan:** Ini aman untuk penggunaan internal/pengembangan. Untuk production publik, pertimbangkan menggunakan server proxy agar token HF tidak bisa diekstrak dari APK.

#### Fungsi `analyze_image_with_hf()` — HTTP Call ke AI

```rust
// Payload yang dikirim ke Hugging Face (format OpenAI-compatible)
{
  "model": "Qwen/Qwen3-VL-235B-A22B-Instruct",
  "messages": [
    { "role": "system", "content": system_prompt },  // ← konteks & daftar harga
    { "role": "user", "content": [
      { "type": "text", "text": user_prompt },        // ← instruksi format output
      { "type": "image_url", "image_url": { "url": "data:image/jpeg;base64,..." } }
    ]}
  ],
  "max_tokens": 1024
}
```

Setelah respons datang, parser JSON mengambil isi teks dari `choices[0].message.content`.

---

### 3.5 `src-tauri/src/ai_config.toml` — Otak Strategis AI

File ini adalah **jiwa dari akurasi bot**. Ini bukan sekedar konfigurasi biasa—ini adalah instruksi penuh yang dikirimkan ke model AI setiap kali ada analisa gambar.

```
ai_config.toml terdiri dari 3 bagian besar:
┌─────────────────────────────────────────────────────────────┐
│ [prompt].system                                             │
│   → Daftar harga 50+ jenis sampah per kg                   │
│   → Tabel referensi berat satuan per jenis                  │
│   → Catatan penting dan pengecualian                        │
├─────────────────────────────────────────────────────────────┤
│ [[rules]] name="default"                                    │
│   → Format output wajib (5 field per item)                  │
│   → 11 aturan mutlak (termasuk matematika scratchpad)       │
│   → Contoh output ideal                                     │
├─────────────────────────────────────────────────────────────┤
│ [filters]                                                   │
│   → Format gambar yang diizinkan                            │
└─────────────────────────────────────────────────────────────┘
```

Trik paling penting di dalam file ini adalah **baris "Perhitungan"** — sebuah teknik *Prompt Engineering* yang disebut **Scratchpad (Chain-of-Thought)**:

```
TANPA Scratchpad (❌ AI berhalusinasi):
  Harga: Rp 1000   ← AI tebaक langsung, angka tidak akurat

DENGAN Scratchpad (✓ AI berpikir dahulu):
  Perhitungan : 8 x (250/1000) x 100 = 200
  Harga       : Rp 200   ← AI mengisi berdasarkan hasil kerja pikirannya
```

Baris `Perhitungan` kemudian **dibuang secara diam-diam** oleh parser di `ai_services.rs` dan tidak pernah muncul di layar pengguna.

---

## 🧩 4. Diagram Komponen Lengkap

```
┌──────────────────────────────────────────────────────────────────────┐
│                         EcoScan Application                          │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │                     LAYER FRONTEND (Vue 3)                   │    │
│  │                                                              │    │
│  │   ┌─────────────┐   ┌──────────────┐   ┌─────────────────┐  │    │
│  │   │ Camera Input│   │ Image Preview│   │  Result Cards   │  │    │
│  │   │ <input>     │──►│ <img :src>   │   │ v-for AIItem[]  │  │    │
│  │   │ capture=env │   │              │   │ Harga (green)   │  │    │
│  │   └──────┬──────┘   └──────────────┘   └────────▲────────┘  │    │
│  │          │                                       │           │    │
│  │          │ FileReader.readAsDataURL()            │           │    │
│  │          │ Base64 String                         │           │    │
│  │          ▼                                       │           │    │
│  │   ┌─────────────────────────────────────────────┘           │    │
│  │   │ invoke("analisa_image", { image: { photobase64 } })      │    │
│  │   └───────────────────────────────────────────────┐         │    │
│  │                                                   ▼         │    │
│  └───────────────────────────────────────────────────▼─────────┘    │
│                               TAURI IPC BRIDGE                       │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │                    LAYER BACKEND (Rust)                        │  │
│  │                                                                │  │
│  │  lib.rs                                                        │  │
│  │  └── register: analisa_image                                   │  │
│  │                     │                                          │  │
│  │  ai_services.rs ◄───┘                                          │  │
│  │  ├── check_size_image()  ← Validasi ukuran gambar              │  │
│  │  ├── get_ai_rules()      ← Baca ai_config.toml (compile-time)  │  │
│  │  ├── [panggil api.rs]    ← Dapatkan teks mentah AI             │  │
│  │  └── [Parser Loop]       ← Urai teks → Vec<AIItem>             │  │
│  │                                                                │  │
│  │  api.rs                                                        │  │
│  │  ├── get_hf_token()       ← Env var → fallback .env embedded   │  │
│  │  └── analyze_image_with_hf() ← HTTP POST ke HuggingFace       │  │
│  │                                                                │  │
│  │  ai_config.toml (embedded via include_str!)                    │  │
│  │  ├── [prompt].system   ← Daftar harga & referensi berat        │  │
│  │  └── [[rules]] default ← Format output + 11 aturan            │  │
│  └────────────────────────────────────────────────────────────────┘  │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

---

## ⚗️ 5. Alur Error Handling

Proyek ini memiliki 3 lapisan penanganan error:

```
Layer 1 — Rust (ai_services.rs):
  check_size_image() → false → return Err("Ukuran gambar terlalu besar")

Layer 2 — Rust (api.rs):
  token kosong        → return Err("HF_Token tidak ditemukan")
  HTTP error 4xx/5xx  → return Err("HF API Error: {detail}")
  JSON parse gagal    → return Err("Gagal parsing JSON")
  
Layer 3 — Vue (App.vue):
  catch (error) → errorMessage.value = `Gagal menganalisis: ${error}`
               → Ditampilkan di kartu merah di layar
               
Fallback Internal — ai_services.rs:
  Jika parser tidak bisa menghasilkan 1 item pun (Vec kosong)
  → Kembalikan 1 AIItem dengan keterangan = seluruh teks mentah AI
  → Pengguna masih bisa membaca respons AI walau format-nya kacau
```

---

## 🤔 6. Mengapa Prompt-Based (Bukan Deep Learning)?

Ini pertanyaan yang sering muncul. EnBee sudah menjawabnya di `CatatanPengembang.md`, dan di sini saya tambahkan perspektif teknisnya:

| Aspek | Prompt-Based (Proyek Ini) | Fine-Tuned Deep Learning |
|---|---|---|
| **Kemampuan Nalar** | ✅ Sangat tinggi (bisa memperkirakan berat tak dikenal) | ❌ Terbatas pada kategori training |
| **Data Training** | ✅ Tidak perlu | ❌ Butuh ribuan gambar berlabel |
| **Biaya Awal** | ✅ Gratis (HF Serverless) | ❌ Mahal (GPU cloud) |
| **Output Variatif** | ✅ Narasi natural | ❌ Kaku (categorical) |
| **Akurasi Matematis** | ⚠️ Butuh teknik Scratchpad | ✅ Hasil kalkulasi stabil |
| **Kecepatan Respons** | ⚠️ ~3-8 detik per gambar | ✅ < 1 detik |
| **Biaya Per-Gambar** | ⚠️ ~$0.002/gambar (limit reset per bulan) | ✅ Tergantung hosting |

Untuk skala aplikasi kasir daur ulang yang tidak memproses ribuan gambar per menit, pendekatan Prompt-Based ini **lebih dari cukup** dan jauh lebih cepat untuk dikembangkan.

---

## 📋 7. Ringkasan Aliran Data (Data Flow)

```
📸 Foto Sampah (JPEG/PNG)
        │
        ▼ (FileReader di Vue 3)
🔤 Base64 String "data:image/jpeg;base64,/9j/4AAQ..."
        │
        ▼ (Tauri IPC invoke)
🦀 Rust menerima → validasi ukuran → baca config TOML
        │
        ▼ (HTTP POST via reqwest)
🤖 Qwen3-VL-235B-A22B di Hugging Face Cloud ~3-8 detik
        │
        ▼ (Raw text response)
📝 Teks terstruktur mentah:
   "Jenis Sampah  : Kardus\nJumlah Sampah : 10\nPerhitungan : ...\nHarga : Rp 2000\nKeterangan : ..."
        │
        ▼ (Parser loop di ai_services.rs)
📦 Vec<AIItem> = [
     AIItem { jenis: "Kardus", jumlah: "10", harga: "Rp 2000", ket: "kondisi baik, ±350 gram" }
   ]
        │
        ▼ (Tauri IPC return)
🖥️ Vue 3 render kartu hasil di layar Android
```

---

*— Antigravity AI, April 2026*
*Dokumen ini adalah pendamping dari [`CatatanPengembang.md`](./CatatanPengembang.md)*
