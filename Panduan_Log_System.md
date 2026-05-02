# 📋 Panduan Sistem Log ScanTrash

## Gambaran Umum

ScanTrash punya **2 jenis log** dengan tujuan berbeda:

```
┌─────────────────────────────────────────────────────────┐
│                   SISTEM LOG SCANTRASH                  │
├──────────────────────────┬──────────────────────────────┤
│   LOCAL LOG (di HP)      │   ONLINE LOG (Supabase)      │
│   write_local_log_command│   create_log_command         │
├──────────────────────────┼──────────────────────────────┤
│ • Navigasi & UI          │ • Transaksi keuangan         │
│ • Error jaringan ringan  │ • Hasil scan AI              │
│ • Debug developer        │ • Perubahan data penting     │
│ • Peringatan sistem      │ • Audit trail                │
└──────────────────────────┴──────────────────────────────┘
```

---

## 📁 Local Log — `write_local_log_command`

### Format Output di File
```
[YYYY-MM-DD HH:MM:SS] [LEVEL] pesan kamu
```
**Contoh nyata:**
```
[2026-05-02 13:45:01] [INFO] User membuka halaman kamera.
[2026-05-02 13:45:20] [WARNING] Koneksi internet terputus saat memuat pricelist.
[2026-05-02 13:46:10] [ERROR] Gagal mengirim gambar ke server AI.
```

### Level yang Tersedia
| Level | Kapan dipakai |
|-------|---------------|
| `INFO` | Aktivitas normal (buka halaman, klik tombol, sukses) |
| `WARNING` | Hal tidak normal tapi app masih jalan (internet putus, izin ditolak) |
| `ERROR` | Sesuatu gagal (koneksi gagal, data tidak termuat) |

### Cara Pakai (Frontend Vue)
```typescript
import { invoke } from '@tauri-apps/api/core';

// ✅ Timestamp otomatis ditambahkan backend — kamu tidak perlu kirim waktu
await invoke('write_local_log_command', {
  level: 'INFO',
  message: 'User membuka halaman kamera'
});

await invoke('write_local_log_command', {
  level: 'WARNING',
  message: 'User menolak izin akses kamera'
});

await invoke('write_local_log_command', {
  level: 'ERROR',
  message: 'Gagal memuat daftar harga dari server'
});
```

### Kapan Wajib Dipanggil (Frontend)
```
✅ Pakai write_local_log_command untuk:
   - User buka halaman scan / kamera / profil
   - Tombol scan ditekan
   - Izin kamera ditolak user
   - Internet putus saat fetch data
   - Error ringan yang tidak melibatkan uang

❌ JANGAN pakai untuk:
   - Scan berhasil dan saldo bertambah  → pakai create_log_command
   - Transaksi penarikan uang          → pakai create_log_command
   - Perubahan harga oleh admin        → pakai create_log_command
```

---

## 🌐 Online Log — `create_log_command`

Tersimpan di tabel `log_system` Supabase. Bisa dilihat langsung dari dashboard Supabase.

### Cara Pakai (Frontend Vue)
```typescript
import { invoke } from '@tauri-apps/api/core';

// ✅ Log saat scan AI berhasil
await invoke('create_log_command', {
  level: 'INFO',
  message: 'Scan berhasil: AI mendeteksi Kardus, estimasi Rp 1750'
});

// ✅ Log saat transaksi gagal
await invoke('create_log_command', {
  level: 'ERROR',
  message: 'Gagal menyimpan hasil scan ke database'
});
```

> **Catatan:** `user_id`, `id`, dan `created_at` ditambahkan otomatis oleh backend.

---

## 📖 Membaca Local Log

### Cara 1 — Via ADB (Paling Cepat untuk Developer)

> ⚠️ **Windows:** `adb` tidak otomatis dikenali. Gunakan path lengkap atau jalankan lewat Android Studio Terminal.

**Opsi A — Path Lengkap (PowerShell):**
```powershell
# Simpan ke variabel biar tidak panjang
$adb = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"

# Cek HP/emulator terdeteksi
& $adb devices

# Baca log langsung di terminal
& $adb shell "run-as com.users.scantrash cat scantrash_local.log"

# Simpan ke file .txt di komputer
& $adb shell "run-as com.users.scantrash cat scantrash_local.log" > log_hp.txt

# Filter hanya baris ERROR
& $adb shell "run-as com.users.scantrash cat scantrash_local.log" | Select-String "[ERROR]"
```

**Opsi B — Tambah ADB ke PATH (Sekali saja, permanen):**
```powershell
# Jalankan sekali di PowerShell sebagai Administrator
$sdkPath = "$env:LOCALAPPDATA\Android\Sdk\platform-tools"
[System.Environment]::SetEnvironmentVariable("Path", $env:Path + ";$sdkPath", "User")
# Restart terminal, lalu bisa pakai 'adb' langsung
```

**Setelah PATH ditambahkan, cukup pakai:**
```powershell
adb devices
adb shell "run-as com.users.scantrash cat scantrash_local.log"
adb shell "run-as com.users.scantrash cat scantrash_local.log" > log_hp.txt
```


### Cara 2 — Dari Dalam App (Vue Component)

**Composable untuk baca log:**
```typescript
// composables/useLocalLog.ts
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface LogEntry {
  timestamp: string;  // "2026-05-02 13:45:01"
  level: 'INFO' | 'WARNING' | 'ERROR';
  message: string;
}

export function useLocalLog() {
  const logs = ref<LogEntry[]>([]);
  const isLoading = ref(false);

  // Parse satu baris: "[2026-05-02 13:45:01] [INFO] pesan"
  function parseLine(line: string): LogEntry | null {
    const match = line.match(/^\[(.+?)\] \[(.+?)\] (.+)$/);
    if (!match) return null;
    return {
      timestamp: match[1],
      level: match[2] as LogEntry['level'],
      message: match[3],
    };
  }

  async function loadLogs() {
    isLoading.value = true;
    try {
      const raw = await invoke<string>('read_local_log_command');
      logs.value = raw
        .split('\n')
        .filter(line => line.trim() !== '')
        .map(parseLine)
        .filter((entry): entry is LogEntry => entry !== null)
        .reverse(); // terbaru di atas
    } catch (e) {
      console.error('Gagal membaca log:', e);
    } finally {
      isLoading.value = false;
    }
  }

  return { logs, isLoading, loadLogs };
}
```

**Contoh tampilan di halaman Vue:**
```vue
<template>
  <div>
    <button @click="loadLogs">🔄 Refresh Log</button>

    <!-- Filter by level -->
    <select v-model="filterLevel">
      <option value="">Semua</option>
      <option value="INFO">INFO</option>
      <option value="WARNING">WARNING</option>
      <option value="ERROR">ERROR</option>
    </select>

    <p v-if="isLoading">Memuat log...</p>

    <div v-for="log in filteredLogs" :key="log.timestamp + log.message">
      <!-- Warna berbeda per level -->
      <span :style="{ color: levelColor(log.level) }">
        [{{ log.timestamp }}] [{{ log.level }}] {{ log.message }}
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted } from 'vue';
import { useLocalLog } from '@/composables/useLocalLog';

const { logs, isLoading, loadLogs } = useLocalLog();
const filterLevel = ref('');

const filteredLogs = computed(() =>
  filterLevel.value
    ? logs.value.filter(l => l.level === filterLevel.value)
    : logs.value
);

function levelColor(level: string) {
  return { INFO: 'green', WARNING: 'orange', ERROR: 'red' }[level] ?? 'gray';
}

onMounted(() => loadLogs());
</script>
```


---

## 🗺️ Contoh Alur Lengkap — Halaman Scan

```typescript
// 1. User buka halaman scan
await invoke('write_local_log_command', { level: 'INFO', message: 'User membuka halaman scan' });

// 2. User klik tombol kamera
await invoke('write_local_log_command', { level: 'INFO', message: 'Tombol kamera ditekan' });

// 3. Proses scan
try {
  const hasil = await invoke<ScanResult[]>('scan_trash', { image: imageBase64 });

  // 4a. Sukses → log ONLINE (karena melibatkan data penting)
  await invoke('create_log_command', {
    level: 'INFO',
    message: `Scan sukses: ${hasil.length} item terdeteksi`
  });

} catch (error) {
  // 4b. Gagal → log LOKAL (error teknis, bukan transaksi)
  await invoke('write_local_log_command', {
    level: 'ERROR',
    message: `Scan gagal: ${String(error)}`
  });
}
```

---

## ⚡ Ringkasan Cepat

| Situasi | Command |
|---------|---------|
| Buka halaman | `write_local_log_command` → INFO |
| Klik tombol penting | `write_local_log_command` → INFO |
| Internet putus | `write_local_log_command` → WARNING |
| Izin kamera ditolak | `write_local_log_command` → WARNING |
| Fetch data gagal | `write_local_log_command` → ERROR |
| **Scan AI berhasil** | **`create_log_command` → INFO** |
| **Transaksi gagal** | **`create_log_command` → ERROR** |
| **Admin ubah harga** | **`create_log_command` → INFO** |

---

## 🗂️ Peta Implementasi Log Backend (Sudah Otomatis)

> **Penting untuk tim:** Semua log di bagian ini sudah berjalan **otomatis** oleh backend Rust. **Frontend tidak perlu melakukan apapun** untuk 27 titik ini.

### 🔐 Auth — `auth_handler.rs`

| # | Kejadian | Level | Dipicu Oleh |
|---|----------|:-----:|-------------|
| 1 | Token ditemukan & valid saat cek sesi | `INFO` | `check_auth_status_command` |
| 2 | Token expired → auto-refresh **berhasil** | `INFO` | `check_auth_status_command` |
| 3 | Token expired → auto-refresh **gagal** | `WARNING` | `check_auth_status_command` |
| 4 | User login via Google OAuth berhasil | `INFO` | Deep link OAuth callback |
| 5 | UI belum siap menerima sinyal login | `WARNING` | Deep link OAuth callback |
| 6 | User berhasil logout | `INFO` | `logout_command` |

### 👤 Profil — `profile_handler.rs`

| # | Kejadian | Level | Dipicu Oleh |
|---|----------|:-----:|-------------|
| 7 | Data profil berhasil dimuat (username dicatat) | `INFO` | `get_profile_command` |
| 8 | Gagal ambil profil dari server | `ERROR` | `get_profile_command` |
| 9 | Akses ditolak: tidak ada sesi aktif | `WARNING` | `get_profile_command` |

### 🤖 AI Hugging Face — `api.rs`

| # | Kejadian | Level | Dipicu Oleh |
|---|----------|:-----:|-------------|
| 10 | HF_Token tidak ditemukan di `.env` | `ERROR` | `scan_trash` |
| 11 | Gambar terlalu besar — ditolak (DoS prevention) | `WARNING` | `scan_trash` |
| 12 | Base64 non-ASCII — ditolak (payload injection) | `WARNING` | `scan_trash` |
| 13 | Format gambar bukan jpeg/png/webp — ditolak | `WARNING` | `scan_trash` |
| 14 | Gagal kirim request ke API Hugging Face | `ERROR` | `scan_trash` |
| 15 | HF API balas dengan error (4xx/5xx) | `ERROR` | `scan_trash` |
| 16 | Gagal parsing JSON dari respons HF | `ERROR` | `scan_trash` |
| 17 | Format respons HF tidak dikenali | `ERROR` | `scan_trash` |

### 🧠 AI Service — `ai_services.rs`

| # | Kejadian | Level | Dipicu Oleh |
|---|----------|:-----:|-------------|
| 18 | SUPABASE_URL belum diset di `.env` | `ERROR` | `scan_trash` |
| 19 | Gagal ambil `rules` dari Supabase | `ERROR` | `scan_trash` |
| 20 | Gagal parsing JSON `rules` | `ERROR` | `scan_trash` |
| 21 | Gagal ambil `pricelist` dari Supabase | `ERROR` | `scan_trash` |
| 22 | Gagal parsing JSON `pricelist` | `ERROR` | `scan_trash` |
| 23 | Hasil mentah teks dari AI (isi respons AI asli) | `INFO` | `scan_trash` |

### 💾 Database — `scan_service.rs`

| # | Kejadian | Level | Dipicu Oleh |
|---|----------|:-----:|-------------|
| 24 | URL atau Key Supabase kosong di `.env` | `ERROR` | `scan_trash` |
| 25 | Koneksi ke DB Supabase gagal saat simpan scan | `ERROR` | `scan_trash` |
| 26 | Server DB menolak insert data scan | `ERROR` | `scan_trash` |
| 27 | Berhasil menyimpan N item scan ke database | `INFO` | `scan_trash` |

---
