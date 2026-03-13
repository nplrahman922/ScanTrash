<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useRouter } from 'vue-router';
import { computed } from 'vue'
import { useAuthStore } from './stores/authStore'

import Header from './components/Header.vue'
import BottomNav from './components/BottomNav.vue'

const authStore = useAuthStore()
const router = useRouter()
const isAuth = computed(() => authStore.isAuthenticated)

const isLoading = ref(true);

async function fetchProfileAndRoute() {
  try {
    const profile = await authStore.fetchProfile();

    if (profile.role === 'admin') {
      router.push('/admin-dashboard');
    } else {
      router.push('/user-dashboard');
    }
  } catch (err) {
    console.error("Profile Error:", err);
    router.push('/login');
  } finally {
    isLoading.value = false;
  }
}

onMounted(async () => {
  try {
    const isLoggedIn = await authStore.checkAuthStatus();
    if (isLoggedIn) {
      await fetchProfileAndRoute();
    } else {
      router.push('/login');
      isLoading.value = false;
    }
  } catch (err) {
    console.error("Gagal cek status:", err);
    router.push('/login');
    isLoading.value = false;
  }

  // Listen event login-success dari deep link
  listen('login-success', async (event) => {
    isLoading.value = true;
    await fetchProfileAndRoute();
  });
});
</script>

<template>
  <div class="min-h-screen flex flex-col">
    <!-- Header tetap tampil kalau sudah login -->
    <Header v-if="isAuth" />

    <main class="content-area bg-[#FFFFFF] grow">
      <!-- Loading spinner saat check auth -->
      <div v-if="isLoading" class="flex justify-center items-center min-h-screen">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-green-600"></div>
      </div>

      <!-- Router view untuk semua halaman -->
      <router-view v-else />
    </main>

    <BottomNav v-if="isAuth" />
  </div>
</template>