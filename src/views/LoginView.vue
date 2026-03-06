<script setup lang="ts">
import { ref } from 'vue'
import { useAuthStore } from '../stores/authStore'
import { useRouter } from 'vue-router'

const email = ref('')
const password = ref('')
const authStore = useAuthStore()
const router = useRouter()

const handleLogin = async () => {
  try {
    await authStore.login({
      email: email.value,
      password: password.value
    })
    
    if (authStore.isAuthenticated) {
      router.push('/')
    }
  } catch (error) {
    console.error('Login error:', error)
  }
}
</script>

<template>
  <form @submit.prevent="handleLogin">
    <input v-model="email" type="email" placeholder="Email" />
    <input v-model="password" type="password" placeholder="Password" />
    <button :disabled="authStore.loading">
      {{ authStore.loading ? 'Loading...' : 'Login' }}
    </button>
    <p v-if="authStore.error" class="error">{{ authStore.error }}</p>
  </form>
</template>