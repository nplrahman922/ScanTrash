<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';
import { listen } from '@tauri-apps/api/event';

const statusText = ref("Sedang mengecek sesi...");

async function loginWithGoogle() {
  try {
    statusText.value = "Membuka browser...";
    const authUrl = await invoke('get_google_auth_url_command');
    await openUrl(authUrl); 
  } catch (error) {
    statusText.value = "Error: " + error;
  }
}

onMounted(async () => {
  // 1. TANYA LANGSUNG KE BACKEND: "Apakah kamu pegang token?"
  try {
    const isLoggedIn = await invoke('check_auth_status_command');
    if (isLoggedIn) {
      statusText.value = "Sesi Aktif (Token aman di Backend)!";
      // router.push('/dashboard');
    } else {
      statusText.value = "Belum Login";
    }
  } catch (err) {
    console.error("Gagal cek status:", err);
  }

  // 2. Tetap pasang telinga (buat jaga-jaga kalau HP-nya sangat cepat dan Vite tidak me-refresh UI)
  await listen('login-success', (event) => {
    statusText.value = event.payload; 
  });
});

async function handleLogout() {
  try {
    statusText.value = "Sedang proses logout...";
    
    // 1. Perintahkan Rust untuk menghancurkan semua token
    await invoke('logout_command');
    
    // 2. Update UI
    statusText.value = "Belum Login";
    
    // Kalau sudah pakai Vue Router, arahkan ke halaman login:
    // router.push('/login');
    
  } catch (error) {
    console.error("Gagal logout:", error);
  }
}


</script>

<template>
  <div class="p-4 flex flex-col gap-4 mt-10">
    <h1 class="text-2xl font-bold text-center">Aplikasi Bank Sampah</h1>
    
    <button @click="loginWithGoogle" class="bg-green-600 hover:bg-green-700 text-white p-3 rounded-lg font-semibold w-full">
      Login with Google
    </button>
    
    <div class="p-4 flex flex-col gap-4 mt-10">
      <button @click="handleLogout" class="bg-red-600 hover:bg-red-700 text-white p-3 rounded-lg font-semibold w-full mt-4">
        Logout
      </button>
    </div>

    <div class="p-4 bg-gray-100 rounded-lg border border-gray-300 text-center">
      <p class="font-bold text-gray-700">Status API:</p>
      <p class="text-blue-600 font-semibold mt-1">{{ statusText }}</p>
    </div>
  </div>
</template>