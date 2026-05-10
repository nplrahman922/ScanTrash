<script setup lang="ts">
import { useAuthStore } from '../stores/authStore'
  
/* gambar‑gambar dari assets */
import logo from '../assets/Logo1.svg'
import googleIcon from '../assets/google.svg'

const authStore = useAuthStore()

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
  <div class="min-h-screen flex flex-col items-center justify-center" style="background-color: var(--color-tertiary);">
    <!-- logo di atas card -->
    <img :src="logo" alt="ScanTrash logo" class="w-32 h-32 mb-8" />

    <!-- card hijau -->
    <div class="rounded-xl p-8 w-full max-w-sm text-center shadow-lg" style="background-color: var(--color-primary);">
      <h1 class="text-white text-2xl font-semibold mb-6">
        Welcome to ScanTrash
      </h1>

      <!-- tombol Google -->
      <button
        @click="handleLogin"
        :disabled="authStore.loading"
        class="google-btn flex items-center justify-center rounded-full px-4 py-2 w-full transition-colors"
      >
        <img :src="googleIcon" alt="Google" class="w-6 h-6 mr-2" />
        <span class="font-medium">
          {{ authStore.loading ? 'Loading…' : 'Sign in with Google' }}
        </span>
      </button>

      <p class="text-white text-sm mt-4">
        Mari kita mulai untuk menjaga kebersihan bersama!
      </p>

      <p v-if="authStore.error" class="text-sm mt-2" style="color: var(--color-secondary);">
        {{ authStore.error }}
      </p>
    </div>
  </div>
</template>

<style scoped>
.google-btn {
  background-color: white;
  color: #333;
}

.google-btn:hover {
  background-color: #f0f0f0;
}

.google-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>