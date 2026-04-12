<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { listen } from '@tauri-apps/api/event' // 🔥 Wajib di-import
import { useAuthStore } from '../stores/authStore'
  
/* gambar‑gambar dari assets */
import logo from '../assets/Logo1.svg'
import googleIcon from '../assets/google.svg'

const authStore = useAuthStore()
const router = useRouter()
let unlistenLogin: () => void

// 🔥 PASANG TELINGA SAAT HALAMAN DIBUKA
onMounted(async () => {
  try {
    // Mendengarkan sinyal dari Backend Rust
    unlistenLogin = await listen('login-success', async (event) => {
      console.log("🔊 Sinyal dari Rust diterima:", event.payload)

      // 1. Suruh Pinia mengecek status brankas Rust
      const isLoggedIn = await authStore.checkAuthStatus()

      if (isLoggedIn) {
        console.log("Brankas terkonfirmasi! Pindah ke Dashboard!")
        // 2. Pindah halaman ke Dashboard
        router.push('/user-dashboard')
      }
    })
  } catch (error) {
    console.error("Gagal memasang pendengar sinyal:", error)
  }
})

// Bersihkan memori saat pindah halaman
onUnmounted(() => {
  if (unlistenLogin) {
    unlistenLogin()
  }
})

const handleLogin = async () => {
  try {
    await authStore.googleLogin()
    // Auth store akan handle redirect setelah login success
  } catch (error) {
    console.error('Login error:', error)
    authStore.setError(error instanceof Error ? error.message : 'An unknown error occurred')
  }
}
</script>

<template>
  <div class="min-h-screen flex flex-col items-center justify-center bg-gray-100">
    <!-- logo di atas card -->
    <img :src="logo" alt="ScanTrash logo" class="w-32 h-32 mb-8" />

    <!-- card hijau -->
    <div class="bg-green-500 rounded-xl p-8 w-full max-w-sm text-center shadow-lg">
      <h1 class="text-white text-2xl font-semibold mb-6">
        Welcome to ScanTrash
      </h1>

      <!-- tombol Google -->
      <button
        @click="handleLogin"
        :disabled="authStore.loading"
        class="flex items-center justify-center bg-white rounded-full px-4 py-2 w-full hover:bg-gray-100 transition-colors"
      >
        <img :src="googleIcon" alt="Google" class="w-6 h-6 mr-2" />
        <span class="text-gray-800 font-medium">
          {{ authStore.loading ? 'Loading…' : 'Sign in with Google' }}
        </span>
      </button>

      <p class="text-white text-sm mt-4">
        Mari kita mulai untuk menjaga kebersihan bersama!
      </p>

      <p v-if="authStore.error" class="text-red-200 text-sm mt-2">
        {{ authStore.error }}
      </p>
    </div>
  </div>
</template>

<style scoped>
/* style minimal; mayoritas dikerjakan oleh Tailwind */
</style>