# ScanTrash Backend API Documentation

Dokumentasi lengkap untuk Frontend Team - Semua command/invoke yang perlu digunakan untuk integrasi dengan backend Rust Tauri.

**Stack:** Rust + Tauri v2 + Supabase PostgreSQL

## Sebelum Memulai Buat .env dulu yagesya
buat di Scantrash/.env
``` bash
SUPABASE_URL=url di group
SUPABASE_KEY=kunci juga di group
SESSION_SECRET_KEY=Sc4nTr4sh_S3cur3_K3y_2026_Atau_Apapun_Bebas
HF_Token=hf_tokenhuggingfaceawdfaslkfe
```

---

## 📋 Quick Reference - Semua Commands

| Command | Fungsi | Auth Required | Return Type |
|---------|--------|---------------|------------|
| `get_google_auth_url_command` | Dapatkan URL login Google | ❌ Tidak | `String` (OAuth URL) |
| `check_auth_status_command` | Cek user sudah login atau tidak | ❌ Tidak | `bool` |
| `get_profile_command` | Ambil profile user yang login | ✅ Ya | `Profile` (single object) |
| `logout_command` | Logout user | ✅ Ya | `()` |
| `get_pricelist_command` | Ambil daftar harga sampah | ✅ Ya | `Vec<Pricelist>` (array) |
| `create_log_command` | Log aktivitas ke Supabase (audit trail) | ✅ Ya | `()` |
| `write_local_log_command` | Log aktivitas lokal (dengan timestamp otomatis) | ❌ Tidak | `()` |
| `read_local_log_command` | Baca isi file log lokal dari HP | ❌ Tidak | `String` |
| `scan_trash` | Pindai sampah via AI | ✅ Ya | `Vec<ScanResult>` (array) |
| `get_balance_command` | Ambil saldo nasabah terkini | ✅ Ya | `i64` (number) |
| `get_savings_history_command` | Ambil riwayat transaksi nasabah | ✅ Ya | `Vec<TransactionItem>` (array) |
| `get_schedules_command` | Ambil jadwal setor (buka/tutup) | ✅ Ya | `Vec<Schedule>` (array) |
| `get_nasabah_list_command` | Daftar nasabah + saldo ⭐ Admin | ✅ Ya | `Vec<NasabahItem>` |
| `get_admin_dashboard_command` | Data dashboard admin ⭐ Admin | ✅ Ya | `AdminDashboard` |
| `create_setoran_command` | Setor saldo ke nasabah ⭐ Admin | ✅ Ya | `()` |
| `create_penarikan_command` | Tarik saldo dari nasabah ⭐ Admin | ✅ Ya | `()` |
| `create_schedule_command` | Tambah jadwal setor ⭐ Admin | ✅ Ya | `()` |
| `update_schedule_command` | Edit jadwal setor ⭐ Admin | ✅ Ya | `()` |
| `delete_schedule_command` | Hapus jadwal setor ⭐ Admin | ✅ Ya | `()` |
| `create_pricelist_command` | Tambah item katalog harga + upload gambar ⭐ Admin | ✅ Ya | `()` |
| `update_pricelist_command` | Edit item katalog harga ⭐ Admin | ✅ Ya | `()` |
| `delete_pricelist_command` | Hapus item katalog harga + gambar ⭐ Admin | ✅ Ya | `()` |

---

## 🔐 Authentication Flow (Alur Lengkap)

### Flow Diagram
```
┌─────────────────────────────────────────────────────────────────┐
│                      APP START                                  │
│                    (App.vue mounted)                            │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│  1. check_auth_status_command() - Cek sudah ada token?          │
│     (Sebelum render apapun, check dulu)                         │
└────────────────────────┬────────────────────────────────────────┘
                         │
                    ┌────┴─────┐
                    │           │
              YA (true)      TIDAK (false)
                    │           │
        ┌───────────▼───┐     ┌──▼───────────────┐
        │ 2. get_profile|     │Redirect ke /login│
        │    _command() |     │                  │
        └───────────┬───┘     └──────────────────┘
                    │
                    ▼
        ┌──────────────────────────┐
        │ Profile Loaded! Simpan   │
        │ ke store (user data)     │
        └──────────────┬───────────┘
                       │
                       ▼
        ┌──────────────────────────┐
        │ Redirect ke Dashboard    │
        │ (berdasarkan role)       │
        └──────────────────────────┘

┌──────────────────────────────────────────────────────────────┐
│              USER KLIK LOGIN WITH GOOGLE BUTTON              │
├──────────────────────────────────────────────────────────────┤
│ 1. invoke('get_google_auth_url_command')                     │
│    → Dapat URL: https://...                                  │
│ 2. Buka URL (browser/webview)                                │
│ 3. User login & authorize                                    │
│ 4. Google redirect ke deep link:                             |
| com.users.scantrash://auth?access_token=xxx&refresh_token=yyy│
│ 5. Backend intercept → tukar code dgn token → simpan token   │
│ 6. Emit event 'login-success'                                │
│ 7. Frontend listen event → call get_profile_command()        │
└──────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────┐
│              USER KLIK LOGOUT BUTTON                         │
├──────────────────────────────────────────────────────────────┤
│ 1. invoke('logout_command')                                  │
│    → Delete token dari storage                               │
│    → Hapus profile dari store                                │
│ 2. Redirect ke /login                                        │
└──────────────────────────────────────────────────────────────┘
```

---

## 🔧 Command Details

### 1️⃣ `get_google_auth_url_command`

**Fungsi:** Mendapatkan URL untuk login dengan Google OAuth

**Parameter:** Tidak ada

**Return Type:** `String`

**Response:** URL OAuth Google yang siap dibuka di browser
```
https://accounts.google.com/o/oauth2/v2/auth?client_id=...&redirect_uri=...&scope=...
```

**Kapan digunakan:** Ketika user klik tombol "Login dengan Google"

**Code Example:**
```typescript
import { invoke } from '@tauri-apps/api/core';

async function handleGoogleLogin() {
  try {
    const authUrl = await invoke<string>('get_google_auth_url_command');
    console.log("Auth URL:", authUrl);
    
    // Buka URL di browser/webview
    window.location.href = authUrl;
    
    // ATAU buka di webview baru
    // const webview = new WebviewWindow('auth', { url: authUrl });
  } catch (error) {
    console.error("Error mendapatkan auth URL:", error);
    showErrorToast("Gagal memulai login Google");
  }
}
```

**Error Handling:**
- Jika error: `"Failed to get Google auth URL"` → Cek koneksi internet

---

### 2️⃣ `check_auth_status_command`

**Fungsi:** Cek apakah user sudah login (ada token valid)

**Parameter:** Tidak ada (token diambil otomatis dari storage)

**Return Type:** `bool`
- `true` = User sudah login, ada token valid
- `false` = User belum login atau token sudah expired

**Kapan digunakan:**
1. Saat app startup (App.vue mounted) untuk check status
2. Sebelum akses halaman yang butuh auth

**Code Example:**
```typescript
import { invoke } from '@tauri-apps/api/core';

async function checkUserAuth() {
  try {
    const isLoggedIn = await invoke<boolean>('check_auth_status_command');
    
    if (isLoggedIn) {
      console.log("✅ User sudah login");
      // Lanjut ke dashboard
    } else {
      console.log("❌ User belum login");
      // Redirect ke login page
      router.push('/login');
    }
  } catch (error) {
    console.error("Error checking auth:", error);
    router.push('/login');
  }
}
```

**Features:**
- ✅ Auto-refresh token jika sudah expired (backend handle)
- ✅ Return true jika refresh berhasil
- ✅ Return false jika refresh gagal (user harus login ulang)

---

### 3️⃣ `get_profile_command`

**Fungsi:** Ambil data profile user yang sedang login

**Parameter:** Tidak ada (user_id diambil dari token)

**Return Type:** `Profile` (single object, bukan array)

**Response Type:**
```typescript
interface Profile {
  user_id: string;         // UUID dari Supabase auth
  email: string;           // Email user
  username: string;        // Username
  photo_url?: string;      // URL foto profil dari Google
  role: string;            // "user" atau "admin"
  status?: string;         // "active", "inactive", dll
  created_at?: string;     // ISO datetime
}
```

**Kapan digunakan:**
- Setelah login berhasil (di event 'login-success')
- Setelah check_auth_status_command return true
- Ketika page profile atau dashboard dimuat

**Code Example:**
```typescript
import { invoke } from '@tauri-apps/api/core';

interface Profile {
  user_id: string;
  email: string;
  username: string;
  photo_url?: string;
  role: string;
  status?: string;
  created_at?: string;
}

async function loadUserProfile() {
  try {
    // ✅ Return SINGLE OBJECT, bukan array
    const profile = await invoke<Profile>('get_profile_command');
    
    console.log("User:", profile.username);
    console.log("Role:", profile.role);
    console.log("Email:", profile.email);
    
    // Simpan ke Pinia store
    userStore.setProfile(profile);
    
    // Role-based routing
    if (profile.role === 'admin') {
      router.push('/admin/dashboard');
    } else {
      router.push('/user/dashboard');
    }
  } catch (error) {
    console.error("Error loading profile:", error);
    showErrorToast("Gagal memuat profil");
    // Fallback: logout
    await invoke('logout_command');
    router.push('/login');
  }
}
```

**Important Notes:**
- ⚠️ Backend otomatis filter berdasarkan `user_id` dari token
- ⚠️ Jangan kirim user_id sebagai parameter
- ⚠️ Return adalah SINGLE OBJECT, bukan array (langsung akses `profile.username`)
- ✅ Error jika user belum login (check_auth_status dulu)

---

### 4️⃣ `logout_command`

**Fungsi:** Logout user - hapus token dari storage

**Parameter:** Tidak ada

**Return Type:** `()` (void/unit type)

**Kapan digunakan:** Ketika user klik tombol logout

**Code Example:**
```typescript
import { invoke } from '@tauri-apps/api/core';

async function handleLogout() {
  try {
    // Call backend logout
    await invoke('logout_command');
    console.log("✅ Logged out successfully");
    
    // Hapus profile dari store
    userStore.clearProfile();
    
    // Redirect ke login
    router.push('/login');
    
    // Show toast
    showSuccessToast("Logout berhasil");
  } catch (error) {
    console.error("Error logout:", error);
    showErrorToast("Gagal logout");
  }
}
```

**Note:**
- Tidak perlu cek auth (backend akan handle error jika tidak login)
- Setelah logout, akses ke protected route akan redirect ke login

---

### 5️⃣ `get_pricelist_command`

**Fungsi:** Ambil daftar harga sampah untuk scanning

**Parameter:** Tidak ada

**Return Type:** `Vec<Pricelist>` (array of pricelist items)

**Response Type:**
```typescript
interface PricelistItem {
  id?: string;             // UUID
  labels: string;          // Jenis sampah (Plastik, Kertas, dll)
  price: number;           // Harga per kg (angka)
  img_url?: string;        // URL gambar icon sampah
  created_at?: string;     // ISO datetime
}
```

**Kapan digunakan:**
- Saat halaman scanning/pricing dimuat
- Ketika user perlu lihat daftar harga
- Cache di store agar tidak request berulang

**Code Example:**
```typescript
import { invoke } from '@tauri-apps/api/core';

interface PricelistItem {
  id?: string;
  labels: string;
  price: number;
  img_url?: string;
  created_at?: string;
}

async function loadPricelist() {
  try {
    // Return ARRAY of pricelists
    const pricelists = await invoke<PricelistItem[]>('get_pricelist_command');
    
    console.log("Pricelist items:", pricelists.length);
    
    // Simpan ke store untuk reusable
    pricelistStore.setPricelists(pricelists);
    
    // Display di UI
    pricelists.forEach(item => {
      console.log(`${item.labels}: Rp${item.price}/kg`);
      if (item.img_url) {
        console.log("Gambar icon tersedia di:", item.img_url);
      }
    });
  } catch (error) {
    console.error("Error loading pricelist:", error);
    showErrorToast("Gagal memuat daftar harga");
  }
}
```

**Best Practice:**
- ✅ Cache hasil di store agar tidak request berulang
- ✅ Refresh setiap 1 jam atau saat user klik "Refresh"
- ✅ Show loading spinner saat fetch
- ✅ Handle error dengan graceful

---

### 6️⃣ `create_log_command`

**Fungsi:** 
Mencatat aktivitas kritikal, transaksi, keuangan, dan hasil AI ke database Supabase (Audit Trail).

**Parameter:**
```typescript
{
  level: string,   // "INFO" | "WARNING" | "ERROR"
  message: string  // Deskripsi detail aktivitas
}
```
**Return Type ```()``` (void/unit type)**

**Kapan digunakan (Wajib ke supabase):**
- 🟢 User berhasil mendapat saldo atau melakukan penarikan.
- 🟢 AI berhasil melakukan scan dan mengidentifikasi sampah.
- 🔴 Gagal melakukan transaksi keuangan atau error pada server AI.
- 🟡 Admin mengubah data penting (seperti harga di pricelist).
- ❌ PENTING: Jangan gunakan command ini untuk log ringan seperti klik tombol, pindah halaman, atau putus internet. (Gunakan write_local_log_command untuk error ringan)

**Code Example:**
```Typesecript
import { invoke } from '@tauri-apps/api/core';

// 1. Contoh: Log saat scan AI berhasil dan saldo bertambah
async function logTransaksiScan(jenisSampah: string, nominal: number) {
  try {
    await invoke('create_log_command', {
      level: 'INFO',
      message: `Transaksi Sukses: AI mendeteksi ${jenisSampah}, Saldo bertambah Rp${nominal}`
    });
    console.log("✅ Log transaksi tersimpan di Supabase");
  } catch (error) {
    // Silent fail agar tidak mengganggu UX user
    console.error("Error logging ke Supabase:", error);
  }
}

// 2. Contoh: Log saat sistem AI gagal (Error Kritis)
async function logErrorAI(errorMsg: string) {
  try {
    await invoke('create_log_command', {
      level: 'ERROR',
      message: `Sistem AI Gagal memproses gambar: ${errorMsg}`
    });
  } catch (error) {
    console.error("Error logging ke Supabase:", error);
  }
}
````

**Important Notes:**
- ✅ Backend sudah otomatis menyisipkan user_id dari token sesi yang aktif.
- ✅ Backend sudah otomatis membuatkan ID dan Timestamp (created_at).
- ⚠️ Gunakan command ini dengan bijak (hanya untuk transaksi/data penting) agar tidak menghabiskan kuota row database Supabase.

---

### 7️⃣ `write_local_log_command`
**Kapan digunakan:**
- 🔵 Navigasi & UI: User berpindah ke halaman Kamera, membuka halaman Profil, atau menekan tombol tertentu.

- 🟡 Peringatan Sistem: User menolak memberikan izin akses kamera (Permission Denied), atau aplikasi berjalan lambat.

- 🔴 Error Non-Kritis (Jaringan): Gagal mengambil data pricelist karena internet putus, atau gambar gagal dimuat.

- ❌ PENTING: Jangan gunakan ini untuk mencatat penambahan saldo, perubahan harga, atau hasil tebakan AI. (Gunakan create_log_command untuk urusan uang/data penting).

> [!NOTE]
> **Timestamp Otomatis:** Backend sudah otomatis menambahkan timestamp ke setiap baris log dalam format `[YYYY-MM-DD HH:MM:SS]`. Frontend **tidak perlu** mengirimkan waktu. Format log di file: `[2026-04-28 00:22:41] [INFO] pesan kamu`.

**Code Example:**
```typescript
import { invoke } from '@tauri-apps/api/core';

async function logLokal() {
  await invoke('write_local_log_command', {
    level: 'WARNING',
    message: 'Koneksi internet user putus saat memuat gambar'
  });
}
```

---

### 7b️⃣ `read_local_log_command`

**Fungsi:** Membaca seluruh isi file log lokal (`scantrash_local.log`) yang tersimpan di memori internal HP.

**Parameter:** Tidak ada

**Return Type:** `string` (seluruh isi log sebagai satu string panjang)

**Kapan digunakan:**
- Untuk halaman debug / admin tools yang menampilkan riwayat log.
- Untuk mengambil log dan mengirimkannya ke server saat user melaporkan bug.
- Untuk keperluan developer melihat log langsung dari dalam aplikasi.

**Code Example:**
```typescript
import { invoke } from '@tauri-apps/api/core';

async function tampilkanLog() {
  try {
    const isiLog = await invoke<string>('read_local_log_command');
    
    if (!isiLog) {
      console.log('File log masih kosong.');
      return;
    }

    // Pisah per baris untuk ditampilkan di UI
    const baris = isiLog.split('\n').filter(b => b.trim() !== '');
    console.log(`Total ${baris.length} baris log.`);
    console.log(baris);
    
  } catch (error) {
    console.error('Gagal membaca log lokal:', error);
  }
}
```

**Cara Akses Log dari Luar Aplikasi (via ADB):**
```powershell
# Baca langsung di terminal
& "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe" shell "run-as com.users.scantrash cat scantrash_local.log"

# Download ke file .txt di komputer
& "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe" shell "run-as com.users.scantrash cat scantrash_local.log" > log_hp_saya.txt
```

> [!IMPORTANT]
> File log disimpan di direktori privat Android (`app_local_data_dir()`). File ini hanya dapat diakses oleh proses aplikasi itu sendiri. Di luar ADB (mode debug), file ini tidak dapat dibaca oleh aplikasi lain di HP.

### 8️⃣ `scan_trash`
Command ini adalah jantung utama dari aplikasi ScanTrash. Fungsinya adalah menerima gambar dari kamera HP, mengirimkannya ke AI Hugging Face untuk dianalisis, menyimpan riwayatnya ke database Supabase, dan mengembalikan hasil perhitungannya ke layar HP.

> [!IMPORTANT]
> **Perubahan Keamanan (Update Terbaru):** Pesan error yang dikembalikan ke frontend sekarang **bersifat generik** untuk melindungi infrastruktur server. Kamu tidak akan lagi melihat stack trace atau URL Supabase di pesan error. Detail error teknis hanya tersimpan di `scantrash_local.log` di HP.

**Alur Kerja:**
1. Vue (Frontend) mengambil foto berformat Base64 (JPEG/PNG/WEBP).
2. Vue memanggil `invoke("scan_trash", { image: foto_base64 })`.
3. Backend memvalidasi ukuran gambar (max ~5MB / 7.000.000 karakter). Jika terlalu besar, langsung return error.
4. Backend memvalidasi format prefix Base64 (hanya `jpeg`, `png`, `webp` yang diizinkan).
5. Rust (Backend) otomatis mengambil JWT Token user yang sedang login dari Brankas (AppState).
6. Rust mengirimnya ke AI beserta instruksi harga dari Supabase.
7. AI merespons, Rust memecah datanya menjadi Array (mendukung banyak objek sekaligus).
8. Rust menembak data tersebut ke Supabase tabel scan (menyimpan ke database).
9. Rust mengembalikan Array tersebut ke Vue untuk ditampilkan di UI.

**Struktur Datanya**
```typescript
// Apa yang akan kamu terima dari Rust
export interface ScanResult {
  trash_type: string;      // Contoh: "Kardus"
  label_id: string;        // Contoh: "kardus" (lowercase_dengan_underscore)
  material_info: string;   // Contoh: "Jumlah/Berat: 1"
  kondisi: string;         // Contoh: "Kardus kotak lampu LED, kondisi utuh..."
  kebersihan: string;      // Contoh: "Sesuai deteksi visual"
  estimasi_harga: number;  // Contoh: 175 (Angka bulat, bukan teks Rp)
}
```

**Cara menggunakannya:**
```typescript
import { invoke } from "@tauri-apps/api/core";

// 1. Siapkan variabel gambar (JPEG/PNG/WEBP Base64)
const imageBase64 = "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQ...";

// 2. Tembak ke Rust
try {
  const hasilScan = await invoke<ScanResult[]>("scan_trash", { 
    image: imageBase64 
  });
  
  // 3. Sukses! hasilScan adalah Array.
  console.log(`Ada ${hasilScan.length} sampah yang terdeteksi!`);
  console.log("Total Harga Objek Pertama:", hasilScan[0].estimasi_harga);

} catch (error) {
  // 4. Gagal! Pesan error yang diterima bersifat generik (tidak ada URL/stack trace)
  // Contoh pesan error yang mungkin muncul:
  // "Payload gambar terlalu besar. Maksimal ~5MB."
  // "Format gambar tidak diizinkan. Gunakan JPG, PNG, atau WEBP."
  // "Koneksi gagal. Pastikan internet Anda aktif."
  // "Gagal menyimpan data scan ke sistem."
  // "Sesi habis atau user belum login!"
  console.error("Gagal Scan:", error);
  showErrorToast(String(error));
}
```

**Error yang Mungkin Dikembalikan ke Frontend:**
| Pesan Error | Penyebab |
|---|---|
| `"Payload gambar terlalu besar. Maksimal ~5MB."` | Gambar base64 >7juta karakter |
| `"Format gambar tidak diizinkan. Gunakan JPG, PNG, atau WEBP."` | Format gambar bukan jpeg/png/webp |
| `"Koneksi gagal. Pastikan internet Anda aktif."` | Gagal ambil rules/pricelist dari Supabase |
| `"Gagal terhubung ke layanan AI."` | Koneksi ke Hugging Face gagal/timeout |
| `"Layanan AI sedang mengalami gangguan."` | HF API error (4xx/5xx) |
| `"Gagal menyimpan data scan ke sistem."` | Insert ke DB Supabase gagal |
| `"Sesi habis atau user belum login!"` | Token tidak ada di AppState |
| `"Data kosong, AI gagal membaca sampah."` | AI tidak mengenali objek sama sekali |

---

### 9️⃣ `get_balance_command`
**Fungsi:** Mengambil saldo terkini. 
- Karena fitur keamanan baru (RLS Fallback), parameter ini **wajib diisi** dengan ID user.
- User biasa wajib mengirimkan `user_id` miliknya sendiri (didapat dari `profile`). Backend akan memvalidasi apakah ID yang diminta sesuai dengan sesi.
- Admin bisa mengirimkan `targetUserId` milik nasabah mana pun untuk melihat saldonya.

**Parameter:**
```typescript
{
  targetUserId: string // WAJIB. Isi dengan user_id dari profil (user biasa), atau UUID spesifik (untuk Admin melihat nasabah).
}
```

**Return Type:** `i64` (number)

**Code Example:**
```typescript
import { invoke } from "@tauri-apps/api/core";
import { useAuthStore } from "@/stores/authStore";

// 1. Contoh Nasabah Biasa
async function cekSaldoSendiri() {
  try {
    const authStore = useAuthStore();
    const myId = authStore.profile?.user_id;
    if (!myId) throw new Error("Profil belum dimuat!");

    const saldo = await invoke<number>("get_balance_command", { targetUserId: myId });
    console.log("Saldo Terkini:", saldo);
  } catch (error) {
    console.error("Gagal ambil saldo:", error);
  }
}

// 2. Contoh Admin Melihat Saldo Nasabah
async function cekSaldoNasabah(uid: string) {
  try {
    const saldo = await invoke<number>("get_balance_command", { targetUserId: uid });
    console.log("Saldo Nasabah:", saldo);
  } catch (error) {
    console.error("Gagal ambil saldo nasabah:", error);
  }
}
```

---

### 🔟 `get_savings_history_command`
**Fungsi:** Mengambil 50 riwayat transaksi terbaru dari tabel `savings`. Mendukung penggunaan oleh User (melihat riwayatnya sendiri) maupun Admin (melihat riwayat nasabah tertentu). Parameter `targetUserId` **wajib** diisi.
Kalkulasi (menentukan income/expense dan menghitung nominal) sudah **dilakukan oleh backend Rust** sehingga frontend bisa langsung memakainya.

**Parameter:**
```typescript
{
  targetUserId: string // WAJIB. Isi dengan user_id dari profil, atau UUID spesifik (untuk Admin).
}
```

**Return Type:** `Vec<TransactionItem>` (Array)

**Response Type:**
```typescript
interface TransactionItem {
  id: string;
  name: string;          // Keterangan atau label default ("Setoran" / "Penarikan")
  date: string;          // Waktu transaksi (ISO format, frontend cukup ubah ke string format)
  nominal: number;       // Selisih absolut (sudah dihitung backend)
  transaction_type: "income" | "expense"; // Tipe transaksi (ditentukan backend)
}
```

**Code Example:**
```typescript
import { invoke } from "@tauri-apps/api/core";
import { useAuthStore } from "@/stores/authStore";

// 1. User melihat riwayatnya sendiri
const authStore = useAuthStore();
const myId = authStore.profile?.user_id;
const riwayatku = await invoke<TransactionItem[]>("get_savings_history_command", { targetUserId: myId });

// 2. Admin melihat riwayat nasabah spesifik
const riwayatNasabah = await invoke<TransactionItem[]>("get_savings_history_command", { targetUserId: "uuid-nasabah-tersebut" });
```

---

### 1️⃣1️⃣ `get_schedules_command`
**Fungsi:** Mengambil daftar jadwal buka/tutup lapak (tabel `tanggal`).

**Parameter:** Tidak ada

**Return Type:** `Vec<Schedule>` (Array)

**Response Type:**
```typescript
interface ScheduleItem {
  id_tanggal?: string;
  waktu_buka: string;     // Contoh: "09:00:00+00"
  lokasi: string;         // Lokasi lapak
  waktu_tutup: string;    // Contoh: "16:00:00+00"
  tanggal: string;        // Contoh: "2026-05-10"
}
```

**Code Example:**
```typescript
import { invoke } from "@tauri-apps/api/core";

async function ambilJadwal() {
  try {
    const jadwal = await invoke<ScheduleItem[]>("get_schedules_command");
    if (jadwal.length > 0) {
      console.log("Jadwal Terdekat:", jadwal[0].tanggal);
    }
  } catch (error) {
    console.error("Gagal memuat jadwal:", error);
  }
}
```

---

### 1️⃣2️⃣ `get_nasabah_list_command` ⭐ Admin Only

**Fungsi:** Mengambil daftar semua nasabah (role `users`) beserta saldo terkini masing-masing. Mendukung pencarian berdasarkan username. **Hanya bisa diakses oleh admin.**

**Parameter:**
```typescript
{
  keyword: string  // Kata kunci pencarian username (case-insensitive, partial match).
                   // Kirim string kosong "" untuk mengambil semua nasabah.
}
```

**Return Type:** `NasabahItem[]` (Array)

**Response Type:**
```typescript
interface NasabahItem {
  user_id: string;       // UUID nasabah
  username: string;      // Nama nasabah
  email: string;         // Email (ditampilkan sebagai info kontak)
  photo_url?: string;    // URL foto profil (opsional)
  saldo: number;         // Saldo terkini dalam Rupiah (0 jika belum ada transaksi)
}
```

**Kapan digunakan:**
- Halaman admin untuk melihat & mencari daftar nasabah
- Sebelum admin memilih nasabah untuk melihat detail / riwayat transaksi

**Code Example:**
```typescript
import { invoke } from "@tauri-apps/api/core";

interface NasabahItem {
  user_id: string;
  username: string;
  email: string;
  photo_url?: string;
  saldo: number;
}

// 1. Tampilkan semua nasabah saat halaman dimuat
async function muatSemuaNasabah() {
  try {
    const daftar = await invoke<NasabahItem[]>("get_nasabah_list_command", {
      keyword: "",
    });
    console.log(`Total ${daftar.length} nasabah ditemukan.`);
    // Simpan ke store / tampilkan di UI
    adminStore.setNasabahList(daftar);
  } catch (error) {
    console.error("Gagal memuat daftar nasabah:", error);
    showErrorToast(String(error));
  }
}

// 2. Cari nasabah berdasarkan nama (dipanggil saat input search berubah)
async function cariNasabah(keyword: string) {
  try {
    const hasil = await invoke<NasabahItem[]>("get_nasabah_list_command", {
      keyword: keyword,  // Contoh: "pra" → akan menemukan "Prabowo", "Prasetiyo", dll.
    });
    adminStore.setNasabahList(hasil);
  } catch (error) {
    console.error("Gagal mencari nasabah:", error);
    showErrorToast(String(error));
  }
}

// 3. Setelah memilih nasabah, gunakan user_id-nya untuk cek saldo atau riwayat
async function lihatDetailNasabah(nasabah: NasabahItem) {
  // Gunakan get_balance_command atau get_savings_history_command
  const riwayat = await invoke("get_savings_history_command", {
    targetUserId: nasabah.user_id,
  });
}
```

**Error yang Mungkin Dikembalikan ke Frontend:**
| Pesan Error | Penyebab |
|---|---|
| `"Akses ditolak: Tidak ada sesi aktif."` | User belum login |
| `"Akses ditolak: Hanya admin yang dapat mengakses fitur ini."` | User login tapi bukan admin |
| `"Gagal memuat daftar nasabah. Silakan coba lagi."` | Error network/server |

> [!IMPORTANT]
> Pencarian (`keyword`) dilakukan di **sisi backend** menggunakan filter `ilike` ke Supabase, bukan di frontend. Ini lebih efisien dan aman untuk data besar.

> [!NOTE]
> Jika saldo salah satu nasabah gagal diambil (misalnya jaringan sesaat bermasalah), backend akan default-kan saldo nasabah tersebut ke **0** dan tetap mengembalikan nasabah lain. Seluruh list tidak akan gagal karena satu nasabah bermasalah.

---

### 1️⃣3️⃣ `get_admin_dashboard_command` ⭐ Admin Only

**Fungsi:** Mengambil semua data untuk halaman dashboard admin dalam satu panggilan:
- Total saldo seluruh nasabah
- Jumlah nasabah
- 10 aktivitas transaksi terbaru dari semua nasabah

**Parameter:** Tidak ada (token admin diambil otomatis dari sesi)

**Return Type:** `AdminDashboard` (single object)

**Response Type:**
```typescript
interface AdminDashboard {
  total_saldo: number;           // Total saldo semua nasabah (Rupiah)
  total_nasabah: number;         // Jumlah nasabah (role = 'users')
  aktivitas_terbaru: AktivitasItem[];
}

interface AktivitasItem {
  id: string;
  username: string;             // Nama nasabah pelaku transaksi
  user_id: string;              // UUID nasabah (untuk navigasi ke detail)
  date: string;                 // ISO timestamp
  keterangan: string;           // "Botol Plastik", "Setoran", "Penarikan", dll.
  nominal: number;              // Nilai transaksi (selisih absolut, sudah dihitung backend)
  transaction_type: "income" | "expense";
}
```

**Kapan digunakan:**
- Saat halaman dashboard admin pertama kali dimuat
- Saat admin menekan tombol refresh

**Code Example:**
```typescript
import { invoke } from "@tauri-apps/api/core";

interface AktivitasItem {
  id: string;
  username: string;
  user_id: string;
  date: string;
  keterangan: string;
  nominal: number;
  transaction_type: "income" | "expense";
}

interface AdminDashboard {
  total_saldo: number;
  total_nasabah: number;
  aktivitas_terbaru: AktivitasItem[];
}

async function muatDashboardAdmin() {
  try {
    // Tidak perlu parameter — token admin diambil otomatis
    const dashboard = await invoke<AdminDashboard>("get_admin_dashboard_command");

    console.log("Total Saldo:", dashboard.total_saldo);        // Rp 500.000
    console.log("Total Nasabah:", dashboard.total_nasabah);    // 15
    console.log("Aktivitas:", dashboard.aktivitas_terbaru);    // Array[10]

    // Tampilkan aktivitas terbaru di UI
    dashboard.aktivitas_terbaru.forEach(item => {
      const prefix = item.transaction_type === "income" ? "+" : "-";
      console.log(`${item.username} | ${item.keterangan} | ${prefix}Rp${item.nominal}`);
      // Navigasi ke detail: gunakan item.user_id
    });

  } catch (error) {
    console.error("Gagal memuat dashboard:", error);
    showErrorToast(String(error));
  }
}
```

**Error yang Mungkin Dikembalikan ke Frontend:**
| Pesan Error | Penyebab |
|---|---|
| `"Akses ditolak: Tidak ada sesi aktif."` | Belum login |
| `"Akses ditolak: Hanya admin yang dapat mengakses fitur ini."` | Bukan admin |
| `"Gagal memuat data dashboard. Silakan coba lagi."` | Error network/server |

> [!NOTE]
> Aktivitas terbaru diambil dengan **1 query langsung ke tabel savings** (tanpa filter user_id), diurutkan dari yang paling baru, dibatasi 10 item. Username di-lookup dari cache profil (HashMap) sehingga tidak ada query tambahan per transaksi.

---

### 1️⃣4️⃣ `create_setoran_command` ⭐ Admin Only

**Fungsi:** Menyetor saldo ke rekening nasabah tertentu. Backend otomatis membaca saldo terkini, menambahkannya dengan nominal, lalu menyimpan record baru ke tabel `savings`.

**Parameter:**
```typescript
{
  targetUserId: string  // UUID nasabah yang menerima setoran
  nominal: number       // Jumlah rupiah yang disetor (harus > 0)
  keterangan: string    // Deskripsi setoran, contoh: "Botol Plastik"
}
```

**Return Type:** `()` (void — sukses jika tidak throw error)

**Kapan digunakan:** Saat admin menekan konfirmasi di form setoran

**Code Example:**
```typescript
import { invoke } from "@tauri-apps/api/core";

async function konfirmasiSetoran(nasabahId: string, nominal: number, ket: string) {
  try {
    await invoke("create_setoran_command", {
      targetUserId: nasabahId,
      nominal: nominal,
      keterangan: ket,
    });
    console.log("✅ Setoran berhasil");
    // Refresh dashboard
    await muatDashboard();
  } catch (error) {
    // Error message aman (tidak bocorkan detail teknis)
    showErrorToast(String(error));
  }
}
```

**Error yang Mungkin Dikembalikan:**
| Pesan Error | Penyebab |
|---|---|
| `"Akses ditolak: Tidak ada sesi aktif."` | Belum login |
| `"Akses ditolak: Hanya admin yang dapat..."` | Bukan admin |
| `"Jumlah setoran harus lebih dari Rp0."` | Nominal ≤ 0 |
| `"Nasabah tidak ditemukan."` | targetUserId tidak valid |
| `"Gagal menyimpan setoran ke server."` | Error network/DB |

---

### 1️⃣5️⃣ `create_penarikan_command` ⭐ Admin Only

**Fungsi:** Menarik saldo dari rekening nasabah. Backend otomatis memvalidasi kecukupan saldo sebelum menyimpan transaksi.

**Parameter:**
```typescript
{
  targetUserId: string  // UUID nasabah yang ditarik saldonya
  nominal: number       // Jumlah rupiah yang ditarik (harus > 0 dan ≤ saldo saat ini)
  keterangan: string    // Deskripsi penarikan, contoh: "Pencairan tunai"
}
```

**Return Type:** `()` (void)

> [!IMPORTANT]
> Backend secara otomatis mengambil saldo terkini dan memvalidasi: jika `nominal > saldo`, transaksi **ditolak** dan error message spesifik dikembalikan ke frontend beserta saldo terkini nasabah.

**Code Example:**
```typescript
import { invoke } from "@tauri-apps/api/core";

async function prosesPenarikan(nasabahId: string, nominal: number, ket: string) {
  try {
    await invoke("create_penarikan_command", {
      targetUserId: nasabahId,
      nominal: nominal,
      keterangan: ket,
    });
    showSuccessToast("Penarikan berhasil!");
  } catch (error) {
    // Contoh error: "Saldo nasabah tidak mencukupi. Saldo saat ini: Rp15.000."
    showErrorToast(String(error));
  }
}
```

**Error yang Mungkin Dikembalikan:**
| Pesan Error | Penyebab |
|---|---|
| `"Jumlah penarikan harus lebih dari Rp0."` | Nominal ≤ 0 |
| `"Saldo nasabah tidak mencukupi. Saldo saat ini: Rp{N}."` | Nominal > saldo terkini |
| `"Gagal menyimpan penarikan ke server."` | Error network/DB |

---

### 1️⃣6️⃣ `create_schedule_command` ⭐ Admin Only

**Fungsi:** Menambahkan jadwal setor baru ke tabel `tanggal`.

**Parameter:**
```typescript
{
  tanggal: string    // Format: "YYYY-MM-DD", contoh: "2026-06-01"
  waktuBuka: string  // Format: "HH:MM", contoh: "09:00" (WITA)
  waktuTutup: string // Format: "HH:MM", contoh: "12:00" (WITA)
  lokasi: string     // Contoh: "Pos 1", "Gedung A"
}
```

> [!NOTE]
> Backend otomatis mengkonversi waktu dari `"HH:MM"` ke format PostgreSQL `"HH:MM:00+08"` (WITA = UTC+8) sebelum menyimpan ke DB.

**Return Type:** `()` (void)

**Code Example:**
```typescript
await invoke("create_schedule_command", {
  tanggal: "2026-06-15",
  waktuBuka: "09:00",
  waktuTutup: "12:00",
  lokasi: "Pos 1",
});
```

**Error:** Semua field wajib diisi. Error: `"Semua field wajib diisi."`

---

### 1️⃣7️⃣ `update_schedule_command` ⭐ Admin Only

**Fungsi:** Memperbarui jadwal yang sudah ada berdasarkan UUID.

**Parameter:**
```typescript
{
  idTanggal: string  // UUID jadwal yang akan diubah (dari id_tanggal di response get_schedules_command)
  tanggal: string    // Format: "YYYY-MM-DD"
  waktuBuka: string  // Format: "HH:MM"
  waktuTutup: string // Format: "HH:MM"
  lokasi: string
}
```

**Return Type:** `()` (void)

**Code Example:**
```typescript
await invoke("update_schedule_command", {
  idTanggal: "uuid-jadwal-yang-mau-diubah",
  tanggal: "2026-06-20",
  waktuBuka: "10:00",
  waktuTutup: "14:00",
  lokasi: "Pos 2",
});
```

---

### 1️⃣8️⃣ `delete_schedule_command` ⭐ Admin Only

**Fungsi:** Menghapus jadwal berdasarkan UUID.

**Parameter:**
```typescript
{
  idTanggal: string  // UUID jadwal yang akan dihapus
}
```

**Return Type:** `()` (void)

**Code Example:**
```typescript
await invoke("delete_schedule_command", {
  idTanggal: "uuid-jadwal-yang-mau-dihapus",
});
```

---

### 1️⃣9️⃣ `create_pricelist_command` ⭐ Admin Only

**Fungsi:** Menambahkan item baru ke katalog harga (tabel `pricelist`). Gambar PNG di-upload ke Supabase Storage bucket `photo_pricelist` dan URL-nya disimpan di kolom `img_url`.

**Parameter:**
```typescript
{
  labels: string       // Nama jenis sampah, contoh: "Plastik (PET)"
  price: number        // Harga per kg dalam Rupiah (harus > 0)
  imageBase64: string  // Data gambar PNG dalam format base64 TANPA prefix (wajib untuk item baru)
                       // Contoh: "iVBORw0KGgo..." (bukan "data:image/png;base64,iVBOR...")
}
```

> [!IMPORTANT]
> **Validasi gambar dilakukan 2 kali:**
> - **Frontend:** Cek `file.type === "image/png"` dan `file.size <= 524288` (512 KB) sebelum invoke
> - **Backend:** Cek magic bytes PNG (`89 50 4E 47`) dan ukuran ≤ 512 KB setelah decode base64
>
> **Rollback otomatis:** Jika gambar berhasil diupload ke Storage tapi insert DB gagal, backend otomatis menghapus gambar dari Storage.

**Cara ambil base64 dari file di frontend:**
```typescript
// Baca file sebagai base64, POTONG prefix data:image/png;base64,
const onFileSelect = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;

  // Validasi frontend (sebelum kirim ke backend)
  if (file.type !== "image/png") {
    showError("Hanya file PNG yang diizinkan.");
    return;
  }
  if (file.size > 524288) {  // 512 KB
    showError(`File terlalu besar (${Math.round(file.size / 1024)} KB). Maksimal 512 KB.`);
    return;
  }

  const reader = new FileReader();
  reader.onload = (e) => {
    const result = e.target?.result as string;
    // PENTING: Potong prefix "data:image/png;base64," — kirim hanya bagian setelah koma
    const base64 = result.split(",")[1] ?? "";
    form.imageBase64 = base64;
    form.imagePreviewUrl = result;  // Data URL untuk <img> preview
  };
  reader.readAsDataURL(file);
};

// Invoke ke backend
await invoke("create_pricelist_command", {
  labels: "Plastik (PET)",
  price: 4000,
  imageBase64: form.imageBase64,  // Base64 tanpa prefix
});
```

**Error yang Mungkin Dikembalikan:**
| Pesan Error | Penyebab |
|---|---|
| `"Nama sampah tidak boleh kosong."` | `labels` kosong |
| `"Harga harus lebih dari Rp0."` | `price` ≤ 0 |
| `"Gambar wajib dipilih untuk item baru."` | `imageBase64` kosong |
| `"File harus berformat PNG."` | Bukan PNG (magic bytes salah) |
| `"Ukuran gambar melebihi batas 512 KB (saat ini: N KB)."` | File > 512 KB |
| `"Gagal menyimpan gambar ke server."` | Error upload ke Storage |
| `"Gagal menyimpan item ke server."` | Error insert DB |

---

### 2️⃣0️⃣ `update_pricelist_command` ⭐ Admin Only

**Fungsi:** Memperbarui item pricelist. Gambar bersifat **opsional** — jika `imageBase64` dikosongkan, gambar lama dipertahankan.

**Parameter:**
```typescript
{
  id: string              // UUID item yang diupdate (dari field `id` di response get_pricelist_command)
  labels: string          // Nama baru
  price: number           // Harga baru
  imageBase64: string     // Base64 PNG tanpa prefix; KOSONGKAN jika tidak ganti gambar
  currentImgUrl: string   // URL gambar saat ini — digunakan backend untuk hapus gambar lama
                          // jika ada gambar baru. Ambil dari field `img_url` item yang sedang diedit.
}
```

> [!NOTE]
> Jika `imageBase64` dikosongkan (`""`), backend hanya mengupdate `labels` dan `price` — field `img_url` di DB tidak disentuh.

**Code Example:**
```typescript
// Mode edit: tidak ganti gambar
await invoke("update_pricelist_command", {
  id: item.id,
  labels: "Plastik Daur Ulang",
  price: 4500,
  imageBase64: "",              // Kosong = tidak ganti gambar
  currentImgUrl: item.img_url,  // URL lama tetap disimpan
});

// Mode edit: ganti gambar
await invoke("update_pricelist_command", {
  id: item.id,
  labels: "Plastik Daur Ulang",
  price: 4500,
  imageBase64: newImageBase64,   // Base64 PNG baru
  currentImgUrl: item.img_url,   // URL lama → backend hapus setelah upload baru sukses
});
```

---

### 2️⃣1️⃣ `delete_pricelist_command` ⭐ Admin Only

**Fungsi:** Menghapus item pricelist dari DB **dan** menghapus gambarnya dari Supabase Storage secara bersamaan.

**Parameter:**
```typescript
{
  id: string      // UUID item yang dihapus
  imgUrl: string  // URL gambar item (isi dengan img_url dari item); kosong jika tidak ada gambar
}
```

> [!NOTE]
> Penghapusan gambar dari Storage bersifat **best-effort** — jika gagal (misal file sudah tidak ada), item DB tetap dihapus dan tidak throw error. Hanya warning di local log.

**Code Example:**
```typescript
await invoke("delete_pricelist_command", {
  id: item.id,
  imgUrl: item.img_url ?? "",
});
// Setelah ini, refresh list
await fetchPricelist();
```

---



## 📊 Complete Data Models

### 1. Profile
```typescript
interface Profile {
  user_id: string;
  email: string;
  username: string;
  photo_url?: string;
  role: string;        // "user" atau "admin"
  status?: string;
  created_at?: string;
}
```

### 2. PricelistItem
```typescript
interface PricelistItem {
  id?: string;       // UUID (auto-generated oleh Supabase)
  labels: string;    // Nama jenis sampah, contoh: "Plastik (PET)"
  price: number;     // Harga per kg dalam Rupiah
  img_url?: string;  // Public URL gambar dari Supabase Storage bucket photo_pricelist
                     // ⚠️ PERHATIAN: Nilai lama mungkin "0" (string, bukan null).
                     // Selalu validasi: isValid = img_url && img_url !== "0" && img_url.startsWith("http")
  created_at?: string;
}
```

### 3. Log Payload (Untuk create_log_command & write_local_log_command)
```typescript
interface LogPayload {
  level: string;     // "INFO" | "WARNING" | "ERROR" | "DEBUG"
  message: string;   // Deskripsi lengkap kejadian/transaksi
}
```

### 4. ScheduleItem
```typescript
interface ScheduleItem {
  id_tanggal?: string;  // UUID jadwal
  tanggal: string;      // "YYYY-MM-DD"
  waktu_buka: string;   // "HH:MM:SS+08" (WITA) — ambil HH:MM dengan .slice(0,5)
  waktu_tutup: string;  // "HH:MM:SS+08" (WITA) — ambil HH:MM dengan .slice(0,5)
  lokasi: string;
}
```

### 5. NasabahItem
```typescript
interface NasabahItem {
  user_id: string;    // UUID nasabah
  username: string;
  email: string;
  photo_url?: string;
  saldo: number;      // 0 jika belum ada transaksi
}
```

### 6. Additional Models (Reference)
```typescript
// Referensi tabel transaksi scan
interface Scan {
  id: string;
  user_id: string;
  trash_type: string;
  weight_kg: number;
  price_per_kg: number;
  total_price: number;
  created_at: string;
}

// Referensi tabel saldo user
interface Savings {
  id: string;
  user_id: string;
  total_amount: number;
  currency: string;
  last_updated: string;
}

// Referensi tabel log_system di Supabase (Audit Trail)
interface LogSystem {
  id: string;
  user_id: string;
  level: string;       // "INFO", "WARNING", "ERROR"
  message: string;     // Deskripsi kejadian
  created_at: string;
}
```

---

## 🚀 Recommended App.vue Pattern

Ini adalah pattern yang sudah tested dan recommended untuk App.vue:

```vue
<template>
  <div id="app">
    <!-- Loading screen saat check auth -->
    <div v-if="isLoading" class="loading-container">
      <div class="spinner"></div>
      <p>Loading...</p>
    </div>

    <!-- Main content setelah loading selesai -->
    <RouterView v-else />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useRouter } from 'vue-router';
import { useUserStore } from '@/stores/userStore';

const router = useRouter();
const userStore = useUserStore();
const isLoading = ref(true);

// Fetch profile & redirect berdasarkan role
async function fetchProfileAndRoute() {
  try {
    const profile = await invoke('get_profile_command');
    console.log("Profile loaded:", profile);
    
    // Simpan ke store
    userStore.setProfile(profile);

    // Role-based routing
    if (profile.role === 'admin') {
      router.push('/admin/dashboard');
    } else {
      router.push('/user/dashboard');
    }
  } catch (err) {
    console.error("Error fetching profile:", err);
    showErrorToast("Failed to load profile");
    // Logout jika gagal fetch profile
    await invoke('logout_command');
    router.push('/login');
  } finally {
    isLoading.value = false;
  }
}

onMounted(async () => {
  try {
    isLoading.value = true;

    // 1. Check apakah user sudah login
    const isLoggedIn = await invoke('check_auth_status_command');
    console.log("Auth status:", isLoggedIn);

    if (isLoggedIn) {
      // 2. Fetch profile jika sudah login
      await fetchProfileAndRoute();
    } else {
      // 3. Redirect ke login jika belum login
      isLoading.value = false;
      router.push('/login');
    }
  } catch (err) {
    console.error("Auth check error:", err);
    isLoading.value = false;
    router.push('/login');
  }

  // 4. Listen untuk event login-success dari OAuth deep link
  await listen('login-success', async () => {
    console.log("Login success event received!");
    isLoading.value = true;
    await fetchProfileAndRoute();
  });
});
</script>

<style scoped>
.loading-container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.spinner {
  border: 4px solid rgba(255, 255, 255, 0.3);
  border-top: 4px solid white;
  border-radius: 50%;
  width: 40px;
  height: 40px;
  animation: spin 0.8s linear infinite;
  margin-bottom: 20px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>
```

---

## 🛡️ Error Handling Guide

### Common Errors & Solutions

| Error | Cause | Solution |
|-------|-------|----------|
| `"Auth token not found"` | User belum login | Redirect ke /login |
| `"Invalid or expired token"` | Token sudah kadaluarsa | Auto-refresh (backend handle) |
| `"User profile not found"` | Profile belum dibuat saat signup | Create profile di signup flow |
| `"Unauthorized"` | User tidak punya akses | Check role, redirect ke /login |
| `"Failed to get Google auth URL"` | Koneksi internet/config Google | Check internet, check .env |
| `"Network error"` | Backend tidak accessible | Check backend running |

### Error Handling Pattern

```typescript
import { invoke } from '@tauri-apps/api/core';

async function callBackendCommand(commandName: string, params?: any) {
  try {
    const result = await invoke(commandName, params);
    return { success: true, data: result };
  } catch (error) {
    console.error(`Error calling ${commandName}:`, error);
    
    // Handle specific errors
    const errorStr = String(error);
    if (errorStr.includes('token')) {
      // Token error - logout
      await invoke('logout_command');
      router.push('/login');
      return { success: false, error: 'Session expired, please login again' };
    } else if (errorStr.includes('Network')) {
      return { success: false, error: 'Network error, check internet connection' };
    } else {
      return { success: false, error: errorStr };
    }
  }
}

// Usage
const result = await callBackendCommand('get_profile_command');
if (!result.success) {
  showErrorToast(result.error);
} else {
  console.log("Profile:", result.data);
}
```

---

## ✅ Frontend Integration Checklist

**Phase 1: Setup**
- [ ] Install @tauri-apps/api package
- [ ] Create TypeScript interfaces untuk semua models
- [ ] Setup Pinia/Vuex store untuk user data
- [ ] Setup Vue Router dengan protected routes

**Phase 2: Authentication**
- [ ] Implement App.vue with auth check on mount
- [ ] Create Login page dengan "Login dengan Google" button
- [ ] Call `get_google_auth_url_command` & buka URL
- [ ] Listen event `login-success` dari OAuth redirect
- [ ] Call `get_profile_command` setelah login
- [ ] Implement `logout_command` di logout button

**Phase 3: Core Features**
- [ ] Create Dashboard page (protected)
- [ ] Call `get_profile_command` untuk user info
- [ ] Call `get_pricelist_command` di pricing/scanning page
- [ ] Implement scan form dengan `create_log_command`
- [ ] Add loading spinners untuk semua async calls

**Phase 4: Logging & Analytics**
- [ ] Call `create_log_command` untuk page views
- [ ] Log scan/transaction activities
- [ ] Handle silent errors untuk logging

**Phase 5: Error Handling**
- [ ] Implement global error handler
- [ ] Handle token expiration & auto-refresh
- [ ] Show user-friendly error messages
- [ ] Log errors ke backend

**Phase 6: Testing & Polish**
- [ ] Test login flow lengkap
- [ ] Test logout & redirect
- [ ] Test role-based routing
- [ ] Test error scenarios
- [ ] Test offline behavior

---

## 🐛 Debug Tips

### Enable Backend Logs
Saat develop, backend print logs. Buka Tauri console untuk melihat:
- Apa token yang dipakai
- Query apa yang di-execute
- Error detail dari Supabase

### Common Issues

**Issue: "invoke not found"**
```typescript
// ❌ WRONG
const profile = get_profile_command();

// ✅ CORRECT
import { invoke } from '@tauri-apps/api/core';
const profile = await invoke('get_profile_command');
```

**Issue: get_profile_command return array, bukan object**
```typescript
// ❌ WRONG - Treat as array
const name = profiles[0].username;

// ✅ CORRECT - Treat as single object
const name = profile.username;
```

**Issue: Token tidak auto-refresh**
- Backend sudah handle auto-refresh saat `check_auth_status_command`
- Cek di server logs apakah refresh successful

**Issue: Event login-success tidak trigger**
- Pastikan deep link sudah di-register: `com.users.scantrash://auth?access_token=xxx&refresh_token=yyy`
- Check tauri.conf.json untuk deep link config
- Check backend auth_service apakah emit event

---

## 📞 Support

Jika ada issue atau pertanyaan:
1. Check kode di `src-tauri/src/` untuk implementasi detail
2. Check Tauri docs: https://docs.rs/tauri/
3. Check Supabase docs: https://supabase.com/docs
4. Tanya di grup apl

---

**Last Updated:** 2026-05-17 — Admin Transaction & CRUD Features
**Status:** Ready for Frontend Integration ✅

### Changelog
- **2026-05-17:**
  - **NEW API (Admin):** `create_setoran_command` — setor saldo ke nasabah (dengan validasi nominal > 0).
  - **NEW API (Admin):** `create_penarikan_command` — tarik saldo dari nasabah (dengan validasi kecukupan saldo otomatis).
  - **NEW API (Admin):** `create_schedule_command` — tambah jadwal setor.
  - **NEW API (Admin):** `update_schedule_command` — edit jadwal setor.
  - **NEW API (Admin):** `delete_schedule_command` — hapus jadwal setor.
  - **NEW API (Admin):** `create_pricelist_command` — tambah item katalog + upload gambar PNG ke Supabase Storage.
  - **NEW API (Admin):** `update_pricelist_command` — edit item katalog (gambar opsional, rollback otomatis jika DB gagal).
  - **NEW API (Admin):** `delete_pricelist_command` — hapus item katalog + hapus gambar dari Storage.
  - **SECURITY:** Dual-log policy: local log tanpa detail finansial; audit trail lengkap di Supabase `log_system`.
  - **ROLLBACK:** create/update pricelist otomatis hapus gambar dari Storage jika insert/update DB gagal.
  - **FIX:** Model `Pricelist` di dokumentasi diperbarui agar sesuai kolom DB aktual.
- **2026-05-09:** 
  - **NEW API:** `get_balance_command` ditambahkan untuk mengambil saldo realtime dari Supabase.
  - **NEW API:** `get_savings_history_command` ditambahkan untuk mengambil 50 riwayat transaksi (`savings`) terbaru.
  - **NEW API:** `get_schedules_command` ditambahkan untuk membaca jadwal setor dari tabel `tanggal`.
  - **BUG FIX:** Memperbaiki bug pada layanan AI di mana JWT yang expired diabaikan dan membuat scan bingung tanpa sistem rules.
  - **BUG FIX:** Memperbaiki `get_pricelist_command` yang salah menggunakan anon_key untuk autentikasi dan memperbaiki hilangnya kolom `img_url`.
- **2026-04-28:** 
  - Ditambahkan `read_local_log_command` (baca log dari HP)
  - `write_local_log_command`: Timestamp `[YYYY-MM-DD HH:MM:SS]` sekarang otomatis ditambahkan oleh backend, frontend tidak perlu mengirim waktu.
  - `scan_trash`: Pesan error ke frontend sekarang bersifat generik (tidak bocorkan URL/stack trace). Ditambahkan validasi ukuran gambar (max ~5MB) dan validasi format (hanya jpeg/png/webp). Ditambahkan tabel Error Reference.
  - Internal logic `scan_trash` dipindah ke `scan_service.rs` (tidak berdampak pada cara penggunaan dari Vue).