<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';
import { listen } from '@tauri-apps/api/event';

const statusText = ref("Sedang mengecek sesi...");
const userProfile = ref(null); // Menyimpan data profil dari database

// Fungsi baru untuk mengambil profil dan menentukan arah (Admin/User)
async function fetchProfileAndRoute() {
  try {
    statusText.value = "Mengambil data profil...";
    const profile = await invoke('get_profile_command');
    
    // Simpan ke state Vue
    userProfile.value = profile;

    // Logika Navigasi (Menjawab revisi tim)
    if (profile.role === 'admin') {
      statusText.value = "Sesi Aktif (Akses: ADMIN) 👑";
      console.log("Mengarahkan ke Dashboard Admin...");
      // router.push('/admin-dashboard'); 
    } else {
      statusText.value = "Sesi Aktif (Akses: USER) 👤";
      console.log("Mengarahkan ke Dashboard User...");
      // router.push('/user-dashboard');
    }
  } catch (err) {
    statusText.value = "Gagal mengambil profil. Silakan login ulang.";
    console.error("Profile Error:", err);
  }
}

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
  // 1. Cek status token di backend saat aplikasi dibuka
  try {
    const isLoggedIn = await invoke('check_auth_status_command');
    if (isLoggedIn) {
      // Jika token aman, tarik data profilnya!
      await fetchProfileAndRoute();
    } else {
      statusText.value = "Belum Login";
    }
  } catch (err) {
    console.error("Gagal cek status:", err);
    statusText.value = "Belum Login";
  }

  // 2. Pasang telinga untuk event balikan dari Browser (Deep Link)
  await listen('login-success', async (event) => {
    // Saat login sukses, langsung tarik profilnya
    await fetchProfileAndRoute();
  });
});

async function handleLogout() {
  try {
    statusText.value = "Sedang proses logout...";
    
    // Perintahkan Rust untuk menghancurkan token di RAM, Disk, dan Server
    await invoke('logout_command');
    
    // Update UI kembali ke mode awal
    statusText.value = "Belum Login";
    userProfile.value = null; // Kosongkan data profil
    
    // router.push('/login');
    
  } catch (error) {
    console.error("Gagal logout:", error);
  }
}
</script>

<template>
  <div class="p-4 flex flex-col gap-4 mt-10 max-w-md mx-auto">
    <h1 class="text-2xl font-bold text-center">ScanTrash App</h1>
    
    <button 
      v-if="!userProfile" 
      @click="loginWithGoogle" 
      class="bg-green-600 hover:bg-green-700 text-white p-3 rounded-lg font-semibold w-full transition"
    >
      Login with Google
    </button>
    
    <div v-if="userProfile" class="p-5 bg-white rounded-xl shadow-md border border-gray-200 mt-4 flex flex-col items-center">
      <img 
        v-if="userProfile.photo_url" 
        :src="userProfile.photo_url" 
        alt="Profile Picture" 
        class="w-20 h-20 rounded-full border-2 border-green-500 mb-3"
      />
      <div v-else class="w-20 h-20 rounded-full bg-gray-300 flex items-center justify-center mb-3">
        <span class="text-2xl">👤</span>
      </div>

      <h2 class="text-xl font-bold text-gray-800">{{ userProfile.username }}</h2>
      <p class="text-gray-500 text-sm">{{ userProfile.email }}</p>
      
      <span 
        class="mt-2 px-3 py-1 text-xs font-bold rounded-full uppercase"
        :class="userProfile.role === 'admin' ? 'bg-purple-100 text-purple-700' : 'bg-blue-100 text-blue-700'"
      >
        {{ userProfile.role }}
      </span>
    </div>

    <button 
      v-if="userProfile" 
      @click="handleLogout" 
      class="bg-red-500 hover:bg-red-600 text-white p-3 rounded-lg font-semibold w-full transition"
    >
      Logout
    </button>

    <div class="p-4 bg-gray-50 rounded-lg border border-gray-200 text-center mt-4 shadow-sm">
      <p class="text-xs font-bold text-gray-400 uppercase tracking-wider">Status API Log</p>
      <p class="text-gray-700 font-medium mt-1">{{ statusText }}</p>
    </div>
  </div>
</template>