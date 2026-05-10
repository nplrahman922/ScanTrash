<script setup lang="ts">
import { storeToRefs } from "pinia"
import { useScanStore } from "../../stores/scanStore"
import { onMounted, onUnmounted, ref } from "vue"
import { useRouter } from "vue-router"

import FokusIcon from "../assets/userAset/Fokus.svg"
import LoadingIcon from "../assets/userAset/Loading.svg"
import Logo from "../assets/Logo2.svg"
import ImageIcon from "../assets/userAset/Image.svg"
import FotoIcon from "../assets/userAset/Foto.svg"
import ExitIcon from "../assets/userAset/Exit2.svg"
import BaseButton from "../components/BaseButton.vue"

const router = useRouter()
const scanStore = useScanStore()
const { result, error } = storeToRefs(scanStore)

const videoRef = ref<HTMLVideoElement | null>(null)
const fileInput = ref<HTMLInputElement | null>(null)

let stream: MediaStream | null = null

const currentScreen = ref<'scanning' | 'analyzing'>('scanning')

// 🎥 START CAMERA
const startCamera = async () => {
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
}

// 🧹 STOP CAMERA
const stopCamera = () => {
  stream?.getTracks().forEach(track => track.stop())
}

// ⏳ WAIT VIDEO READY (🔥 FIX BUG)
const waitForVideoReady = async () => {
  return new Promise<void>((resolve) => {
    const video = videoRef.value!
    if (video.videoWidth > 0) return resolve()

    video.onloadedmetadata = () => resolve()
  })
}

// 📸 CAPTURE
const captureImage = async (): Promise<string> => {
  await waitForVideoReady()

  const video = videoRef.value!
  const canvas = document.createElement("canvas")
  const ctx = canvas.getContext("2d")!

  const MAX_SIZE = 800
  let w = video.videoWidth
  let h = video.videoHeight

  if (w > h) {
    h *= MAX_SIZE / w
    w = MAX_SIZE
  } else {
    w *= MAX_SIZE / h
    h = MAX_SIZE
  }

  canvas.width = w
  canvas.height = h

  ctx.drawImage(video, 0, 0, w, h)

  return canvas.toDataURL("image/jpeg", 0.7)
}

// 🚀 SCAN CAMERA
const handleScan = async () => {
  try {
    currentScreen.value = "analyzing"

    const image = await captureImage()
    stopCamera()

    await scanStore.scanTrash(image)

  } catch (err) {
    console.error(err)
  } finally {
    currentScreen.value = "scanning"
    startCamera() // 🔥 FIX: hidupkan lagi kamera
  }
}

// 📁 GALLERY
const triggerGallery = () => {
  fileInput.value?.click()
}

const handleGalleryUpload = async (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return

  try {
    currentScreen.value = "analyzing"
    stopCamera()

    const base64 = await compressFileImage(file)
    await scanStore.scanTrash(base64)

  } catch (err) {
    console.error(err)
  } finally {
    currentScreen.value = "scanning"
    startCamera()
  }
}

// 🧠 COMPRESS IMAGE
const compressFileImage = (file: File): Promise<string> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()

    reader.onload = (e) => {
      const img = new Image()

      img.onload = () => {
        const canvas = document.createElement("canvas")
        const ctx = canvas.getContext("2d")!

        const MAX_SIZE = 800
        let w = img.width
        let h = img.height

        if (w > h) {
          h *= MAX_SIZE / w
          w = MAX_SIZE
        } else {
          w *= MAX_SIZE / h
          h = MAX_SIZE
        }

        canvas.width = w
        canvas.height = h

        ctx.drawImage(img, 0, 0, w, h)

        resolve(canvas.toDataURL("image/jpeg", 0.7))
      }

      img.onerror = reject
      img.src = e.target?.result as string
    }

    reader.onerror = reject
    reader.readAsDataURL(file)
  })
}

// ❌ CLOSE RESULT
const closeResult = () => {
  scanStore.resetScan()
  startCamera()
}

onMounted(startCamera)
onUnmounted(stopCamera)

const handleExit = () => {
  scanStore.resetScan()
  stopCamera()
  router.back()
}
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
        <!-- 🎥 VIDEO -->
        <video
          v-if="!result"
          ref="videoRef"
          autoplay
          playsinline
          class="w-full h-full object-cover"
        />

        <!-- 🎯 OVERLAY -->
        <div class="absolute inset-0 flex items-center justify-center">
          <img
            :src="currentScreen === 'analyzing' ? LoadingIcon : FokusIcon"
            :class="[
              'w-40 h-40 transition-all duration-300',
              currentScreen === 'analyzing' && 'animate-spin'
            ]"
          />
        </div>

        <!-- 📝 TEXT -->
        <div class="absolute bottom-10 px-6 text-center">
          <p class="text-[#FAA111] text-sm font-medium">
            {{
              currentScreen === "analyzing"
                ? "Menganalisis Sampah..."
                : "Silakan fokuskan kamera pada sampah agar sistem dapat mendeteksi secara otomatis"
            }}
          </p>

          <p v-if="currentScreen === 'analyzing'" class="text-[#FAA111] text-xs mt-2">
            AI sedang mengecek kelayakan sampah
          </p>

          <p v-if="error" class="text-red-500 text-xs mt-2">
            {{ error }}
          </p>
        </div>
      </div>
    </div>

    <!-- 🔘 ACTION -->
    <div class="mt-auto flex justify-between items-center px-8 py-6">

      <!-- 📁 GALLERY -->
      <img 
        :src="ImageIcon" 
        class="w-14 h-14 cursor-pointer active:scale-95 transition"
        @click="triggerGallery"
      />

      <!-- 📸 SNAP -->
      <button
        @click="handleScan"
        :disabled="currentScreen === 'analyzing'"
        class="disabled:opacity-50 active:scale-95 transition"
      >
        <img :src="FotoIcon" class="w-16 h-16" />
      </button>

      <!-- ❌ EXIT -->
      <img 
        :src="ExitIcon" 
        class="w-14 h-14 cursor-pointer active:scale-95 transition"
        @click="handleExit"
      />
    </div>

    <!-- 📁 INPUT FILE (HIDDEN) -->
    <input 
      type="file" 
      ref="fileInput" 
      accept="image/*" 
      @change="handleGalleryUpload"
      hidden
    />
  </div>

  <!-- 🟢 RESULT MODAL -->
  <div
    v-if="result"
    class="fixed inset-0 flex items-end justify-center bg-black/40 z-50"
  >
    <div
      class="w-full max-w-md bg-white rounded-t-[30px] p-6 shadow-xl animate-slideUp"
    >
      <!-- indicator -->
      <div class="w-12 h-1 bg-green-400 mx-auto mb-4 rounded-full"></div>

      <!-- HEADER -->
      <div class="flex items-center gap-4 mb-4">
        <div class="w-12 h-12 bg-green-100 rounded-xl flex items-center justify-center">
          ✅
        </div>

        <div>
          <h2 class="font-bold text-lg">{{ result.name }}</h2>
          <p class="text-green-600 font-semibold">{{ result.status }}</p>
        </div>
      </div>

      <!-- DETAIL -->
      <div class="space-y-3 mb-4">

        <div class="border rounded-xl p-3">
          <p class="font-semibold text-sm">Material & Ukuran</p>
          <p class="text-sm text-gray-600">{{ result.details.material }}</p>
        </div>

        <div class="border rounded-xl p-3">
          <p class="font-semibold text-sm">Kondisi</p>
          <p class="text-sm text-gray-600">{{ result.details.condition }}</p>
        </div>

        <div class="border rounded-xl p-3">
          <p class="font-semibold text-sm">Kebersihan</p>
          <p class="text-sm text-gray-600">{{ result.details.cleanliness }}</p>
        </div>

      </div>

      <!-- PRICE -->
      <div class="flex justify-between items-center border-t pt-4 mb-6">
        <span class="font-semibold">Estimasi Harga</span>
        <span class="text-green-600 font-bold">
          Rp{{ result.price.toLocaleString("id-ID") }}
        </span>
      </div>

      <!-- BUTTON -->
      <BaseButton
        label="Tutup"
        class="w-full bg-green-500 text-white hover:bg-green-600"
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

/* tambahan biar smooth UX */
video {
  backface-visibility: hidden;
}

/* efek klik halus */
button:active img,
img:active {
  transform: scale(0.95);
}
</style>