<template>
  <div class="p-4 bg-gray-100 min-h-screen space-y-5">

    <!-- 🔙 HEADER -->
    <div class="flex items-center gap-3">
      <img 
        :src="backIcon" 
        class="w-6 h-6 cursor-pointer"
        @click="goBack"
      />
      <h1 class="font-semibold text-lg">Dompet & Saldo</h1>
    </div>

    <!-- 🟢 BALANCE CARD -->
    <div v-if="userStore.loadingBalance" class="animate-pulse bg-gray-300 rounded-2xl h-24 w-full" />
    <div v-else-if="userStore.errorBalance" class="bg-red-100 text-red-600 text-sm px-4 py-3 rounded-xl">
      ⚠️ Gagal memuat saldo: {{ userStore.errorBalance }}
    </div>
    <BalanceCard
      v-else
      :balance="userStore.balance"
      :icon="walletIcon"
      bgColor="bg-[#4E5D4A]"
      :showActions="false"
    />

    <!-- 📊 RIWAYAT -->
    <div>
      <h2 class="font-semibold mb-3">Riwayat Transaksi</h2>

      <!-- 📅 FILTER TANGGAL -->
      <div class="mb-4">
        <input
          type="date"
          v-model="selectedDate"
          class="bg-green-100 text-green-700 px-4 py-2 rounded-xl text-sm"
        />
      </div>

      <!-- ⏳ LOADING SKELETON RIWAYAT -->
      <div v-if="userStore.loadingHistory" class="space-y-3">
        <div
          v-for="n in 3"
          :key="n"
          class="animate-pulse bg-gray-300 rounded-xl h-16 w-full"
        />
      </div>

      <!-- ❌ ERROR RIWAYAT -->
      <div
        v-else-if="userStore.errorHistory"
        class="bg-red-100 text-red-600 text-sm px-4 py-3 rounded-xl"
      >
        ⚠️ Gagal memuat riwayat: {{ userStore.errorHistory }}
      </div>

      <!-- 📭 KOSONG -->
      <div
        v-else-if="filteredTransactions.length === 0"
        class="text-center text-gray-400 text-sm py-8"
      >
        Belum ada transaksi.
      </div>

      <!-- 📋 LIST TRANSAKSI -->
      <div v-else class="space-y-3">
        <TransactionCard
          v-for="trx in filteredTransactions"
          :key="trx.id"
          :name="trx.name"
          :date="trx.date"
          :amount="trx.amount"
          :type="trx.type"
        />
      </div>

    </div>

  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue"
import { useRouter } from "vue-router"
import { useUserStore } from "../stores/userStore"

import BalanceCard from "../components/BalanceCard.vue"
import TransactionCard from "../components/TransactionCard.vue"

// ICON
import backIcon from "../assets/kembali.svg"
import walletIcon from "../assets/user/Dompet1.svg"

const router = useRouter()
const userStore = useUserStore()

// 🚀 FETCH DATA SAAT HALAMAN DIBUKA (saldo + riwayat paralel)
onMounted(() => {
  userStore.fetchBalance()
  userStore.fetchHistory()
})

// 📅 FILTER DATE
const selectedDate = ref("")

const filteredTransactions = computed(() => {
  if (!selectedDate.value) return userStore.transactions

  return userStore.transactions.filter(trx =>
    trx.date === formatDate(selectedDate.value)
  )
})

// 🧠 FORMAT DATE (biar cocok sama data yang sudah di-map)
const formatDate = (date: string) => {
  const d = new Date(date)
  return `${d.getDate()}/${d.getMonth() + 1}/${d.getFullYear()}`
}

// 🔙 BACK
const goBack = () => {
  router.push("/user-dashboard")
}
</script>