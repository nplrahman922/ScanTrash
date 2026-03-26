# ScanTrash Backend API Documentation

Dokumentasi lengkap untuk Frontend Team - Semua command/invoke yang perlu digunakan untuk integrasi dengan backend Rust Tauri.

**Stack:** Rust + Tauri v2 + Supabase PostgreSQL

## Sebelum Memulai Buat .env dulu yagesya
buat di Scantrash/.env
``` bash
SUPABASE_URL=url di group
SUPABASE_KEY=kunci juga di group
SESSION_SECRET_KEY=Sc4nTr4sh_S3cur3_K3y_2026_Atau_Apapun_Bebas
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
| `create_log_command` | Log aktivitas user secara online dan disimpan di supabase | ✅ Ya | `()` |
| `write_local_log_command` | Log aktivitas user secara lokal | ❌ Tidak | `()` |
| `read_local_log_command` | Baca log dari Memori HP | ❌ Tidak | `String` |
| `analyze_trash_command` | Deteksi gambar sampah via AI | ✅ Ya | `ScanResult` (Object) |

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
interface Pricelist {
  id: string;              // UUID
  trash_type: string;      // Jenis sampah (Plastik, Kertas, dll)
  description?: string;    // Deskripsi
  price_per_kg: number;    // Harga per kg
  unit: string;            // Satuan (kg, pcs, dll)
  category?: string;       // Kategori
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

interface Pricelist {
  id: string;
  trash_type: string;
  description?: string;
  price_per_kg: number;
  unit: string;
  category?: string;
  created_at?: string;
}

async function loadPricelist() {
  try {
    // Return ARRAY of pricelists
    const pricelists = await invoke<Pricelist[]>('get_pricelist_command');
    
    console.log("Pricelist items:", pricelists.length);
    
    // Simpan ke store untuk reusable
    pricelistStore.setPricelists(pricelists);
    
    // Display di UI
    pricelists.forEach(item => {
      console.log(`${item.trash_type}: Rp${item.price_per_kg}/${item.unit}`);
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

### 8️⃣ read_local_log_command (Debug Tool)
**Fungsi:** Membaca seluruh isi log dari memori HP untuk ditampilkan di UI.

**Return:** String (Isi teks log)

**Kapan digunakan:** Di halaman "Settings" atau "Developer Mode" untuk melihat riwayat error tanpa kabel USB.

**Code Example:**
```typescript
async function showDebugLogs() {
  const logContent = await invoke<string>('read_local_log_command');
  console.log("Isi Log HP:", logContent);
}
```

### 9️⃣ analyze_trash_command (Saat ini masih mockup saja)
**Fungsi:** Menerima gambar sampah dari device pengguna (Kamera/Galeri), memprosesnya melalui model AI (OpenAI Vision) untuk mendeteksi jenis material dan kelayakannya, mencocokkan harga dari database Supabase, dan mengembalikan hasil analisis lengkap ke layar UI.

**Parameter:** 
``` Typescript
{
  imageBase64: string // Teks Base64 murni TANPA prefix "data:image/jpeg;base64,"
}
```

**Return:** ``Result<ScanResult, String>`` (Akan melempar error string jika gagal).

**Response Model (``ScanResult``):**
```Typescript
interface ScanResult {
  status: string;         // "success" | "failed" | "unrecognized"
  trash_type: string;     // Nama display untuk UI (Contoh: "Botol Plastik PET")
  label_id: string;       // ID untuk referensi DB (Contoh: "plastic_pet")
  kelayakan: string;      // Kategori kelayakan (Contoh: "Layak Ditabung")
  material_info: string;  // Penjelasan material (Contoh: "Plastik PET, ukuran standar.")
  kondisi: string;        // Hasil visual AI (Contoh: "Utuh namun sudah diremas.")
  kebersihan: string;     // Hasil visual AI (Contoh: "Sangat bersih.")
  estimasi_harga: number; // Harga per satuan/kg dari DB Supabase (Contoh: 4000)
}
```

**PENTING UNTUK FRONTEND!!!:** JANGAN PERNAH mengirimkan file gambar mentah (raw image) langsung dari kamera ke dalam command ini. Resolusi kamera HP modern bisa mencapai 10MB - 20MB. Jika file sebesar itu diubah menjadi Base64 dan dikirim ke Rust, memori HP akan penuh (Out of Memory/OOM) dan aplikasi akan Force Close (Crash).

**Aturan Wajib Frontend:** 
1. Tangkap gambar dari tag ``<video>`` atau input file galeri. 
2. Gambar WAJIB di-resize/kompres menggunakan HTML5 ``<canvas>`` di sisi Vue.
3. Batas maksimal ukuran dimensi: 800x800 pixels. (bisa didiskusikan untuk ukuran)
4. Format kompresi: JPEG dengan Quality 70% (0.7). (akan didiskusikan)
5. Buang awalan data:image/jpeg;base64, sebelum dikirim ke invoke.



**Code Example (Vue.js Integration):**
```typescript
import { invoke } from '@tauri-apps/api/core';

// 1. Fungsi Utama Pemanggilan AI
async function scanSampah(fileGambarAsli: File) {
  try {
    console.log("Memulai kompresi gambar...");
    
    // Wajib panggil fungsi kompresi dulu!
    const base64Aman = await compressImageForAI(fileGambarAsli);
    
    console.log("Gambar berhasil dikompres, mengirim ke Backend/AI...");
    
    // Panggil command Rust
    const result = await invoke<ScanResult>('analyze_trash_command', { 
      imageBase64: base64Aman 
    });

    console.log("✅ Hasil Scan Sukses:", result);
    // TODO: Tampilkan result.trash_type dan result.estimasi_harga ke UI Bottom Sheet
    
  } catch (error) {
    console.error("❌ Gagal melakukan scan:", error);
    // TODO: Tampilkan Toast error ke user ("Gagal mengenali gambar, pastikan pencahayaan cukup")
  }
}

// 2. Fungsi Helper: Kompresi Gambar (Standard Krenova)
function compressImageForAI(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = (event) => {
      const img = new Image();
      img.src = event.target?.result as string;
      img.onload = () => {
        const canvas = document.createElement('canvas');
        const MAX_WIDTH = 800; 
        const MAX_HEIGHT = 800;
        let width = img.width;
        let height = img.height;

        // Hitung rasio untuk mencegah gambar gepeng
        if (width > height) {
          if (width > MAX_WIDTH) {
            height *= MAX_WIDTH / width;
            width = MAX_WIDTH;
          }
        } else {
          if (height > MAX_HEIGHT) {
            width *= MAX_HEIGHT / height;
            height = MAX_HEIGHT;
          }
        }

        canvas.width = width;
        canvas.height = height;
        const ctx = canvas.getContext('2d');
        ctx?.drawImage(img, 0, 0, width, height);

        // Export ke JPEG 70%
        const dataUrl = canvas.toDataURL('image/jpeg', 0.7);
        // Hapus prefix agar Rust menerima murni teks Base64
        const base64String = dataUrl.split(',')[1]; 
        
        resolve(base64String);
      };
      img.onerror = (error) => reject(error);
    };
    reader.onerror = (error) => reject(error);
  });
}
```

**⏱️ Catatan Latensi & Alur Backend (Informasi Tambahan)**
Tim Frontend perlu menambahkan Loading Spinner (seperti "Menganalisis Sampah...") karena command ini membutuhkan waktu pemrosesan sekitar 3 hingga 7 detik (tergantung koneksi internet).

Hal ini terjadi karena di belakang layar (Backend), command ini melakukan serangkaian tugas berat secara berurutan:
1. Menerima gambar dan merakit Prompt untuk AI.

2. Mengirim gambar ke API OpenAI (Menunggu respons AI).

3. Mengecek harga ke Database Supabase (tabel pricelist) berdasarkan tebakan AI.

4. Mengunggah gambar tersebut ke Supabase Storage Bucket.

5. Menyimpan riwayat transaksi scan ke dalam Database.

6. Mengembalikan ScanResult ke Frontend.

Atau bisa langsung lihat implementasinya di src/views/ScanView.vue

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

### 2. Pricelist
```typescript
interface Pricelist {
  id: string;
  trash_type: string;
  description?: string;
  price_per_kg: number;
  unit: string;        // "kg", "pcs", etc
  category?: string;
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

### 4. Additional Models (Reference)
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

**Last Updated:** Latest - Sesuai dengan kode backend saat ini
**Status:** Ready for Frontend Integration ✅