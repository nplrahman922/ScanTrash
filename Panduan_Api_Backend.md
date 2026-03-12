# ScanTrash Backend API Documentation

Dokumentasi lengkap API backend Rust untuk Frontend Vue.js. Backend menggunakan **Tauri v2** dengan **Supabase** sebagai database.

---

## 🔄 Changelog - Latest Updates

### Recent Changes (Profile Service & App.vue Pattern)

**Backend: `profile_service.rs` Update**
- Implementasi `get_user_profile()` sekarang lebih aman dengan auto-filtering by user_id
- User_id otomatis di-extract dari access token (via `auth_service::get_user_id()`)
- Query ke Supabase sekarang include filter `user_id=eq.{user_id}` untuk security
- Response type: **SINGLE OBJECT** (bukan array)

**Frontend: `App.vue` Recommended Pattern**
- Ditambahkan `isLoading` state untuk prevent UI flickering
- Function `fetchProfileAndRoute()` untuk reusable logic
- Better error handling & proper loading state management
- Role-based routing (admin vs user dashboard)

**Frontend Checklist:**
- [ ] Treat `get_profile_command` response sebagai **single object**, bukan array
- [ ] Gunakan loading state di semua async operations
- [ ] Implement role-based routing untuk redirect yang tepat
- [ ] Listen `login-success` event dari deep link OAuth
- [ ] Jangan pernah send `user_id` parameter - backend handle otomatis
- [ ] Never store token di frontend - backend manage semua

---

## 📋 Table of Contents

1. [Changelog](#changelog---latest-updates) ← **LIHAT DULU!**
2. [Inisialisasi & Setup](#inisialisasi--setup)
3. [Authentication Flow](#authentication-flow)
4. [Available Commands](#available-commands)
5. [Data Models](#data-models)
6. [Error Handling](#error-handling)
7. [Best Practices](#best-practices)

---

## 🚀 Inisialisasi & Setup

### Pertama-tama buat config.rs di src-tauri/src/config.rs
Isinya adalah

``` rust
pub struct AppConfig {
    pub supabase_url: String,
    pub supabase_key: String,
}

impl AppConfig {
    pub fn init() -> Self {
        // Coba load .env (hanya berguna saat jalan di Windows/Desktop)
        dotenvy::dotenv().ok();

        AppConfig {
            // Gunakan unwrap_or_else agar TIDAK PANIC jika variabel tidak ada.
            // Sementara kita hardcode fallback-nya untuk testing di Android.
            // Nanti key-nya sesuaikan dengan yang ada di diagram ERD kamu ya.
            supabase_url: std::env::var("SUPABASE_URL")
                .unwrap_or_else(|_| "https://[supabase url kita lihat di group].supabase.co".to_string()),

            supabase_key: std::env::var("SUPABASE_KEY")
                .unwrap_or_else(|_| "[sb publishable lihat di group juga]".to_string()),
        }
    }
}

```

### AppState (Brankas Token)

Backend menggunakan `AppState` untuk menyimpan token di RAM dan persistent storage:

```rust
pub struct AppState {
    pub access_token: Mutex<Option<String>>,
    pub refresh_token: Mutex<Option<String>>,
}
```

**Token disimpan di dua tempat:**
- **RAM**: Akses cepat, hilang saat app ditutup
- **Disk**: Menggunakan Tauri's built-in store plugin, persistent

Jangan khawatir tentang token management di frontend - backend menangani auto-refresh dan penyimpanan otomatis.

---

## 🔐 Authentication Flow

### 1. **Login dengan Google OAuth**

#### Step 1: Dapatkan URL Google Auth
```typescript
// Frontend Vue.js
import { invoke } from '@tauri-apps/api/core';

const authUrl = await invoke('get_google_auth_url_command');
// Buka authUrl di browser
window.open(authUrl, '_blank');
```

#### Step 2: Backend menerima Deep Link
Setelah user login di Google, OAuth redirect ke deep link:
```
scantrash://oauth?access_token=xxx&refresh_token=yyy
```

Backend otomatis:
- Parsing token dari URL
- Menyimpan ke RAM & Disk
- Emit event `login-success` ke frontend

#### Step 3: Frontend listen event login-success
```typescript
import { listen } from '@tauri-apps/api/event';

listen('login-success', (event) => {
  console.log("Login berhasil!", event.payload);
  // Redirect ke home page
});
```

---

### 2. **Check Auth Status (dengan Auto-Refresh)**

```typescript
const isAuthenticated = await invoke('check_auth_status_command');

if (isAuthenticated) {
  console.log("✅ User sudah login");
} else {
  console.log("❌ User belum login");
}
```

**Apa yang terjadi di backend:**
1. ✅ Ambil token dari storage
2. ✅ Validasi token ke Supabase
3. ✅ Jika expired → auto-refresh token otomatis
4. ✅ Return `true` jika token valid, `false` jika gagal
5. ✅ Token baru otomatis disimpan (tanpa action dari frontend)

**Frontend tidak perlu khawatir** tentang token refresh - backend menangani semuanya!

---

### 3. **Logout**

```typescript
await invoke('logout_command');
// Token otomatis dihapus dari RAM & Disk
// Frontend bisa langsung redirect ke login page
```

**Apa yang terjadi di backend:**
1. Hapus token dari RAM
2. Hapus token dari Disk
3. Notifikasi ke Supabase untuk matikan sesi

---

## 📡 Available Commands

Semua command dipanggil dari Frontend dengan syntax:
```typescript
import { invoke } from '@tauri-apps/api/core';

const result = await invoke('command_name', { param: value });
```

---

### 1️⃣ `get_google_auth_url_command`

**Tujuan:** Generate URL untuk Google OAuth

**Parameter:** Tidak ada

**Return:** `String` (URL Google Auth)

**Contoh:**
```typescript
const authUrl = await invoke('get_google_auth_url_command');
window.open(authUrl, '_blank');
```

---

### 2️⃣ `check_auth_status_command`

**Tujuan:** Cek apakah user sudah login (dengan auto-refresh token)

**Parameter:** Tidak ada (AppHandle otomatis di-pass oleh Tauri)

**Return:** `Result<bool, String>`
- `true` = User sudah login, token valid
- `false` = User belum login atau token invalid
- `Err` = Ada error saat check

**Contoh:**
```typescript
try {
  const isLoggedIn = await invoke('check_auth_status_command');
  
  if (isLoggedIn) {
    console.log("✅ Sudah login, bisa akses profile");
  } else {
    console.log("❌ Belum login, redirect ke login page");
  }
} catch (error) {
  console.error("Error checking auth:", error);
}
```

**Best Practice:**
- Panggil command ini saat app startup untuk ensure token masih valid
- Gunakan di route guard untuk protect authenticated pages

---

### 3️⃣ `logout_command`

**Tujuan:** Logout user dan hapus token

**Parameter:** Tidak ada (AppHandle otomatis di-pass oleh Tauri)

**Return:** `Result<(), String>`

**Contoh:**
```typescript
try {
  await invoke('logout_command');
  console.log("✅ Logout berhasil");
  // Redirect ke login page
  router.push('/login');
} catch (error) {
  console.error("Logout gagal:", error);
}
```

---

### 4️⃣ `get_profile_command`

**Tujuan:** Ambil data profile user yang sedang login

**Parameter:** Tidak ada (Token otomatis diambil dari storage)

**Return:** `Result<Profile, String>` - Return SINGLE OBJECT langsung, bukan array

**Struktur Profile:**
```typescript
interface Profile {
  user_id: string;
  email: string;
  username: string;
  photo_url?: string;
  role: string;
  status?: string;
  created_at?: string;
}
```

**Contoh:**
```typescript
try {
  const profile = await invoke('get_profile_command');
  console.log("Profile user:", profile);
  // ✅ Backend return SINGLE OBJECT (bukan array)
  // {
  //   user_id: "uuid-xxx",
  //   email: "user@gmail.com",
  //   username: "john_doe",
  //   photo_url: "https://...",
  //   role: "user",
  //   status: "active",
  //   created_at: "2025-01-01T10:00:00Z"
  // }
  
  // ✅ Frontend bisa langsung akses properties
  console.log(profile.username);  // "john_doe"
  console.log(profile.role);      // "user"
} catch (error) {
  console.error("Error fetching profile:", error);
}
```

**Requirement:** 
- User harus sudah login (ada valid token)
- Backend otomatis filter berdasarkan `user_id` yang ada di token
- Hanya profile user yang sedang login yang akan dikembalikan

**Backend Logic:**
1. Extract `user_id` dari access token
2. Query `profiles` table dengan filter `user_id=eq.{user_id}`
3. Gunakan header `Accept: application/vnd.pgrst.object+json` untuk return single object
4. Return Profile object langsung (bukan array)

---

### 5️⃣ `get_pricelist_command`

**Tujuan:** Ambil daftar harga sampah untuk prediksi nilai

**Parameter:** Tidak ada

**Return:** `Result<Vec<Pricelist>, String>`

**Struktur Pricelist:**
```typescript
interface Pricelist {
  id?: string;
  labels: string;        // Nama jenis sampah (e.g., "plastic", "paper")
  price: number;         // Harga per kg
  created_at?: string;
}
```

**Contoh:**
```typescript
try {
  const priceList = await invoke('get_pricelist_command');
  console.log("Daftar harga:", priceList);
  // [
  //   { id: "1", labels: "plastic", price: 5000, created_at: "..." },
  //   { id: "2", labels: "paper", price: 2000, created_at: "..." },
  //   { id: "3", labels: "metal", price: 8000, created_at: "..." }
  // ]
  
  // Bisa digunakan untuk kalkulasi harga setelah scan
} catch (error) {
  console.error("Error fetching pricelist:", error);
}
```

**Note:** Command ini tidak memerlukan authentication, bisa diakses siapa saja.

---

### 6️⃣ `create_log_command`

**Tujuan:** Catat activity log user ke database

**Parameter:**
```typescript
{
  level: string,    // "info" | "warning" | "error" | "debug"
  message: string   // Pesan log
}
```

**Return:** `Result<String, String>`
- Success: `"Log berhasil dicatat!"`
- Error: Pesan error dari server

**Contoh:**
```typescript
try {
  const response = await invoke('create_log_command', {
    level: 'info',
    message: 'User berhasil scan item pertama'
  });
  console.log(response); // "Log berhasil dicatat!"
} catch (error) {
  console.error("Error creating log:", error);
}
```

**Backend secara otomatis:**
- Ambil `user_id` dari token yang tersimpan
- Inject `created_at` timestamp
- Menyimpan ke database

**Kapan gunakan:**
- User login → log "User login"
- User scan item → log "Item scanned: [label]"
- Terjadi error → log "Error: [message]"
- User logout → log "User logout"

---

## 📊 Data Models

### Semua Model yang Tersedia:

#### 1. **LogSystem** (Tabel: `log_system`)
```typescript
interface LogSystem {
  id?: string;
  user_id: string;
  level: string;      // "info", "warning", "error", "debug"
  message: string;
  created_at?: string;
}
```

#### 2. **Profile** (Tabel: `profiles`) - Auto-filtered by user_id

```typescript
interface Profile {
  user_id: string;
  email: string;
  username: string;
  photo_url?: string;
  role: string;       // "admin", "user", etc
  status?: string;    // "active", "inactive"
  created_at?: string;
}
```

**⚠️ PENTING:** Saat call `get_profile_command`:
- Backend otomatis extract `user_id` dari token
- Hanya profile dengan user_id yang sesuai yang dikembalikan
- Frontend tidak perlu kirim user_id sebagai parameter
- Return tipe adalah SINGLE OBJECT, bukan array
- Bisa langsung akses: `profile.username`, `profile.role`, etc

#### 3. **Scan** (Tabel: `scan`)
```typescript
interface Scan {
  user_id: string;
  img_url?: string;           // URL foto hasil scan
  label?: string;             // Jenis sampah yang terdeteksi
  price_predict?: string;     // Prediksi harga
  created_at?: string;
}
```

#### 4. **Pricelist** (Tabel: `pricelist`)
```typescript
interface Pricelist {
  id?: string;
  labels: string;     // Jenis sampah
  price: number;      // Harga per kg
  created_at?: string;
}
```

#### 5. **Savings** (Tabel: `savings`)
```typescript
interface Savings {
  id?: string;
  user_id: string;
  amount_before: number;  // Saldo sebelumnya
  amount: number;         // Saldo sekarang
  created_at?: string;
}
```

---

## ⚠️ Error Handling

Semua command return `Result<T, String>` dimana error adalah pesan `String`.

**Penanganan Error di Frontend:**

```typescript
try {
  const result = await invoke('command_name', { /* params */ });
  // Success handling
} catch (error) {
  const errorMessage = error as string;
  
  if (errorMessage.includes("Akses ditolak")) {
    // Token tidak valid, redirect ke login
    router.push('/login');
  } else if (errorMessage.includes("tidak ada sesi aktif")) {
    // User belum login
    showToast("Silakan login terlebih dahulu");
  } else {
    // Error umum
    console.error("Error:", errorMessage);
  }
}
```

**Common Error Messages:**

| Error Message | Penyebab | Solusi |
|---|---|---|
| `Akses ditolak: User belum login!` | Belum ada valid token | Suruh user login |
| `Akses ditolak: Tidak ada sesi aktif.` | Session sudah expired | Suruh user login ulang |
| `Network error` | Tidak bisa connect ke Supabase | Cek internet connection |
| `Invalid token` | Token tidak valid di Supabase | Auto-refresh akan trigger, atau login ulang |

---

## ✅ Best Practices

### 1. **Route Protection (Guard)**

```typescript
// router/guards.ts
import { invoke } from '@tauri-apps/api/core';

export async function checkAuthGuard() {
  try {
    const isLoggedIn = await invoke('check_auth_status_command');
    return isLoggedIn;
  } catch {
    return false;
  }
}

// Gunakan di route
beforeEach(async (to, from, next) => {
  if (to.meta.requiresAuth) {
    const isAuth = await checkAuthGuard();
    isAuth ? next() : next('/login');
  } else {
    next();
  }
})
```

### 2. **Load Profile setelah Login (dengan Loading State)**

```typescript
// Setelah berhasil login
const isLoading = ref(true);

listen('login-success', async () => {
  try {
    isLoading.value = true;  // Nyalakan loading spinner
    const profile = await invoke('get_profile_command');
    // Profile adalah SINGLE OBJECT, bukan array
    console.log("Nama:", profile.username);  // Bisa akses langsung
    
    // Simpan ke Pinia/Vuex store
    userStore.setProfile(profile);
    
    // Redirect berdasarkan role
    if (profile.role === 'admin') {
      router.push('/admin-dashboard');
    } else {
      router.push('/user-dashboard');
    }
  } catch (error) {
    console.error("Error loading profile:", error);
    showToast("Gagal mengambil profil, silakan login ulang");
  } finally {
    isLoading.value = false;  // Matikan loading spinner
  }
});
```

### 3. **Cache Pricelist**

```typescript
// Load pricelist sekali saja saat app startup
const pricelist = await invoke('get_pricelist_command');
// Simpan ke Pinia store untuk reuse
appStore.setPricelist(pricelist);

// Kemudian gunakan dari store, jangan panggil command lagi
```

### 4. **Logging Activity**

```typescript
// Setiap action penting, catat ke log
async function scanItem(label: string) {
  // ... scan logic ...
  
  // Catat ke database
  await invoke('create_log_command', {
    level: 'info',
    message: `Item scanned: ${label}`
  }).catch(err => console.error("Log failed:", err));
}
```

### 5. **Auto-Check Auth saat App Load (Recommended Pattern)**

Gunakan pattern seperti di App.vue untuk better UX:

```typescript
// App.vue
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

const isLoading = ref(true);
const userProfile = ref(null);

async function fetchProfileAndRoute() {
  try {
    const profile = await invoke('get_profile_command');
    userProfile.value = profile;  // Profile adalah SINGLE OBJECT
    
    // Redirect berdasarkan role
    if (profile.role === 'admin') {
      router.push('/admin-dashboard');
    } else {
      router.push('/user-dashboard');
    }
  } catch (err) {
    console.error("Profile Error:", err);
  } finally {
    isLoading.value = false;  // PENTING: matikan loading
  }
}

onMounted(async () => {
  try {
    isLoading.value = true;  // Nyalakan loading saat check auth
    const isLoggedIn = await invoke('check_auth_status_command');
    
    if (isLoggedIn) {
      await fetchProfileAndRoute();
    } else {
      isLoading.value = false;
      router.push('/login');
    }
  } catch (err) {
    isLoading.value = false;
    router.push('/login');
  }

  // Listen event login-success dari deep link
  listen('login-success', async () => {
    isLoading.value = true;
    await fetchProfileAndRoute();
  });
});
```

**Kenapa pattern ini penting:**
- Loading state mencegah UI flickering
- Proper error handling jika profile gagal diambil
- Better UX dengan loading indicator
- Prevent race condition dengan listen event

### 6. **Never Store Token di Frontend**

```typescript
// ❌ JANGAN LAKUKAN INI
localStorage.setItem('token', accessToken);

// ✅ Backend sudah handle token storage secara aman
// Frontend tinggal panggil commands
```

---

## 🔄 Call Sequence Example: Login → Home

```
1. User di login page klik "Login with Google"
   └─ Frontend: invoke('get_google_auth_url_command')
   └─ Frontend: window.open(authUrl)

2. User login di Google, OAuth redirect ke deep link
   └─ Backend: Parse token dari URL
   └─ Backend: Simpan token ke RAM & Disk
   └─ Backend: Emit 'login-success' event

3. Frontend listen event 'login-success'
   └─ Frontend: Panggil invoke('check_auth_status_command')
   └─ Backend: Return true (sudah ada token valid)

4. Frontend load profile
   └─ Frontend: invoke('get_profile_command')
   └─ Backend: Return user profile

5. Frontend load pricelist
   └─ Frontend: invoke('get_pricelist_command')
   └─ Backend: Return pricelist

6. Frontend redirect ke /home
   └─ Frontend: Tampilkan profile & pricelist di UI
```

---

## 🐛 Debug Tips

Kalau ada issue saat development:

1. **Check Rust Console Output**
   ```
   Cari log dengan prefix [RUST] atau emoji 🔥 ✅ ❌
   Itu adalah debug message dari backend
   ```

2. **Check Token Status**
   ```
   Backend print token baru setiap kali refresh
   Cari "[DEBUG] ACCESS TOKEN BARU:" di console
   ```

3. **Check Deep Link**
   ```
   Pastikan OAuth redirect URL sesuai dengan deep link handler
   Harus format: scantrash://oauth?access_token=xxx&refresh_token=yyy
   ```

4. **Validate Commands**
   ```typescript
   // Sebelum deploy, test semua commands
   const commands = [
     'get_google_auth_url_command',
     'check_auth_status_command',
     'logout_command',
     'get_profile_command',
     'get_pricelist_command',
     'create_log_command'
   ];
   ```

---

## 📝 Checklist Frontend Integration

- [ ] Load `get_google_auth_url_command` saat user di login page
- [ ] Listen event `login-success` dan redirect ke home
- [ ] Implement route guard dengan `check_auth_status_command`
- [ ] Load profile dengan `get_profile_command` setelah login
- [ ] Cache pricelist dengan `get_pricelist_command`
- [ ] Log activity dengan `create_log_command`
- [ ] Implement logout dengan `logout_command`
- [ ] Test semua flows di real Android device
- [ ] Handle network errors dan token expiration
- [ ] Remove debug console.log sebelum production

---

**Good luck! Hubungi backend team kalau ada pertanyaan. 🚀**
