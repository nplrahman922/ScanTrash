<script setup lang="ts">
import { storeToRefs } from "pinia"
import { useScanStore } from "../stores/scanStore"

// import icon (placeholder dulu)
import ScanIcon from "../assets/user/Scan1.svg"

const scanStore = useScanStore()
const { loading, result, error } = storeToRefs(scanStore)

const handleScan = () => {
  scanStore.scanTrash()
}
</script>

<template>
  <div class="flex flex-col items-center justify-center min-h-screen bg-gray-100 p-4">
    
    <!-- Title -->
    <h1 class="text-2xl font-bold mb-6">Scan Sampah</h1>

    <!-- Scan Button -->
    <div
      class="w-40 h-40 rounded-full bg-blue-500 flex items-center justify-center shadow-lg cursor-pointer"
      @click="handleScan"
    >
      <img :src="ScanIcon" alt="Scan Icon"
        class="w-16 h-16 text-white"
        :class="{ 'animate-spin': loading }"
      />
    </div>

    <p class="mt-4 text-gray-600">
      Klik untuk memulai scan
    </p>

    <!-- Loading -->
    <div v-if="loading" class="mt-6 text-blue-500 font-semibold">
      Scanning...
    </div>

    <!-- Error -->
    <div v-if="error" class="mt-4 text-red-500">
      {{ error }}
    </div>

    <!-- Result -->
    <div
      v-if="result"
      class="mt-6 w-full max-w-sm bg-white p-4 rounded-xl shadow"
    >
      <h2 class="text-lg font-bold mb-2">Hasil Scan</h2>

      <p><strong>Nama:</strong> {{ result.name }}</p>
      <p><strong>Jenis:</strong> {{ result.type }}</p>
      <p><strong>Harga:</strong> Rp {{ result.price }}</p>
    </div>

  </div>
</template>