<template>
  <div class="camera-container">
    <video 
      ref="videoElement" 
      autoplay 
      playsinline 
      class="live-video"
      v-show="currentScreen === 'scanning'"
    ></video>

    <div v-if="currentScreen === 'scanning'" class="scan-overlay">
      
      <div class="header-text">
        <p>Silakan fokuskan kamera pada sampah agar sistem dapat mendeteksi secara otomatis</p>
      </div>

      <div class="scan-box-container">
        <div class="scan-box"></div>
      </div>
      
      <div class="action-buttons">
        <button class="icon-btn gallery-btn" @click="triggerGallery">
          📁 </button>

        <button class="snap-button" @click="takeSnapshot"></button>

        <button class="icon-btn cancel-btn" @click="goBack">
          ❌ </button>
      </div>

      <input 
        type="file" 
        ref="fileInput" 
        accept="image/*" 
        @change="handleGalleryUpload" 
        style="display: none;" 
      />
    </div>

    <div v-if="currentScreen === 'analyzing'" class="analyzing-screen">
      <div class="spinner"></div>
      <h3>Menganalisis Sampah...</h3>
      <p>AI sedang mengecek kelayakan sampah</p>
    </div>

    <div v-if="currentScreen === 'result' && scanResult" class="result-sheet">
      <div class="sheet-content">
        <h3>{{ scanResult.trash_type }}</h3>
        <p class="grade">✅ {{ scanResult.kelayakan }}</p>

        <div class="info-box">
          <p><strong>Material dan Ukuran:</strong><br/>{{ scanResult.material_info }}</p>
        </div>
        
        <div class="info-box">
          <p><strong>Kondisi:</strong><br/>{{ scanResult.kondisi }}</p>
        </div>
        <div class="info-box">
          <p><strong>Kebersihan:</strong><br/>{{ scanResult.kebersihan }}</p>
        </div>
        
        <h3 class="price">Estimasi Harga: Rp{{ scanResult.estimasi_harga }}</h3>
        <button @click="resetCamera" class="close-btn">Tutup</button>
      </div>
    </div>

    <canvas ref="canvasElement" style="display: none;"></canvas>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';

const router = useRouter();

// State
const currentScreen = ref<'scanning' | 'analyzing' | 'result'>('scanning');
const scanResult = ref<any>(null);

// DOM Refs
const videoElement = ref<HTMLVideoElement | null>(null);
const canvasElement = ref<HTMLCanvasElement | null>(null);
const fileInput = ref<HTMLInputElement | null>(null);
let mediaStream: MediaStream | null = null;

// --- FUNGSI KAMERA ---
async function startCamera() {
  try {
    mediaStream = await navigator.mediaDevices.getUserMedia({
      video: { facingMode: 'environment' } 
    });
    if (videoElement.value) {
      videoElement.value.srcObject = mediaStream;
    }
  } catch (err) {
    console.warn("Gagal akses kamera (mungkin di PC/Browser tanpa izin).", err);
  }
}

function stopCamera() {
  if (mediaStream) {
    mediaStream.getTracks().forEach(track => track.stop());
  }
}

// --- FUNGSI JEPRET LANGSUNG ---
async function takeSnapshot() {
  if (!videoElement.value || !canvasElement.value) return;

  currentScreen.value = 'analyzing';
  const video = videoElement.value;
  const canvas = canvasElement.value;
  const ctx = canvas.getContext('2d');
  
  // Hitung rasio untuk kompresi (Max 800px)
  const MAX_SIZE = 800;
  let w = video.videoWidth;
  let h = video.videoHeight;
  if (w > h) { h *= MAX_SIZE / w; w = MAX_SIZE; } 
  else { w *= MAX_SIZE / h; h = MAX_SIZE; }
  
  canvas.width = w; canvas.height = h;
  ctx?.drawImage(video, 0, 0, w, h);

  const dataUrl = canvas.toDataURL('image/jpeg', 0.7);
  const base64String = dataUrl.split(',')[1];

  stopCamera();
  await sendToRust(base64String);
}

// --- FUNGSI AMBIL DARI GALERI ---
function triggerGallery() {
  fileInput.value?.click(); // Memanggil input file tersembunyi
}

async function handleGalleryUpload(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;

  currentScreen.value = 'analyzing';
  stopCamera(); // Matikan kamera karena kita pakai foto galeri

  try {
    const base64String = await compressFileImage(file);
    await sendToRust(base64String);
  } catch (err) {
    alert("Gagal memproses gambar galeri.");
    resetCamera();
  }
}

// --- FUNGSI BERSAMA (KIRIM KE RUST) ---
async function sendToRust(base64: string) {
  try {
    const result = await invoke('analyze_trash_command', { imageBase64: base64 });
    scanResult.value = result;
    currentScreen.value = 'result';
  } catch (error) {
    alert("Gagal AI: " + error);
    resetCamera();
  }
}

// --- FUNGSI HELPER: KOMPRESI FILE GALERI ---
function compressFileImage(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (e) => {
      const img = new Image();
      img.onload = () => {
        const canvas = document.createElement('canvas');
        const MAX_SIZE = 800;
        let w = img.width; let h = img.height;
        if (w > h) { h *= MAX_SIZE / w; w = MAX_SIZE; } 
        else { w *= MAX_SIZE / h; h = MAX_SIZE; }
        
        canvas.width = w; canvas.height = h;
        const ctx = canvas.getContext('2d');
        ctx?.drawImage(img, 0, 0, w, h);
        
        resolve(canvas.toDataURL('image/jpeg', 0.7).split(',')[1]);
      };
      img.onerror = reject;
      img.src = e.target?.result as string;
    };
    reader.onerror = reject;
    reader.readAsDataURL(file);
  });
}

function resetCamera() {
  scanResult.value = null;
  currentScreen.value = 'scanning';
  startCamera();
}

function goBack() {
  router.back();
}

onMounted(() => startCamera());
onUnmounted(() => stopCamera());
</script>

<style scoped>
/* 1. CONTAINER UTAMA: Mengunci layar agar tidak bisa di-scroll & responsif */
.camera-container {
  position: absolute; /* Mengambil alih seluruh layar */
  top: 0; left: 0; right: 0; bottom: 0;
  background-color: #000;
  overflow: hidden; /* Mencegah scrollbar */
  color: white;
  font-family: sans-serif;
}

/* 2. VIDEO KAMERA: Memenuhi layar tanpa merusak rasio (anti gepeng) */
.live-video {
  position: absolute;
  top: 0; left: 0;
  width: 100%;
  height: 100%;
  object-fit: cover; /* Rahasia anti-gepeng! */
  z-index: 1;
}

/* 3. OVERLAY UI: Berada di atas video */
.scan-overlay {
  position: absolute;
  top: 0; left: 0; width: 100%; height: 100%;
  z-index: 2;
  display: flex;
  flex-direction: column;
  justify-content: space-between; /* Menyebar elemen Atas, Tengah, Bawah */
}

.header-text {
  padding: 40px 20px 0; /* Jarak dari poni HP */
  text-align: center;
  text-shadow: 1px 1px 3px rgba(0,0,0,0.8);
}

.scan-box-container {
  display: flex;
  justify-content: center;
  align-items: center;
  flex: 1; /* Mengisi ruang kosong di tengah */
}

.scan-box {
  width: 250px;
  height: 250px;
  border: 4px solid #FFC107;
  border-radius: 20px;
  box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.5); /* Efek gelap di luar kotak kuning */
}

/* 4. BARIS TOMBOL BAWAH: Fleksibel dan rata tengah */
.action-buttons {
  display: flex;
  justify-content: space-evenly;
  align-items: center;
  padding-bottom: 50px; /* Jarak dari dagu HP */
  width: 100%;
}

.snap-button {
  width: 75px; height: 75px;
  background-color: #4CAF50;
  border: 5px solid white;
  border-radius: 50%;
  cursor: pointer;
  box-shadow: 0 4px 10px rgba(0,0,0,0.5);
}

.icon-btn {
  width: 50px; height: 50px;
  background-color: rgba(255,255,255,0.2);
  border: none;
  border-radius: 50%;
  font-size: 20px;
  color: white;
  backdrop-filter: blur(5px);
  cursor: pointer;
}

/* 5. LAYAR LOADING & HASIL */
.analyzing-screen, .result-sheet {
  position: absolute;
  top: 0; left: 0; width: 100%; height: 100%;
  background-color: rgba(0,0,0,0.9);
  z-index: 10;
  display: flex; flex-direction: column; align-items: center; justify-content: center;
}

.spinner {
  width: 60px; height: 60px;
  border: 5px solid #333; border-top: 5px solid #FFC107;
  border-radius: 50%; animation: spin 1s linear infinite;
  margin-bottom: 20px;
}
@keyframes spin { 100% { transform: rotate(360deg); } }

.sheet-content {
  background: white; color: #333;
  padding: 30px; border-radius: 20px;
  width: 85%; max-width: 400px;
  text-align: left;
}
.sheet-content h3 { margin-top: 0; color: #2E7D32; }
.info-box { background: #f5f5f5; padding: 10px; border-radius: 10px; margin-bottom: 10px; font-size: 14px; }
.price { color: #FF9800; font-size: 22px; text-align: center; margin: 20px 0; }
.close-btn { width: 100%; padding: 15px; background: #4CAF50; color: white; border: none; border-radius: 10px; font-weight: bold; font-size: 16px;}
</style>