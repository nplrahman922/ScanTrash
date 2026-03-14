<script setup lang="ts">
import { useRouter } from "vue-router"
import { useAuthStore } from "../stores/authStore"

const authStore = useAuthStore()
const router = useRouter()

const handleLogout = async () => {
  await authStore.logout()
  router.push("/login")
}
</script>

<template>
  <button
    @click="handleLogout"
    :disabled="authStore.loading"
    class="bg-red-500 text-white px-4 py-2 rounded-lg hover:bg-red-600 transition"
  >
    {{ authStore.loading ? "Logging out..." : "Logout" }}
  </button>
</template>