<script setup>
import { computed } from 'vue'
import { useAuthStore } from './stores/authStore'

import Header from './components/Header.vue'
import Footer from './components/Footer.vue'

const authStore = useAuthStore()
const isAuth = computed(() => authStore.isAuthenticated)
</script>

<template>
  <div class="min-h-screen flex flex-col">
    <Header v-if="isAuth" />

    <main class="content-area bg-[#FFFFFF] grow">
      <router-view />
    </main>

    <Footer v-if="isAuth" />
  </div>
</template>