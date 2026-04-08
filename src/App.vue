<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface AIItem {
  jenis_sampah: string;
  berat_jumlah: string;
  estimasi_harga: string;
  keterangan: string;
}

const isProcessing = ref(false);
const imagePreview = ref<string | null>(null);
const results = ref<AIItem[]>([]);
const errorMessage = ref<string | null>(null);
const fileInput = ref<HTMLInputElement | null>(null);

function triggerFileSelect() {
  fileInput.value?.click();
}

async function onFileChange(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;

  errorMessage.value = null;
  results.value = [];

  const reader = new FileReader();
  reader.onload = async (e) => {
    const base64Str = e.target?.result as string;
    imagePreview.value = base64Str;
    
    isProcessing.value = true;
    try {
      const response = await invoke<AIItem[]>("analisa_image", { 
        image: { photobase64: base64Str } 
      });
      results.value = response || [];
    } catch (error: any) {
      errorMessage.value = `Gagal menganalisis: ${error}`;
    } finally {
      isProcessing.value = false;
      if (fileInput.value) {
        fileInput.value.value = "";
      }
    }
  };
  reader.readAsDataURL(file);
}
</script>

<template>
  <main class="app-container">
    <div class="glass-panel">
      <header class="header">
        <h1>🌱 <span class="highlight">Eco</span>Scan</h1>
        <p>Deteksi Sampah & Estimasi Harga</p>
      </header>

      <div class="upload-section">
        <input 
          type="file" 
          accept="image/*" 
          capture="environment" 
          ref="fileInput" 
          @change="onFileChange" 
          class="hidden-input"
        />
        
        <div 
          class="preview-box" 
          @click="triggerFileSelect"
          :class="{'has-image': imagePreview}"
        >
          <img v-if="imagePreview" :src="imagePreview" class="uploaded-image" alt="Pratinjau Sampah" />
          <div v-else class="placeholder">
            <svg class="camera-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 13a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <p>Ketuk untuk Mengambil Foto</p>
          </div>
          
          <div v-if="isProcessing" class="loading-overlay">
            <div class="spinner"></div>
            <p>Menganalisis dengan AI...</p>
          </div>
        </div>
      </div>

      <div class="result-section" v-if="results.length > 0 || errorMessage">
        <transition name="fade" mode="out-in">
          <div v-if="results.length > 0" class="results-container">
            <h2 class="results-title">Terdapat {{ results.length }} Objek Terdeteksi!</h2>
            
            <div v-for="(item, index) in results" :key="index" class="result-card success">
              <h3>Objek #{{ index + 1 }}</h3>
              <div class="result-item">
                <span class="label">Jenis Sampah</span>
                <span class="value">{{ item.jenis_sampah || 'Tidak teridentifikasi' }}</span>
              </div>
              <div class="result-item">
                <span class="label">Berat/Satuan</span>
                <span class="value">{{ item.berat_jumlah || '-' }}</span>
              </div>
              <div class="result-item total">
                <span class="label">Estimasi Harga</span>
                <span class="value pricing">{{ item.estimasi_harga || 'Rp 0' }}</span>
              </div>
              <div v-if="item.keterangan" class="keterangan-box">
                <p>{{ item.keterangan }}</p>
              </div>
            </div>
          </div>
          
          <div v-else-if="errorMessage" class="result-card error">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="error-icon">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <p>{{ errorMessage }}</p>
          </div>
        </transition>
      </div>
    </div>
  </main>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;600;700&display=swap');

:root {
  --primary: #10b981;
  --primary-dark: #059669;
  --bg-gradient: linear-gradient(135deg, #f0fdfa 0%, #ecfdf5 100%);
  --surface: rgba(255, 255, 255, 0.85);
  --text-dark: #0f172a;
  --text-muted: #64748b;
  --shadow-sm: 0 4px 6px -1px rgba(0, 0, 0, 0.05);
  --shadow-lg: 0 10px 25px -3px rgba(0, 0, 0, 0.1);
  --radius: 20px;
}

@media (prefers-color-scheme: dark) {
  :root {
    --bg-gradient: linear-gradient(135deg, #0f172a 0%, #020617 100%);
    --surface: rgba(30, 41, 59, 0.7);
    --text-dark: #f8fafc;
    --text-muted: #94a3b8;
  }
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family: 'Outfit', sans-serif;
  background: var(--bg-gradient);
  color: var(--text-dark);
  min-height: 100vh;
  margin: 0;
}

.app-container {
  display: flex;
  justify-content: center;
  align-items: flex-start;
  min-height: 100vh;
  padding: 2rem 1rem;
}

.glass-panel {
  background: var(--surface);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  border: 1px solid rgba(255,255,255,0.2);
  border-radius: var(--radius);
  padding: 2rem;
  width: 100%;
  max-width: 500px;
  box-shadow: var(--shadow-lg);
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.header {
  text-align: center;
}

.header h1 {
  font-size: 2.5rem;
  font-weight: 700;
  margin-bottom: 0.25rem;
}

.highlight {
  color: var(--primary);
}

.header p {
  color: var(--text-muted);
  font-size: 1rem;
}

.hidden-input {
  display: none;
}

.preview-box {
  width: 100%;
  aspect-ratio: 4/5;
  border-radius: calc(var(--radius) - 4px);
  border: 2px dashed rgba(16, 185, 129, 0.4);
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
  overflow: hidden;
  cursor: pointer;
  background-color: rgba(16, 185, 129, 0.05);
  transition: all 0.3s ease;
}

.preview-box:hover {
  border-color: var(--primary);
  background-color: rgba(16, 185, 129, 0.1);
  transform: translateY(-2px);
}

.preview-box.has-image {
  border-style: solid;
  border-color: transparent;
  background-color: #000;
}

.placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  color: var(--primary);
  gap: 1rem;
}

.camera-icon {
  width: 48px;
  height: 48px;
}

.uploaded-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  animation: fadeIn 0.5s ease-in-out;
}

.loading-overlay {
  position: absolute;
  inset: 0;
  background: rgba(0,0,0,0.6);
  backdrop-filter: blur(4px);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: white;
  gap: 1rem;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid rgba(255,255,255,0.3);
  border-top-color: var(--primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.result-card {
  padding: 1.5rem;
  border-radius: calc(var(--radius) - 4px);
  display: flex;
  flex-direction: column;
  gap: 1rem;
  animation: slideUp 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.result-card.success {
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(5, 150, 105, 0.05) 100%);
  border: 1px solid rgba(16, 185, 129, 0.2);
}

.result-card.error {
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.2);
  align-items: center;
  text-align: center;
  color: #ef4444;
}

.results-container {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.results-title {
  font-size: 1.25rem;
  color: var(--text-dark);
  text-align: center;
  font-weight: 700;
  margin-bottom: 0.5rem;
}

.result-card h3 {
  font-size: 1.1rem;
  color: var(--primary-dark);
  margin-bottom: 0.5rem;
  padding-bottom: 0.5rem;
  border-bottom: 2px solid rgba(16, 185, 129, 0.2);
}

.keterangan-box {
  margin-top: 1rem;
  padding: 0.75rem;
  background-color: rgba(255, 255, 255, 0.5);
  border-radius: 8px;
  font-size: 0.85rem;
  line-height: 1.4;
  color: var(--text-muted);
  border-left: 3px solid var(--primary);
}

@media (prefers-color-scheme: dark) {
  .keterangan-box {
    background-color: rgba(0, 0, 0, 0.2);
    color: #cbd5e1;
  }
}

.error-icon {
  width: 40px;
  height: 40px;
}

.result-card h2 {
  font-size: 1.25rem;
  color: var(--text-dark);
  margin-bottom: 0.5rem;
}

.result-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 0.75rem;
  border-bottom: 1px dashed rgba(16, 185, 129, 0.2);
}

.result-item:last-child {
  border-bottom: none;
  padding-bottom: 0;
  padding-top: 0.5rem;
}

.label {
  color: var(--text-muted);
  font-size: 0.95rem;
}

.value {
  font-weight: 600;
  color: var(--text-dark);
  text-align: right;
}

.pricing {
  font-size: 1.5rem;
  color: var(--primary);
  font-weight: 700;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideUp {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}

/* Transitions */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>