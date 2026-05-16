<template>
  <div class="p-4 space-y-4 bg-gray-100 min-h-screen pb-24">

    <!-- ── HEADER saat mode select ── -->
    <div v-if="isSelectMode" class="flex items-center gap-3 mb-1">
      <button @click="cancelSelect" class="p-2 rounded-xl bg-white shadow-sm">
        <img src="../assets/Kembali.svg" class="w-5 h-5" alt="kembali" />
      </button>
      <h2 class="font-bold text-gray-800">Pilih Nasabah</h2>
    </div>

    <!-- ── SEARCH BAR ── -->
    <div class="relative">
      <div class="absolute inset-y-0 left-3 flex items-center pointer-events-none">
        <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
      </div>
      <input
        id="search-nasabah"
        v-model="keyword"
        type="text"
        placeholder="Cari Nasabah..."
        class="w-full pl-10 pr-4 py-3 bg-white rounded-2xl shadow-sm border border-gray-100 text-sm focus:outline-none focus:ring-2 focus:ring-[#F5A623] transition"
        @input="onSearch"
      />
    </div>

    <!-- ── LOADING SKELETON ── -->
    <div v-if="adminStore.loadingNasabah" class="space-y-3">
      <div v-for="i in 5" :key="i" class="h-20 bg-gray-200 rounded-2xl animate-pulse" />
    </div>

    <!-- ── ERROR STATE ── -->
    <div
      v-else-if="adminStore.errorNasabah"
      class="bg-red-50 border border-red-200 rounded-2xl p-6 text-center"
    >
      <p class="text-red-500 text-sm">{{ adminStore.errorNasabah }}</p>
      <button @click="adminStore.fetchNasabah(keyword)" class="mt-3 text-xs text-red-600 underline">
        Coba lagi
      </button>
    </div>

    <!-- ── EMPTY STATE ── -->
    <div
      v-else-if="adminStore.nasabahList.length === 0"
      class="bg-white rounded-2xl p-8 text-center text-gray-400 text-sm shadow-sm"
    >
      <img src="../assets/admin/Nasabah.svg" class="w-12 h-12 mx-auto mb-3 opacity-30" alt="" />
      <p>Tidak ada nasabah ditemukan.</p>
    </div>

    <!-- ── DAFTAR NASABAH ── -->
    <div v-else class="space-y-3">
      <div
        v-for="nasabah in adminStore.nasabahList"
        :key="nasabah.user_id"
        class="flex items-center justify-between p-4 bg-white rounded-2xl shadow-sm active:bg-gray-50 transition"
        :class="{ 'cursor-pointer': isSelectMode }"
        @click="isSelectMode ? pilihNasabah(nasabah) : undefined"
      >
        <!-- Avatar + info -->
        <div class="flex items-center gap-3">
          <div
            class="w-11 h-11 rounded-full bg-[#F5A623] flex items-center justify-center font-bold text-white text-base shrink-0"
          >
            {{ nasabah.username.charAt(0).toUpperCase() }}
          </div>
          <div>
            <p class="font-semibold text-sm text-gray-800">{{ nasabah.username }}</p>
            <p class="text-xs text-gray-400">{{ nasabah.email }}</p>
          </div>
        </div>

        <!-- Saldo + aksi -->
        <div class="flex flex-col items-end gap-1 shrink-0">
          <p class="text-sm font-bold text-blue-600">
            Rp{{ nasabah.saldo.toLocaleString("id-ID") }}
          </p>
          <!-- Mode normal: tombol DETAIL -->
          <button
            v-if="!isSelectMode"
            @click.stop="goToDetail(nasabah.user_id)"
            class="text-[10px] font-bold text-[#F5A623] tracking-widest"
          >
            DETAIL
          </button>
          <!-- Mode select: indicator pilih -->
          <span v-else class="text-[10px] font-bold text-blue-500 tracking-widest">PILIH →</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue"
import { useRouter, useRoute } from "vue-router"
import { useAdminStore } from "../stores/adminStore"
import type { NasabahItem } from "../stores/adminStore"

const adminStore = useAdminStore()
const router = useRouter()
const route = useRoute()

// Deteksi mode: select (dipanggil dari modal setor) atau normal (browse)
const isSelectMode = computed(() => route.query.mode === "select")

const keyword = ref("")

// Debounce pencarian
let debounceTimer: ReturnType<typeof setTimeout> | null = null
const onSearch = () => {
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    adminStore.fetchNasabah(keyword.value)
  }, 400)
}

// Pilih nasabah (mode select) → simpan ke store → kembali ke dashboard
const pilihNasabah = (nasabah: NasabahItem) => {
  const forModal = route.query.for as string

  if (forModal === "penarikan") {
    adminStore.selectNasabahForPenarikan(nasabah.user_id, nasabah.username)
  } else {
    adminStore.selectNasabahForSetoran(nasabah.user_id, nasabah.username)
  }

  // Kembali ke dashboard dengan info modal mana yang harus dibuka
  router.push(`/admin-dashboard?mode=back-from-select&for=${forModal || "setoran"}`)
}

// Batal pilih → kembali ke dashboard dengan modal tetap terbuka
const cancelSelect = () => {
  const forModal = route.query.for as string
  router.push(`/admin-dashboard?mode=back-from-select&for=${forModal || "setoran"}`)
}

// Navigasi ke halaman detail nasabah (mode normal)
const goToDetail = (userId: string) => {
  router.push(`/admin-nasabah/${userId}`)
}

onMounted(() => {
  adminStore.fetchNasabah("")
})
</script>
