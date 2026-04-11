<script setup lang="ts">
import { storeToRefs } from "pinia"
import { useScanStore } from "../stores/scanStore"
import { onMounted, onUnmounted, ref } from "vue"

import FokusIcon from "../assets/user/Fokus.svg"
import LoadingIcon from "../assets/user/Loading.svg"
import Logo from "../assets/Logo2.svg"
import ImageIcon from "../assets/user/Image.svg"
import FotoIcon from "../assets/user/Foto.svg"
import ExitIcon from "../assets/user/Exit2.svg"
import BaseButton from "../components/BaseButton.vue"

const scanStore = useScanStore()
const { loading, result, error } = storeToRefs(scanStore)

const videoRef = ref<HTMLVideoElement | null>(null)
let stream: MediaStream | null = null

// 🎥 Start kamera
onMounted(async () => {
  try {
    stream = await navigator.mediaDevices.getUserMedia({
      video: { facingMode: "environment" }
    })

    if (videoRef.value) {
      videoRef.value.srcObject = stream
    }
  } catch (err) {
    console.error("Camera error:", err)
  }
})

// 🧹 Stop kamera saat keluar
onUnmounted(() => {
  stream?.getTracks().forEach(track => track.stop())
})

// 📸 Capture frame dari kamera (SUDAH DIUPDATE: Resize + JPEG 70%)
const captureImage = (): string => {
  const canvas = document.createElement("canvas")
  const video = videoRef.value!

  // Paksa mentok di 800 pixel agar AI cepat & HP tidak ngelag
  const MAX_SIZE = 800;
  let w = video.videoWidth;
  let h = video.videoHeight;
  if (w > h) { h *= MAX_SIZE / w; w = MAX_SIZE; } 
  else { w *= MAX_SIZE / h; h = MAX_SIZE; }

  canvas.width = w
  canvas.height = h

  const ctx = canvas.getContext("2d")!
  ctx.drawImage(video, 0, 0, w, h)

  // WAJIB JPEG Kualitas 70%. Jangan PNG!
  return canvas.toDataURL("image/jpeg", 0.7)
}

// 🚀 Trigger scan
const handleScan = () => {
  const image = captureImage()
  scanStore.scanTrash(image)
}

// ❌ Tutup modal
const closeResult = () => {
  scanStore.resetScan()
}
</script>

<template>
  <div class="flex flex-col min-h-screen bg-gray-100">

    <div class="flex justify-center py-4">
      <img :src="Logo" alt="Logo" class="h-10" />
    </div>

    <div class="flex justify-center px-4">
      <div
        class="relative w-full max-w-sm aspect-9/16 bg-black rounded-[40px] border-[6px] border-lime-400 overflow-hidden shadow-lg"
      >
        <video
          ref="videoRef"
          autoplay
          playsinline
          class="w-full h-full object-cover"
        />

        <div class="absolute inset-0 flex items-center justify-center">
          <img
            :src="loading ? LoadingIcon : FokusIcon"
            :class="[
              'w-40 h-40 transition-all duration-300',
              loading && 'animate-spin'
            ]"
          />
        </div>

        <div class="absolute bottom-10 px-6 text-center">
          <p class="text-[#FAA111] text-sm font-medium">
            {{
              loading
                ? "Menganalisis Sampah..."
                : "Silakan fokuskan kamera pada sampah agar sistem dapat mendeteksi secara otomatis"
            }}
          </p>

          <p v-if="loading" class="text-[#FAA111] text-xs mt-2">
            AI sedang mengecek kelayakan sampah
          </p>

          <p v-if="error" class="text-red-500 text-xs mt-2">
            {{ error }}
          </p>
        </div>
      </div>
    </div>

    <div class="mt-auto flex justify-between items-center px-8 py-6">
      <img :src="ImageIcon" class="w-14 h-14 cursor-pointer" />

      <button
        @click="handleScan"
        :disabled="loading"
        class="disabled:opacity-50 active:scale-95 transition"
      >
        <img :src="FotoIcon" class="w-16 h-16" />
      </button>

      <img :src="ExitIcon" class="w-14 h-14 cursor-pointer" />
    </div>
  </div>

  <div
    v-if="result"
    class="fixed inset-0 flex items-end justify-center bg-black/40 z-50 p-4 pb-0"
  >
    <div
      class="w-full max-w-md bg-white rounded-t-[30px] p-6 shadow-xl animate-slideUp"
    >
      <div class="w-12 h-1 bg-green-400 mx-auto mb-4 rounded-full"></div>

      <div class="flex items-center gap-4 mb-4">
        <div class="w-12 h-12 bg-green-100 rounded-xl flex items-center justify-center text-xl">
          ✅
        </div>

        <div>
          <h2 class="font-bold text-lg">{{ result.trash_type }}</h2>
        </div>
      </div>

      <div class="space-y-3 mb-4">

        <div class="border rounded-xl p-3 bg-gray-50">
          <p class="font-semibold text-sm">Material & Ukuran</p>
          <p class="text-sm text-gray-600">{{ result.material_info }}</p>
        </div>

        <div class="border rounded-xl p-3 bg-gray-50">
          <p class="font-semibold text-sm">Kondisi</p>
          <p class="text-sm text-gray-600">{{ result.kondisi }}</p>
        </div>

      </div>

      <div class="flex justify-between items-center border-t pt-4 mb-6">
        <span class="font-semibold text-gray-700">Estimasi Harga</span>
        <span class="text-green-600 font-bold text-xl">
          Rp{{ result.estimasi_harga.toLocaleString("id-ID") }}
        </span>
      </div>

      <BaseButton
        label="Tutup"
        class="w-full bg-green-500 text-white hover:bg-green-600 py-3 rounded-xl font-bold"
        @click="closeResult"
      />
    </div>
  </div>
</template>

<style scoped>
@keyframes slideUp {
  from {
    transform: translateY(100%);
  }
  to {
    transform: translateY(0);
  }
}

.animate-slideUp {
  animation: slideUp 0.3s ease-out;
}
</style>