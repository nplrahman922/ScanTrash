<template>
  <div class="p-4 space-y-5 bg-gray-100 min-h-screen pb-24">

    <!-- ── BALANCE CARD ── -->
    <div class="rounded-2xl p-5 text-white shadow-md bg-[#F5A623] relative overflow-hidden">
      <div class="flex justify-between items-start">
        <div>
          <p class="text-sm font-medium opacity-90">Total Saldo Nasabah</p>
          <h1 class="text-3xl font-bold mt-1">
            <span v-if="adminStore.loadingDashboard" class="opacity-60 text-xl">Memuat...</span>
            <span v-else>Rp{{ adminStore.totalSaldo.toLocaleString("id-ID") }}</span>
          </h1>
        </div>
        <img src="../assets/user/Dompet1.svg" class="w-16 h-16 opacity-80" alt="saldo" />
      </div>

      <div class="flex gap-3 mt-4">
        <div class="flex-1 bg-white/25 rounded-xl px-4 py-2 text-center">
          <p class="text-xs opacity-80">Total Sampah</p>
          <p class="font-bold text-sm">0.0 Kg</p>
        </div>
        <div class="flex-1 bg-white/25 rounded-xl px-4 py-2 text-center">
          <p class="text-xs opacity-80">Total Nasabah</p>
          <p class="font-bold text-sm">
            <span v-if="adminStore.loadingDashboard">...</span>
            <span v-else>{{ adminStore.totalNasabah }}</span>
          </p>
        </div>
      </div>
    </div>

    <!-- ── QUICK ACTIONS ── -->
    <div class="grid grid-cols-3 gap-3">
      <button
        id="btn-setor"
        @click="openSetoranModal"
        class="flex flex-col items-center bg-white rounded-2xl p-4 shadow-sm gap-2 active:scale-95 transition-transform"
      >
        <div class="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center">
          <img src="../assets/admin/Nasabah.svg" class="w-5 h-5" alt="setor" />
        </div>
        <span class="text-xs font-semibold text-blue-600">Setor</span>
      </button>

      <button
        id="btn-tarik"
        @click="openPenarikanModal"
        class="flex flex-col items-center bg-white rounded-2xl p-4 shadow-sm gap-2 active:scale-95 transition-transform"
      >
        <div class="w-10 h-10 rounded-full bg-red-100 flex items-center justify-center">
          <img src="../assets/Riwayat.svg" class="w-5 h-5" alt="tarik" />
        </div>
        <span class="text-xs font-semibold text-red-500">Tarik</span>
      </button>

      <button
        class="flex flex-col items-center bg-white rounded-2xl p-4 shadow-sm gap-2 active:scale-95 transition-transform"
      >
        <div class="w-10 h-10 rounded-full bg-green-100 flex items-center justify-center">
          <img src="../assets/admin/Jadwal Setor.svg" class="w-5 h-5" alt="jadwal" />
        </div>
        <span class="text-xs font-semibold text-green-600">Jadwal</span>
      </button>
    </div>

    <!-- ── AKTIVITAS TERBARU ── -->
    <div>
      <div class="flex justify-between items-center mb-3">
        <h2 class="font-semibold text-gray-800">Aktivitas Terbaru</h2>
        <button @click="router.push('/admin-nasabah')" class="text-[#F5A623] text-sm font-medium">
          Lihat Semua
        </button>
      </div>

      <div v-if="adminStore.loadingDashboard" class="space-y-3">
        <div v-for="i in 3" :key="i" class="h-16 bg-gray-200 rounded-xl animate-pulse" />
      </div>

      <div
        v-else-if="adminStore.errorDashboard"
        class="bg-red-50 border border-red-200 rounded-xl p-4 text-center"
      >
        <p class="text-red-500 text-sm">{{ adminStore.errorDashboard }}</p>
        <button @click="adminStore.fetchDashboard()" class="mt-2 text-xs text-red-600 underline">
          Coba lagi
        </button>
      </div>

      <div
        v-else-if="adminStore.aktivitasTerbaru.length === 0"
        class="bg-white rounded-xl p-6 text-center text-gray-400 text-sm"
      >
        Belum ada aktivitas transaksi.
      </div>

      <div v-else class="space-y-3">
        <div
          v-for="item in adminStore.aktivitasTerbaru"
          :key="item.id"
          class="flex items-center justify-between p-4 bg-white rounded-xl shadow-sm"
        >
          <div class="flex items-center gap-3">
            <div
              :class="[
                'w-10 h-10 rounded-xl flex items-center justify-center font-bold text-white text-sm',
                item.transaction_type === 'income' ? 'bg-green-400' : 'bg-red-400',
              ]"
            >
              {{ item.username.charAt(0).toUpperCase() }}
            </div>
            <div>
              <p class="font-semibold text-sm text-gray-800">{{ item.username }}</p>
              <p class="text-xs text-gray-400">{{ item.date }} · {{ item.keterangan }}</p>
            </div>
          </div>
          <p
            :class="[
              'font-semibold text-sm',
              item.transaction_type === 'income' ? 'text-green-500' : 'text-red-500',
            ]"
          >
            {{ item.transaction_type === "income" ? "+" : "-" }}Rp{{
              item.nominal.toLocaleString("id-ID")
            }}
          </p>
        </div>
      </div>
    </div>
  </div>

  <!-- ══════════════════════════════════════════════════ -->
  <!-- MODAL SETOR (Bottom Sheet)                        -->
  <!-- ══════════════════════════════════════════════════ -->
  <Transition name="fade">
    <div
      v-if="adminStore.showSetoranModal"
      class="fixed inset-0 bg-black/40 z-40"
      @click="closeSetoranModal"
    />
  </Transition>

  <Transition name="slide-up">
    <div
      v-if="adminStore.showSetoranModal"
      class="fixed bottom-0 left-0 right-0 z-50 bg-white rounded-t-3xl shadow-2xl px-5 pt-5 pb-10"
    >
      <div class="w-10 h-1 bg-gray-200 rounded-full mx-auto mb-5" />
      <h2 class="text-lg font-bold text-gray-800 mb-5">Setor Sampah</h2>

      <!-- Sukses -->
      <div v-if="adminStore.successSetoran" class="text-center py-6">
        <div class="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-3">
          <svg class="w-8 h-8 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M5 13l4 4L19 7" />
          </svg>
        </div>
        <p class="font-bold text-gray-800 mb-1">Setoran Berhasil!</p>
        <p class="text-sm text-gray-500 mb-5">
          Rp{{ adminStore.setoranNominal.toLocaleString("id-ID") }} telah disetor untuk
          {{ adminStore.setoranTargetUsername }}
        </p>
        <button @click="closeSetoranModal" class="w-full py-3 bg-[#F5A623] text-white font-bold rounded-2xl">
          Selesai
        </button>
      </div>

      <!-- Form -->
      <div v-else class="space-y-4">
        <!-- Nasabah -->
        <div>
          <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1 block">Nasabah</label>
          <button
            id="btn-pilih-nasabah-setor"
            @click="router.push('/admin-nasabah?mode=select&for=setoran')"
            class="w-full flex items-center justify-between p-4 bg-gray-50 rounded-2xl border border-gray-200 text-left"
          >
            <p :class="adminStore.setoranTargetUserId ? 'text-gray-800 font-medium' : 'text-gray-400'">
              {{ adminStore.setoranTargetUserId ? adminStore.setoranTargetUsername : "Pilih Nasabah" }}
            </p>
            <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          </button>
        </div>

        <!-- Keterangan -->
        <div>
          <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1 block">Keterangan</label>
          <input
            id="input-keterangan-setor"
            v-model="adminStore.setoranKeterangan"
            type="text"
            placeholder="Contoh: Botol Plastik, Kertas, dll."
            class="w-full p-4 bg-gray-50 rounded-2xl border border-gray-200 text-sm focus:outline-none focus:ring-2 focus:ring-[#F5A623]"
          />
        </div>

        <!-- Nominal -->
        <div>
          <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1 block">Nominal (Rp)</label>
          <div class="relative">
            <span class="absolute inset-y-0 left-4 flex items-center text-gray-500 font-medium text-sm">Rp</span>
            <input
              id="input-nominal-setor"
              v-model.number="adminStore.setoranNominal"
              type="number"
              min="0"
              placeholder="0"
              class="w-full pl-10 pr-4 py-4 bg-gray-50 rounded-2xl border border-gray-200 text-sm focus:outline-none focus:ring-2 focus:ring-[#F5A623]"
            />
          </div>
        </div>

        <p v-if="adminStore.errorSetoran" class="text-red-500 text-sm text-center">
          {{ adminStore.errorSetoran }}
        </p>

        <button
          id="btn-konfirmasi-setoran"
          @click="adminStore.createSetoran()"
          :disabled="adminStore.loadingSetoran"
          class="w-full py-4 bg-[#F5A623] text-white font-bold rounded-2xl disabled:opacity-60 active:scale-95 transition-transform"
        >
          {{ adminStore.loadingSetoran ? "Memproses..." : "Konfirmasi Setoran" }}
        </button>
      </div>
    </div>
  </Transition>

  <!-- ══════════════════════════════════════════════════ -->
  <!-- MODAL PENARIKAN (Bottom Sheet)                    -->
  <!-- ══════════════════════════════════════════════════ -->
  <Transition name="fade">
    <div
      v-if="adminStore.showPenarikanModal"
      class="fixed inset-0 bg-black/40 z-40"
      @click="closePenarikanModal"
    />
  </Transition>

  <Transition name="slide-up">
    <div
      v-if="adminStore.showPenarikanModal"
      class="fixed bottom-0 left-0 right-0 z-50 bg-white rounded-t-3xl shadow-2xl px-5 pt-5 pb-10"
    >
      <div class="w-10 h-1 bg-gray-200 rounded-full mx-auto mb-5" />
      <h2 class="text-lg font-bold text-gray-800 mb-5">Tarik Saldo</h2>

      <!-- Sukses -->
      <div v-if="adminStore.successPenarikan" class="text-center py-6">
        <div class="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-3">
          <svg class="w-8 h-8 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M5 13l4 4L19 7" />
          </svg>
        </div>
        <p class="font-bold text-gray-800 mb-1">Penarikan Berhasil!</p>
        <p class="text-sm text-gray-500 mb-5">
          Rp{{ adminStore.penarikanNominal.toLocaleString("id-ID") }} berhasil ditarik dari saldo
          {{ adminStore.penarikanTargetUsername }}
        </p>
        <button @click="closePenarikanModal" class="w-full py-3 bg-red-500 text-white font-bold rounded-2xl">
          Selesai
        </button>
      </div>

      <!-- Form -->
      <div v-else class="space-y-4">
        <!-- Nasabah -->
        <div>
          <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1 block">Nasabah</label>
          <button
            id="btn-pilih-nasabah-tarik"
            @click="router.push('/admin-nasabah?mode=select&for=penarikan')"
            class="w-full flex items-center justify-between p-4 bg-gray-50 rounded-2xl border border-gray-200 text-left"
          >
            <p :class="adminStore.penarikanTargetUserId ? 'text-gray-800 font-medium' : 'text-gray-400'">
              {{ adminStore.penarikanTargetUserId ? adminStore.penarikanTargetUsername : "Pilih Nasabah" }}
            </p>
            <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          </button>
        </div>

        <!-- Keterangan -->
        <div>
          <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1 block">Keterangan</label>
          <input
            id="input-keterangan-tarik"
            v-model="adminStore.penarikanKeterangan"
            type="text"
            placeholder="Contoh: Pencairan tunai, dll."
            class="w-full p-4 bg-gray-50 rounded-2xl border border-gray-200 text-sm focus:outline-none focus:ring-2 focus:ring-red-400"
          />
        </div>

        <!-- Nominal -->
        <div>
          <label class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1 block">Jumlah Penarikan (Rp)</label>
          <div class="relative">
            <span class="absolute inset-y-0 left-4 flex items-center text-gray-500 font-medium text-sm">Rp</span>
            <input
              id="input-nominal-tarik"
              v-model.number="adminStore.penarikanNominal"
              type="number"
              min="0"
              placeholder="0"
              class="w-full pl-10 pr-4 py-4 bg-gray-50 rounded-2xl border border-gray-200 text-sm focus:outline-none focus:ring-2 focus:ring-red-400"
            />
          </div>
        </div>

        <p v-if="adminStore.errorPenarikan" class="text-red-500 text-sm text-center">
          {{ adminStore.errorPenarikan }}
        </p>

        <button
          id="btn-proses-penarikan"
          @click="adminStore.createPenarikan()"
          :disabled="adminStore.loadingPenarikan"
          class="w-full py-4 bg-red-500 text-white font-bold rounded-2xl disabled:opacity-60 active:scale-95 transition-transform"
        >
          {{ adminStore.loadingPenarikan ? "Memproses..." : "Proses Penarikan" }}
        </button>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { onMounted } from "vue"
import { useRouter, useRoute } from "vue-router"
import { useAdminStore } from "../stores/adminStore"

const adminStore = useAdminStore()
const router = useRouter()
const route = useRoute()

const openSetoranModal = () => {
  adminStore.resetSetoranForm()
  adminStore.showSetoranModal = true
}

const closeSetoranModal = () => {
  adminStore.showSetoranModal = false
  adminStore.resetSetoranForm()
}

const openPenarikanModal = () => {
  adminStore.resetPenarikanForm()
  adminStore.showPenarikanModal = true
}

const closePenarikanModal = () => {
  adminStore.showPenarikanModal = false
  adminStore.resetPenarikanForm()
}

onMounted(() => {
  adminStore.fetchDashboard()

  // Kembali dari halaman nasabah (mode select) — buka modal yang sesuai
  const modeBack = route.query.mode
  const forModal = route.query.for

  if (modeBack === "back-from-select") {
    if (forModal === "penarikan") {
      adminStore.showPenarikanModal = true
    } else {
      // default: setoran
      adminStore.showSetoranModal = true
    }
  }
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