<script setup lang="ts">
import { storeToRefs } from "pinia"
import { useScanStore } from "../stores/scanStore"

import FokusIcon from "../assets/user/Fokus.svg"
import LoadingIcon from "../assets/user/Loading.svg"
import Logo from "../assets/Logo2.svg"
import ImageIcon from "../assets/user/Image.svg"
import FotoIcon from "../assets/user/Foto.svg"
import ExitIcon from "../assets/user/Exit2.svg"

const scanStore = useScanStore()
const { loading, result, error } = storeToRefs(scanStore)

const handleScan = () => {
  scanStore.scanTrash()
}

import { onMounted, ref } from "vue"

const videoRef = ref<HTMLVideoElement | null>(null)

onMounted(async () => {
  const stream = await navigator.mediaDevices.getUserMedia({
    video: { facingMode: "environment" } // kamera belakang HP
  })

  if (videoRef.value) {
    videoRef.value.srcObject = stream
  }
})
</script>

<template>
  <div class="flex flex-col min-h-screen bg-gray-100">

    <!-- 🔝 HEADER -->
    <div class="flex justify-center py-4">
      <img :src="Logo" alt="Logo" class="h-10" />
    </div>

    <!-- 📸 CAMERA FRAME -->
    <div class="flex justify-center px-4">
      <div
        class="relative w-full max-w-sm aspect-9/16 bg-black rounded-[40px] border-[6px] border-lime-400 overflow-hidden shadow-lg"
      >
        <!-- 🔴 nanti kamera masuk di sini -->
        <!-- contoh: <video ref="video" autoplay class="w-full h-full object-cover" /> -->
        <video
          ref="videoRef"
          autoplay
          playsinline
          class="w-full h-full object-cover"
        />

        <!-- Overlay fokus -->
        <div class="absolute inset-0 flex items-center justify-center">
          <img 
            :src="loading ? LoadingIcon : FokusIcon"
            :class="[
              'w-40 h-40 transition-all duration-300',
              loading && 'animate-spin'
            ]" 
          />
        </div>

        <!-- Text -->
        <div class="absolute bottom-10 px-6 text-center">
          <p class="text-[#FAA111] text-sm font-medium">
            {{ loading 
              ? "Menganalisis Sampah..." 
              : "Silakan fokuskan kamera pada sampah agar sistem dapat mendeteksi secara otomatis"
            }}
          </p>

          <p v-if="loading" class="text-[#FAA111] text-xs mt-2">
            AI sedang mengecek kelayakan sampah
          </p>
        </div>
      </div>
    </div>

    <!-- 🔘 BOTTOM ACTION -->
    <div class="mt-auto flex justify-between items-center px-8 py-6">
      <img :src="ImageIcon" class="w-14 h-14" />

      <!-- tombol scan -->
      <button
        @click="handleScan"
        :disabled="loading"
      >
      <img :src="FotoIcon" class="w-14 h-14" />
      </button>

      <img :src="ExitIcon" class="w-14 h-14" />
    </div>
  </div>
</template>