<template>
  <div class="min-h-screen bg-gray-50 p-4">
    <div class="max-w-md mx-auto">
      <div class="bg-white rounded-xl p-6 shadow-md mb-6">
        <h1 class="text-2xl font-bold text-gray-800 mb-4">Dashboard User</h1>
        <p class="text-gray-600 mb-4">Selamat datang di dashboard pengguna ScanTrash!</p>

        <div class="space-y-4">
          <div class="bg-green-50 p-4 rounded-lg">
            <h3 class="font-semibold text-green-800">Scan Sampah</h3>
            <p class="text-green-600 text-sm">Gunakan kamera untuk scan sampah Anda</p>
          </div>

          <div class="bg-blue-50 p-4 rounded-lg">
            <h3 class="font-semibold text-blue-800">Riwayat Scan</h3>
            <p class="text-blue-600 text-sm">Lihat riwayat scan dan tabungan Anda</p>
          </div>

          <div class="bg-purple-50 p-4 rounded-lg">
            <h3 class="font-semibold text-purple-800">Saldo Tabungan</h3>
            <p class="text-purple-600 text-sm">Cek saldo tabungan dari sampah yang dikumpulkan</p>
          </div>
        </div>
      </div>

      <!-- Profile Info -->
      <div v-if="authStore.userProfile" class="bg-white rounded-xl p-6 shadow-md">
        <h2 class="text-xl font-bold text-gray-800 mb-4">Informasi Profil</h2>
        <div class="flex items-center space-x-4">
          <img
            v-if="authStore.userProfile.photo_url"
            :src="authStore.userProfile.photo_url"
            alt="Profile"
            class="w-16 h-16 rounded-full border-2 border-green-500"
          />
          <div v-else class="w-16 h-16 rounded-full bg-gray-300 flex items-center justify-center">
            <span class="text-2xl">👤</span>
          </div>
          <div>
            <p class="font-semibold text-gray-800">{{ authStore.userProfile.username }}</p>
            <p class="text-gray-600 text-sm">{{ authStore.userProfile.email }}</p>
            <span class="inline-block mt-1 px-2 py-1 text-xs font-bold rounded-full bg-blue-100 text-blue-700">
              {{ authStore.userProfile.role }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '../stores/authStore'

const authStore = useAuthStore()
</script>