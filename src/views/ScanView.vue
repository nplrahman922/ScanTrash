<script setup lang="ts">
import { storeToRefs } from "pinia"
import { useScanStore } from "../stores/scanStore"
import { onMounted, onUnmounted, ref } from "vue"
import { useRouter } from "vue-router"

import FokusIcon from "../assets/user/Fokus.svg"
import LoadingIcon from "../assets/user/Loading.svg"
import Logo from "../assets/Logo2.svg"
import ImageIcon from "../assets/user/Image.svg"
import FotoIcon from "../assets/user/Foto.svg"
import ExitIcon from "../assets/user/Exit2.svg"
import BaseButton from "../components/BaseButton.vue"

const router = useRouter()
const scanStore = useScanStore()
const { result, error } = storeToRefs(scanStore)

const videoRef = ref<HTMLVideoElement | null>(null)
const fileInput = ref<HTMLInputElement | null>(null)

let stream: MediaStream | null = null

const currentScreen = ref<'scanning' | 'analyzing'>('scanning')
const capturedImage = ref<string | null>(null)

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
    capturedImage.value = image // 👈 2a. Tampilkan gambarnya di layar!
    stopCamera()

    await scanStore.scanTrash(image)

  } catch (err) {
    console.error(err)
  } finally {
    currentScreen.value = "scanning"
    capturedImage.value = null // 👈 2b. Hapus gambarnya saat loading selesai
    startCamera()
  }
}

// 📁 GALLERY
const triggerGallery = () => {
  fileInput.value?.click()
}

// 📁 GALLERY UPLOAD
const handleGalleryUpload = async (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]

  target.value = ''
  
  if (!file) return

  try {
    currentScreen.value = "analyzing"
    stopCamera()

    const base64 = await compressFileImage(file)
    capturedImage.value = base64 // 👈 2c. Tampilkan gambar galerinya di layar!
    
    await scanStore.scanTrash(base64)

  } catch (err) {
    console.error(err)
  } finally {
    currentScreen.value = "scanning"
    capturedImage.value = null // 👈 2d. Hapus gambarnya saat loading selesai
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
          v-show="!capturedImage"
          ref="videoRef"
          autoplay
          playsinline
          class="w-full h-full object-cover"
        />

        <img
          v-if="capturedImage"
          :src="capturedImage"
          class="absolute inset-0 w-full h-full object-cover"
        />

        <div 
          v-if="currentScreen === 'analyzing'" 
          class="absolute inset-0 bg-black/50 backdrop-blur-sm flex flex-col items-center justify-center z-10"
        >
          <img :src="LoadingIcon" class="w-32 h-32 animate-spin mb-2" />
        </div>

        <div 
          v-else-if="currentScreen === 'scanning' && !result" 
          class="absolute inset-0 flex items-center justify-center z-10"
        >
          <img :src="FokusIcon" class="w-40 h-40 transition-all duration-300" />
        </div>

        <div class="absolute bottom-10 w-full px-6 text-center z-20">
          <p class="text-[#FAA111] text-sm font-medium drop-shadow-md">
            {{
              currentScreen === "analyzing"
                ? "Menganalisis Sampah..."
                : "Silakan fokuskan kamera pada sampah agar sistem dapat mendeteksi secara otomatis"
            }}
          </p>

          <p v-if="currentScreen === 'analyzing'" class="text-[#FAA111] text-xs mt-2 drop-shadow-md">
            AI sedang mengecek kelayakan sampah
          </p>

          <p v-if="error" class="text-red-500 text-xs mt-2 font-bold bg-white/80 py-1 rounded-full">
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
    v-if="result && result.length > 0"
    class="fixed inset-0 flex items-end justify-center bg-black/40 z-50 p-4 pb-0"
  >
    <div class="w-full max-w-md bg-white rounded-t-[30px] p-6 shadow-xl animate-slideUp max-h-[85vh] overflow-y-auto relative">
      
      <div class="w-12 h-1 bg-green-400 mx-auto mb-6 rounded-full"></div>

      <h2 class="text-center font-extrabold text-[#112F30] text-xl mb-6">
        Terdapat {{ result.length }} Objek Terdeteksi!
      </h2>

    <div 
        v-for="(item, index) in result" 
        :key="index"
        :class="[
          'rounded-2xl p-5 border mb-6 transition-all',
          (item.trash_type.includes('Response tidak') || Number(item.estimasi_harga) === 0)
            ? 'bg-red-50 border-red-200' 
            : 'bg-[#F0FDF4] border-green-100'
        ]"
      >
        <h3 :class="[
          'font-bold mb-4 border-b pb-2',
          (item.trash_type.includes('Response tidak') || Number(item.estimasi_harga) === 0)
            ? 'text-red-500 border-red-200'
            : 'text-green-600 border-green-200'
        ]">
          Objek #{{ index + 1 }}
        </h3>

        <div class="flex items-center gap-4 mb-4">
          <div :class="[
            'w-12 h-12 rounded-xl flex items-center justify-center text-2xl shadow-sm',
            (item.trash_type.includes('Response tidak') || Number(item.estimasi_harga) === 0)
              ? 'bg-red-100' 
              : 'bg-white'
          ]">
            {{ (item.trash_type.includes('Response tidak') || Number(item.estimasi_harga) === 0) ? '⚠️' : '🔍' }}
          </div>
          
          <div>
            <h2 class="font-bold text-lg text-gray-800 leading-tight">
              {{ item.trash_type.includes('Response tidak') ? 'Objek Tidak Dikenali' : item.trash_type }}
            </h2>
            <p :class="[
              'font-semibold text-sm',
              (item.trash_type.includes('Response tidak') || Number(item.estimasi_harga) === 0)
                ? 'text-red-500'
                : 'text-green-600'
            ]">
              {{ (item.trash_type.includes('Response tidak') || Number(item.estimasi_harga) === 0) ? 'Tidak Bernilai Jual' : 'Layak Ditabung' }}
            </p>
          </div>
        </div>

        <div class="space-y-3 mb-4">
          <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100">
            <p class="font-semibold text-xs text-gray-500">Material & Ukuran</p>
            <p class="text-sm text-gray-800 font-medium">{{ item.material_info }}</p>
          </div>

          <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100">
            <p class="font-semibold text-xs text-gray-500">Kondisi ({{ item.kebersihan }})</p>
            <p class="text-sm text-gray-800">{{ item.kondisi }}</p>
          </div>
        </div>

        <div :class="[
          'flex justify-between items-center mt-2 pt-4 border-t',
          (item.trash_type.includes('Response tidak') || Number(item.estimasi_harga) === 0)
            ? 'border-red-200'
            : 'border-green-200'
        ]">
          <span class="font-semibold text-gray-700">Estimasi Harga Total</span>
          
          <span :class="[
            'font-bold text-xl',
            (item.trash_type.includes('Response tidak') || Number(item.estimasi_harga) === 0)
              ? 'text-red-500'
              : 'text-green-600'
          ]">
            Rp{{ Number(item.estimasi_harga).toLocaleString("id-ID") }}
          </span>
        </div>
      </div>

      <div class="sticky bottom-0 bg-white pt-2 pb-4 mt-4">
        <BaseButton
          label="Tutup"
          class="w-full bg-green-500 text-white hover:bg-green-600 py-3 rounded-xl font-bold"
          @click="closeResult"
        />
      </div>

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