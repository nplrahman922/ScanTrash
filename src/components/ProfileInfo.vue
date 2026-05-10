<script setup lang="ts">
import { ref } from "vue"
import { storeToRefs } from "pinia"
import { useAuthStore } from "../stores/authStore"
import BaseButton from "../components/BaseButton.vue"

const authStore = useAuthStore()
const { userProfile } = storeToRefs(authStore)

const isProfileOpen = ref(false)

const toggleProfile = () => {
  isProfileOpen.value = !isProfileOpen.value
}

const closeProfile = () => {
  isProfileOpen.value = false
}
</script>

<template>
  <!-- Avatar Button -->
  <img
    class="cursor-pointer w-12 h-12 rounded-full border-2"
    :style="{ borderColor: 'var(--color-primary)' }"
    :src="userProfile?.photo_url || '/placeholder.svg'"
    alt="Profile"
    @click="toggleProfile"
  />

  <!-- Overlay Backdrop -->
  <div
    v-if="isProfileOpen"
    class="fixed inset-0 bg-black bg-opacity-50 z-40"
    @click="closeProfile"
  />

  <!-- Profile Card Popup -->
  <div
    v-if="isProfileOpen && userProfile"
    class="fixed inset-0 flex items-center justify-center z-50 p-4"
  >
    <div
      class="bg-white rounded-2xl p-8 w-full max-w-sm shadow-2xl border-2"
      :style="{ borderColor: 'var(--color-tertiary)' }"
      @click.stop
    >
      <!-- Profile Picture (Circular) -->
      <div class="flex justify-center mb-6">
        <img
          v-if="userProfile.photo_url"
          :src="userProfile.photo_url"
          alt="Profile"
          class="w-24 h-24 rounded-full border-4"
          :style="{ borderColor: 'var(--color-primary)' }"
        />
        <div
          v-else
          class="w-24 h-24 rounded-full flex items-center justify-center text-5xl"
          :style="{ backgroundColor: 'var(--color-secondary)' }"
        >
          👤
        </div>
      </div>

      <!-- Name -->
      <h2 class="text-center text-xl font-bold mb-2">
        {{ userProfile.username }}
      </h2>

      <!-- Email -->
      <p class="text-center text-gray-600 text-sm mb-2">
        {{ userProfile.email }}
      </p>

      <!-- Terkoneksi dengan -->
      <p class="text-center text-sm font-medium mb-4" style="color: var(--color-secondary);">
        Terhubung dengan Google
      </p>

      <!-- Divider -->
      <hr class="my-4" :style="{ borderColor: 'var(--color-tertiary)' }" />

      <!-- Info Section -->
      <div class="space-y-3 mb-6">
        <!-- Bergabung -->
        <div class="flex items-center space-x-3 text-gray-600">
          <span class="text-lg">📅</span>
          <div>
            <p class="text-xs text-gray-500">Bergabung</p>
            <p class="text-sm font-medium text-gray-800">
              30 Januari 2026
            </p>
          </div>
        </div>

        <!-- Status Akun -->
        <div class="flex items-center space-x-3">
          <span class="text-lg">👤</span>
          <div class="flex-1">
            <p class="text-xs text-gray-500">Status Akun</p>
            <p
              class="text-sm font-medium"
              :style="{ color: 'var(--color-primary)' }"
            >
              Aktif
            </p>
          </div>
        </div>
      </div>

      <!-- Close Button -->
      <BaseButton
        label="Tutup"
        class="w-full text-white font-semibold py-2 rounded-lg transition-all"
        :style="{ backgroundColor: 'var(--color-primary)' }"
        @click="closeProfile"
      />
    </div>
  </div>
</template>

<style scoped>
.border-2 {
  border-width: 2px;
}

.border-4 {
  border-width: 4px;
}
</style>