<template>
  <div class="p-4 bg-gray-100 min-h-screen pb-24">

    <!-- ── HEADER ── -->
    <div class="flex items-center justify-between mb-4">
      <h1 class="text-lg font-bold text-gray-800">Jadwal Setor</h1>
      <div class="flex gap-2">
        <button
          id="btn-tambah-jadwal"
          @click="adminStore.openAddScheduleModal()"
          class="flex items-center gap-1 px-3 py-2 bg-white border border-gray-300 rounded-xl text-sm font-semibold text-gray-700 shadow-sm active:scale-95 transition-transform"
        >
          <span class="text-base">+</span> Tambah
        </button>
        <button
          class="flex items-center gap-1 px-3 py-2 bg-[#3B82F6] rounded-xl text-sm font-semibold text-white shadow-sm active:scale-95 transition-transform"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" />
          </svg>
          Bagikan
        </button>
      </div>
    </div>

    <!-- ── LOADING SKELETON ── -->
    <div v-if="adminStore.loadingSchedule" class="space-y-4">
      <div v-for="i in 3" :key="i" class="h-36 bg-gray-200 rounded-2xl animate-pulse" />
    </div>

    <!-- ── ERROR STATE ── -->
    <div
      v-else-if="adminStore.errorSchedule"
      class="bg-red-50 border border-red-200 rounded-2xl p-6 text-center"
    >
      <p class="text-red-500 text-sm">{{ adminStore.errorSchedule }}</p>
      <button @click="adminStore.fetchSchedules()" class="mt-3 text-xs text-red-600 underline">
        Coba lagi
      </button>
    </div>

    <!-- ── EMPTY STATE ── -->
    <div
      v-else-if="adminStore.scheduleList.length === 0"
      class="bg-white rounded-2xl p-10 text-center"
    >
      <div class="w-14 h-14 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-3">
        <img src="../assets/admin/Jadwal Setor.svg" class="w-7 h-7 opacity-40" alt="" />
      </div>
      <p class="text-gray-400 text-sm">Belum ada jadwal setor.</p>
      <button
        @click="adminStore.openAddScheduleModal()"
        class="mt-4 px-5 py-2 bg-[#F5A623] text-white text-sm font-semibold rounded-xl"
      >
        + Tambah Jadwal
      </button>
    </div>

    <!-- ── DAFTAR JADWAL ── -->
    <div v-else class="space-y-4">
      <div
        v-for="jadwal in adminStore.scheduleList"
        :key="jadwal.id_tanggal"
        class="bg-white rounded-2xl shadow-sm overflow-hidden"
      >
        <!-- Badge hari (warna berdasarkan hari) -->
        <div :class="['px-4 py-2 inline-block rounded-br-xl text-white text-xs font-bold', getDayColor(jadwal.tanggal)]">
          {{ getDayName(jadwal.tanggal) }}
        </div>

        <!-- Detail jadwal -->
        <div class="px-4 pb-3 space-y-2 mt-1">
          <!-- Tanggal -->
          <div class="flex items-center gap-2 text-sm text-gray-700">
            <svg class="w-4 h-4 text-gray-400 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
            <span class="font-medium">{{ formatTanggal(jadwal.tanggal) }}</span>
          </div>

          <!-- Jam -->
          <div class="flex items-center gap-2 text-sm text-gray-700">
            <svg class="w-4 h-4 text-gray-400 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span>{{ formatWaktu(jadwal.waktu_buka) }} – {{ formatWaktu(jadwal.waktu_tutup) }} WITA</span>
          </div>

          <!-- Lokasi -->
          <div class="flex items-center gap-2 text-sm text-gray-700">
            <svg class="w-4 h-4 text-gray-400 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <span>{{ jadwal.lokasi }}</span>
          </div>

          <!-- Aksi: Hapus + Edit -->
          <div v-if="adminStore.deletingScheduleId !== jadwal.id_tanggal" class="flex gap-2 pt-1">
            <button
              @click="adminStore.deletingScheduleId = jadwal.id_tanggal ?? null"
              class="flex-1 py-2 border border-gray-200 rounded-xl text-sm font-semibold text-gray-600 active:bg-gray-50"
            >
              Hapus
            </button>
            <button
              @click="adminStore.openEditScheduleModal(jadwal)"
              class="flex-1 py-2 bg-gray-100 rounded-xl text-sm font-semibold text-gray-700 active:bg-gray-200"
            >
              Edit
            </button>
          </div>

          <!-- Konfirmasi hapus inline -->
          <div v-else class="pt-1">
            <p class="text-xs text-red-500 text-center mb-2">Yakin ingin menghapus jadwal ini?</p>
            <div class="flex gap-2">
              <button
                @click="adminStore.deletingScheduleId = null"
                class="flex-1 py-2 border border-gray-200 rounded-xl text-sm font-medium text-gray-600"
              >
                Batal
              </button>
              <button
                @click="hapusJadwal(jadwal.id_tanggal!)"
                class="flex-1 py-2 bg-red-500 rounded-xl text-sm font-bold text-white active:bg-red-600"
              >
                Hapus
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- ══════════════════════════════════════════════════════ -->
  <!-- MODAL TAMBAH / EDIT JADWAL (Bottom Sheet)            -->
  <!-- ══════════════════════════════════════════════════════ -->
  <Transition name="fade">
    <div
      v-if="adminStore.showScheduleModal"
      class="fixed inset-0 bg-black/40 z-40"
      @click="adminStore.closeScheduleModal()"
    />
  </Transition>

  <Transition name="slide-up">
    <div
      v-if="adminStore.showScheduleModal"
      class="fixed bottom-0 left-0 right-0 z-50 bg-white rounded-t-3xl shadow-2xl px-5 pt-5 pb-10"
    >
      <div class="w-10 h-1 bg-gray-200 rounded-full mx-auto mb-5" />
      <h2 class="text-lg font-bold text-gray-800 mb-5">
        {{ adminStore.editingScheduleId ? "Edit Jadwal" : "Tambah Jadwal" }}
      </h2>

      <div class="space-y-4">
        <!-- Pilih Tanggal -->
        <div>
          <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1 block">
            Pilih Tanggal
          </label>
          <div class="relative">
            <input
              id="input-tanggal-jadwal"
              v-model="adminStore.scheduleForm.tanggal"
              type="date"
              class="w-full p-4 bg-gray-50 rounded-2xl border border-gray-200 text-sm focus:outline-none focus:ring-2 focus:ring-[#F5A623]"
            />
          </div>
        </div>

        <!-- Waktu Buka -->
        <div>
          <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1 block">
            Waktu Buka (WITA)
          </label>
          <div class="relative">
            <input
              id="input-waktu-buka"
              v-model="adminStore.scheduleForm.waktuBuka"
              type="time"
              class="w-full p-4 bg-gray-50 rounded-2xl border border-gray-200 text-sm focus:outline-none focus:ring-2 focus:ring-[#F5A623]"
            />
          </div>
        </div>

        <!-- Waktu Tutup -->
        <div>
          <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1 block">
            Waktu Tutup (WITA)
          </label>
          <div class="relative">
            <input
              id="input-waktu-tutup"
              v-model="adminStore.scheduleForm.waktuTutup"
              type="time"
              class="w-full p-4 bg-gray-50 rounded-2xl border border-gray-200 text-sm focus:outline-none focus:ring-2 focus:ring-[#F5A623]"
            />
          </div>
        </div>

        <!-- Lokasi -->
        <div>
          <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1 block">
            Lokasi
          </label>
          <input
            id="input-lokasi-jadwal"
            v-model="adminStore.scheduleForm.lokasi"
            type="text"
            placeholder="Contoh: Pos 1, Pos 2, Gedung A"
            class="w-full p-4 bg-gray-50 rounded-2xl border border-gray-200 text-sm focus:outline-none focus:ring-2 focus:ring-[#F5A623]"
          />
        </div>

        <!-- Error -->
        <p v-if="adminStore.errorScheduleForm" class="text-red-500 text-sm text-center">
          {{ adminStore.errorScheduleForm }}
        </p>

        <!-- Submit -->
        <button
          id="btn-submit-jadwal"
          @click="adminStore.submitSchedule()"
          :disabled="adminStore.loadingScheduleForm"
          class="w-full py-4 bg-green-500 text-white font-bold rounded-2xl disabled:opacity-60 active:scale-95 transition-transform"
        >
          <span v-if="adminStore.loadingScheduleForm">Menyimpan...</span>
          <span v-else-if="adminStore.editingScheduleId">Simpan Perubahan</span>
          <span v-else>Tambah Jadwal</span>
        </button>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { onMounted } from "vue"
import { useAdminStore } from "../stores/adminStore"

const adminStore = useAdminStore()

// ── Helpers untuk format tampilan ──────────────────────────────────────────────

const HARI = ["Minggu", "Senin", "Selasa", "Rabu", "Kamis", "Jumat", "Sabtu"]
const DAY_COLORS = [
  "bg-red-400",    // Minggu
  "bg-blue-400",   // Senin
  "bg-green-400",  // Selasa
  "bg-purple-400", // Rabu
  "bg-[#F5A623]",  // Kamis
  "bg-teal-400",   // Jumat
  "bg-gray-500",   // Sabtu
]

const getDayName = (tanggal: string): string => {
  // Tambah T00:00:00 agar tidak kena offset timezone browser
  const date = new Date(tanggal + "T00:00:00")
  return HARI[date.getDay()]
}

const getDayColor = (tanggal: string): string => {
  const date = new Date(tanggal + "T00:00:00")
  return DAY_COLORS[date.getDay()]
}

const formatTanggal = (tanggal: string): string => {
  // "YYYY-MM-DD" → "DD/MM/YYYY"
  const [y, m, d] = tanggal.split("-")
  return `${d}/${m}/${y}`
}

const formatWaktu = (waktu: string): string => {
  // "10:00:00+08" atau "10:00:00" → "10:00"
  return waktu.slice(0, 5)
}

// ── Hapus dengan error handling ────────────────────────────────────────────────

const hapusJadwal = async (id: string) => {
  try {
    await adminStore.deleteSchedule(id)
  } catch {
    // Error sudah di-log di store, tampilkan pesan lokal jika perlu
    alert("Gagal menghapus jadwal. Silakan coba lagi.")
  }
}

onMounted(() => {
  adminStore.fetchSchedules()
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
