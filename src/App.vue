<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';
import { listen } from '@tauri-apps/api/event';
// import { useRouter } from 'vue-router'; // <-- Nanti buka comment ini kalau Vue Router sudah siap

const statusText = ref("Sedang mengecek sesi...");
const userProfile = ref(null);
const isLoading = ref(true); // <-- TAMBAHAN: State untuk menahan UI saat awal buka

// const router = useRouter(); // <-- Nanti buka comment ini

async function fetchProfileAndRoute() {
  try {
    statusText.value = "Mengambil data profil...";
    const profile = await invoke('get_profile_command');
    
    userProfile.value = profile;

    if (profile.role === 'admin') {
      statusText.value = "Sesi Aktif (Akses: ADMIN) 👑";
      // router.push('/admin-dashboard'); 
    } else {
      statusText.value = "Sesi Aktif (Akses: USER) 👤";
      // router.push('/user-dashboard');
    }
  } catch (err) {
    statusText.value = "Gagal mengambil profil. Silakan login ulang.";
    console.error("Profile Error:", err);
  } finally {
    isLoading.value = false; // <-- Matikan loading setelah selesai
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
  try {
    const isLoggedIn = await invoke('check_auth_status_command');
    if (isLoggedIn) {
      await fetchProfileAndRoute();
    } else {
      statusText.value = "Belum Login";
      isLoading.value = false; // <-- Matikan loading jika ternyata belum login
    }
  } catch (err) {
    console.error("Gagal cek status:", err);
    statusText.value = "Belum Login";
    isLoading.value = false;
  }

  await listen('login-success', async (event) => {
    isLoading.value = true; // Nyalakan loading saat data balikan masuk
    await fetchProfileAndRoute();
  });
});

async function handleLogout() {
  try {
    isLoading.value = true; // Nyalakan loading saat proses logout
    statusText.value = "Sedang proses logout...";
    
    await invoke('logout_command');
    
    statusText.value = "Belum Login";
    userProfile.value = null; 
    
    // router.push('/login');
  } catch (error) {
    console.error("Gagal logout:", error);
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <div class="p-4 flex flex-col gap-4 mt-10 max-w-md mx-auto">
    <h1 class="text-2xl font-bold text-center">ScanTrash App</h1>
    
    <div v-if="isLoading" class="flex justify-center p-5">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-green-600"></div>
    </div>

    <template v-else>
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
        class="bg-red-500 hover:bg-red-600 text-white p-3 rounded-lg font-semibold w-full transition mt-4"
      >
        Logout
      </button>
    </template>

    <div class="p-4 bg-gray-50 rounded-lg border border-gray-200 text-center mt-4 shadow-sm">
      <p class="text-xs font-bold text-gray-400 uppercase tracking-wider">Status API Log</p>
      <p class="text-gray-700 font-medium mt-1">{{ statusText }}</p>
    </div>
  </div>
</template>