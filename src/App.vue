<script setup lang="ts">
import { computed } from 'vue'
import { useAuthStore } from './stores/authStore'

import Header from './components/Header.vue'
import BottomNav from './components/BottomNav.vue'
import LoginView from './views/LoginView.vue'      // import login

const authStore = useAuthStore()
const isAuth = computed(() => authStore.isAuthenticated)
</script>

<template>
  <div class="min-h-screen flex flex-col">
    <!-- header tetap tampil kalau sudah login -->
    <Header v-if="isAuth" />

    <main class="content-area bg-[#FFFFFF] grow">
      <!-- kalau belum login, tampilkan login view,
           kalau sudah, pakai router-view untuk sisa halaman -->
      <LoginView v-if="!isAuth" />
      <router-view v-else />
    </main>

    <BottomNav v-if="isAuth" />
  </div>
</template>