<script setup>
import { onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';
import { onOpenUrl } from '@tauri-apps/plugin-deep-link';
import { load } from '@tauri-apps/plugin-store';

// Fungsi dipanggil saat tombol "Login Google" diklik
async function loginWithGoogle() {
  try {
    const authUrl = await invoke('get_google_auth_url_command');
    // Gunakan openUrl di sini
    await openUrl(authUrl); 
  } catch (error) {
    console.error("Gagal membuka auth URL:", error);
  }
}

// Fungsi untuk membedah URL dari Supabase
async function handleSupabaseCallback(url) {
  console.log("Deeplink URL masuk:", url);
  
  // Supabase mengirim token di bagian hash (#access_token=...)
  // Kita harus mengekstraknya
  const hashString = url.split('#')[1];
  if (!hashString) return;

  const urlParams = new URLSearchParams(hashString);
  const accessToken = urlParams.get('access_token');
  const refreshToken = urlParams.get('refresh_token');

  if (accessToken) {
    console.log("Token berhasil ditangkap!");
    
    // Simpan ke Native Persistent Storage
    const store = await load('session.json', { autoSave: true });
    await store.set('access_token', accessToken);
    await store.set('refresh_token', refreshToken);
    
    // TODO: Arahkan user ke halaman Dashboard Bank Sampah kamu
    // misal: router.push('/dashboard');
  }
}

onMounted(async () => {
  await onOpenUrl((urls) => {
    for (const url of urls) {
      // Tanpa /callback
      if (url.startsWith('com.users.scantrash://auth')) {
        handleSupabaseCallback(url);
      }
    }
  });
});
</script>

<template>
  <div class="p-4">
    <button @click="loginWithGoogle" class="bg-red-500 text-white p-2 rounded">
      Login with Google
    </button>
  </div>
</template>