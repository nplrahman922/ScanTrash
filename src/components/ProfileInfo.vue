<script setup lang="ts">
import { ref, computed } from "vue"
import { storeToRefs } from "pinia"
import { useAuthStore } from "../stores/authStore"
import BaseButton from "../components/BaseButton.vue"

const authStore = useAuthStore()
const { userProfile } = storeToRefs(authStore)

const isAdmin = computed(() => userProfile.value?.role === "admin")

const isProfileOpen = ref(false)

const toggleProfile = () => {
  isProfileOpen.value = !isProfileOpen.value
}

const closeProfile = () => {
  isProfileOpen.value = false
}
</script>

<template>
  <div class="relative">
    
    <!-- Avatar -->
    <img
      class="cursor-pointer w-12 h-12 rounded-full border-2 border-gray-300"
      :src="userProfile?.photo_url || '/placeholder.svg'"
      alt="Profile"
      @click="toggleProfile"
    />

    <!-- Popup Profile -->
    <div
      v-if="userProfile && isProfileOpen"
      class="absolute right-0 top-14 w-72 bg-white rounded-xl p-6 shadow-lg border z-50"
    >
      <h2 class="text-xl font-bold text-gray-800 mb-4">
        {{ isAdmin ? "Informasi Admin" : "Informasi Profil" }}
      </h2>

      <!-- Info (row) -->
      <div class="flex items-center space-x-4">
        
        <!-- Foto -->
        <img
          v-if="userProfile.photo_url"
          :src="userProfile.photo_url"
          alt="Profile"
          :class="[
            'w-16 h-16 rounded-full border-2',
            isAdmin ? 'border-purple-500' : 'border-green-500'
          ]"
        />

        <!-- Default -->
        <div
          v-else
          class="w-16 h-16 rounded-full bg-gray-300 flex items-center justify-center"
        >
          <span class="text-2xl">
            {{ isAdmin ? "👑" : "👤" }}
          </span>
        </div>

        <!-- Text -->
        <div>
          <p class="font-semibold text-gray-800">
            {{ userProfile.username }}
          </p>

          <p class="text-gray-600 text-sm">
            {{ userProfile.email }}
          </p>

          <span
            :class="[
              'inline-block mt-1 px-2 py-1 text-xs font-bold rounded-full',
              isAdmin
                ? 'bg-purple-100 text-purple-700'
                : 'bg-blue-100 text-blue-700'
            ]"
          >
            {{ userProfile.role }}
          </span>
        </div>
      </div>

      <!-- Button (bawah) -->
      <div class="mt-4">
        <BaseButton
          label="Tutup"
          class="w-full bg-green-500 text-white hover:bg-green-600"
          @click="closeProfile"
        />
      </div>

    </div>
  </div>
</template>