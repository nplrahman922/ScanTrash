<template>
  <div class="p-4 bg-gray-100 min-h-screen pb-24">

    <!-- ── HEADER ── -->
    <div class="flex items-center justify-between mb-4">
      <h1 class="text-lg font-bold text-gray-800">Katalog Harga</h1>
      <button
        id="btn-tambah-jenis"
        @click="adminStore.openAddPricelistModal()"
        class="flex items-center gap-1 px-4 py-2 bg-white border border-gray-300 rounded-xl text-sm font-semibold text-gray-700 shadow-sm active:scale-95 transition-transform"
      >
        <span class="text-base font-bold">+</span> Tambah Jenis
      </button>
    </div>

    <!-- ── LOADING SKELETON ── -->
    <div v-if="adminStore.loadingPricelist" class="space-y-4">
      <div v-for="i in 4" :key="i" class="h-28 bg-gray-200 rounded-2xl animate-pulse" />
    </div>

    <!-- ── ERROR STATE ── -->
    <div
      v-else-if="adminStore.errorPricelist"
      class="bg-red-50 border border-red-200 rounded-2xl p-6 text-center"
    >
      <p class="text-red-500 text-sm">{{ adminStore.errorPricelist }}</p>
      <button @click="adminStore.fetchPricelist()" class="mt-3 text-xs text-red-600 underline">
        Coba lagi
      </button>
    </div>

    <!-- ── EMPTY STATE ── -->
    <div
      v-else-if="adminStore.pricelistItems.length === 0"
      class="bg-white rounded-2xl p-10 text-center"
    >
      <p class="text-gray-400 text-sm">Belum ada item di katalog.</p>
      <button
        @click="adminStore.openAddPricelistModal()"
        class="mt-4 px-5 py-2 bg-green-500 text-white text-sm font-semibold rounded-xl"
      >
        + Tambah Jenis Sampah
      </button>
    </div>

    <!-- ── DAFTAR ITEM ── -->
    <div v-else class="space-y-4">
      <div
        v-for="item in adminStore.pricelistItems"
        :key="item.id"
        class="bg-white rounded-2xl shadow-sm overflow-hidden"
      >
        <!-- Konten item: gambar + detail -->
        <div class="flex items-center gap-4 p-4">
          <!-- Gambar -->
          <div class="w-16 h-16 rounded-xl overflow-hidden bg-gray-100 shrink-0 flex items-center justify-center">
            <img
              v-if="isValidImgUrl(item.img_url)"
              :src="item.img_url"
              :alt="item.labels"
              class="w-full h-full object-cover"
              @error="(e) => ((e.target as HTMLImageElement).style.display = 'none')"
            />
            <span v-else class="text-2xl">🗑️</span>
          </div>

          <!-- Nama + harga -->
          <div class="flex-1 min-w-0">
            <p class="font-bold text-gray-800 text-sm truncate">{{ item.labels }}</p>
            <p class="text-green-500 font-bold text-base">
              Rp{{ item.price.toLocaleString("id-ID") }}/kg
            </p>
          </div>
        </div>

        <!-- Aksi: Hapus + Edit -->
        <div v-if="adminStore.deletingPricelistId !== item.id" class="flex gap-3 px-4 pb-4">
          <button
            @click="adminStore.deletingPricelistId = item.id ?? null"
            class="flex-1 py-2 border border-gray-200 rounded-xl text-sm font-semibold text-gray-600 active:bg-gray-50"
          >
            Hapus
          </button>
          <button
            @click="adminStore.openEditPricelistModal(item)"
            class="flex-1 py-2 bg-gray-100 rounded-xl text-sm font-semibold text-gray-700 active:bg-gray-200"
          >
            Edit
          </button>
        </div>

        <!-- Konfirmasi hapus inline -->
        <div v-else class="px-4 pb-4">
          <p class="text-xs text-red-500 text-center mb-2">Yakin ingin menghapus <strong>{{ item.labels }}</strong>?</p>
          <div class="flex gap-2">
            <button
              @click="adminStore.deletingPricelistId = null"
              class="flex-1 py-2 border border-gray-200 rounded-xl text-sm font-medium text-gray-600"
            >
              Batal
            </button>
            <button
              @click="hapusItem(item.id!, item.img_url ?? '')"
              class="flex-1 py-2 bg-red-500 rounded-xl text-sm font-bold text-white"
            >
              Hapus
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- ══════════════════════════════════════════════════════ -->
  <!-- MODAL TAMBAH / EDIT (Bottom Sheet)                    -->
  <!-- ══════════════════════════════════════════════════════ -->
  <Transition name="fade">
    <div
      v-if="adminStore.showPricelistModal"
      class="fixed inset-0 bg-black/40 z-40"
      @click="adminStore.closePricelistModal()"
    />
  </Transition>

  <Transition name="slide-up">
    <div
      v-if="adminStore.showPricelistModal"
      class="fixed bottom-0 left-0 right-0 z-50 bg-white rounded-t-3xl shadow-2xl px-5 pt-5 pb-10"
    >
      <div class="w-10 h-1 bg-gray-200 rounded-full mx-auto mb-5" />
      <h2 class="text-lg font-bold text-gray-800 mb-5">
        {{ adminStore.editingPricelistId ? "Edit Jenis Sampah" : "Tambah Jenis Sampah" }}
      </h2>

      <div class="space-y-4">
        <!-- Upload Gambar -->
        <div>
          <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2 block">
            Gambar (PNG, maks. 512 KB)
          </label>

          <!-- Preview gambar -->
          <div
            @click="triggerFilePicker"
            class="w-full h-36 rounded-2xl border-2 border-dashed border-gray-300 flex flex-col items-center justify-center cursor-pointer active:border-green-400 transition-colors overflow-hidden bg-gray-50"
          >
            <img
              v-if="adminStore.pricelistForm.imagePreviewUrl"
              :src="adminStore.pricelistForm.imagePreviewUrl"
              class="w-full h-full object-cover rounded-2xl"
              alt="Preview"
            />
            <div v-else class="flex flex-col items-center gap-2 text-gray-400">
              <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                  d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              <p class="text-xs">Ketuk untuk pilih gambar PNG</p>
            </div>
          </div>

          <!-- Badge ganti gambar jika edit -->
          <p v-if="adminStore.editingPricelistId && !adminStore.pricelistForm.imageBase64"
             class="text-xs text-gray-400 mt-1 text-center">
            Biarkan kosong untuk tidak mengubah gambar
          </p>
          <p v-if="adminStore.pricelistForm.imageBase64" class="text-xs text-green-500 mt-1 text-center">
            ✓ Gambar baru dipilih
          </p>

          <!-- Hidden file input -->
          <input
            ref="fileInput"
            type="file"
            accept="image/png"
            class="hidden"
            @change="onFileSelect"
          />

          <p v-if="errorImage" class="text-red-500 text-xs text-center mt-1">{{ errorImage }}</p>
        </div>

        <!-- Nama Sampah -->
        <div>
          <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1 block">
            Nama Sampah
          </label>
          <input
            id="input-nama-sampah"
            v-model="adminStore.pricelistForm.labels"
            type="text"
            placeholder="Contoh: Plastik (PET), Kertas/Kardus"
            class="w-full p-4 bg-gray-50 rounded-2xl border border-gray-200 text-sm focus:outline-none focus:ring-2 focus:ring-green-400"
          />
        </div>

        <!-- Harga Per Kg -->
        <div>
          <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1 block">
            Harga Per Kg (Rp)
          </label>
          <div class="relative">
            <span class="absolute inset-y-0 left-4 flex items-center text-gray-400 text-sm font-medium">Rp</span>
            <input
              id="input-harga-pricelist"
              v-model.number="adminStore.pricelistForm.price"
              type="number"
              min="0"
              placeholder="0"
              class="w-full pl-10 pr-4 py-4 bg-gray-50 rounded-2xl border border-gray-200 text-sm focus:outline-none focus:ring-2 focus:ring-green-400"
            />
          </div>
        </div>

        <!-- Error -->
        <p v-if="adminStore.errorPricelistForm" class="text-red-500 text-sm text-center">
          {{ adminStore.errorPricelistForm }}
        </p>

        <!-- Submit -->
        <button
          id="btn-simpan-pricelist"
          @click="adminStore.submitPricelist()"
          :disabled="adminStore.loadingPricelistForm"
          class="w-full py-4 bg-[#F5A623] text-white font-bold rounded-2xl disabled:opacity-60 active:scale-95 transition-transform"
        >
          <span v-if="adminStore.loadingPricelistForm">Menyimpan...</span>
          <span v-else-if="adminStore.editingPricelistId">Simpan Perubahan</span>
          <span v-else>Simpan Jenis Sampah</span>
        </button>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue"
import { useAdminStore } from "../stores/adminStore"

const adminStore = useAdminStore()

// ── File picker ───────────────────────────────────────────────────────────────
const fileInput = ref<HTMLInputElement | null>(null)
const errorImage = ref("")

const triggerFilePicker = () => {
  errorImage.value = ""
  fileInput.value?.click()
}

const onFileSelect = (event: Event) => {
  errorImage.value = ""
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return

  // Validasi tipe
  if (file.type !== "image/png") {
    errorImage.value = "Hanya file PNG yang diizinkan."
    return
  }

  // Validasi ukuran: 512 KB = 524288 bytes
  const MAX_SIZE = 524288
  if (file.size > MAX_SIZE) {
    errorImage.value = `File terlalu besar (${Math.round(file.size / 1024)} KB). Maksimal 512 KB.`
    return
  }

  // Baca sebagai base64
  const reader = new FileReader()
  reader.onload = (e) => {
    const result = e.target?.result as string
    // result format: "data:image/png;base64,XXXXX"
    // Ambil hanya bagian base64 setelah koma
    const base64 = result.split(",")[1] ?? ""
    adminStore.pricelistForm.imageBase64 = base64
    adminStore.pricelistForm.imagePreviewUrl = result  // Pakai data URL untuk preview
  }
  reader.readAsDataURL(file)

  // Reset input agar bisa pilih file yang sama lagi jika perlu
  ;(event.target as HTMLInputElement).value = ""
}

// ── Hapus item ────────────────────────────────────────────────────────────────
const hapusItem = async (id: string, imgUrl: string) => {
  try {
    await adminStore.deletePricelist(id, imgUrl)
  } catch {
    alert("Gagal menghapus item. Silakan coba lagi.")
  }
}

// ── Helpers ───────────────────────────────────────────────────────────────────
const isValidImgUrl = (url?: string): boolean => {
  return !!url && url !== "0" && url.startsWith("http")
}

onMounted(() => {
  adminStore.fetchPricelist()
})
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.25s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.slide-up-enter-active,
.slide-up-leave-active {
  transition: transform 0.3s cubic-bezier(0.32, 0.72, 0, 1);
}
.slide-up-enter-from,
.slide-up-leave-to {
  transform: translateY(100%);
}
</style>
