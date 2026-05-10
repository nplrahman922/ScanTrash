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
    <BalanceCard
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

      <!-- 📋 LIST TRANSAKSI -->
      <div class="space-y-3">
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
import { ref, computed } from "vue"
import { useRouter } from "vue-router"
import { useUserStore } from "../../stores/userStore"

import BalanceCard from "../components/BalanceCard.vue"
import TransactionCard from "../components/TransactionCard.vue"

// ICON
import backIcon from "../assets/kembali.svg"
import walletIcon from "../assets/userAset/Dompet1.svg"

const router = useRouter()
const userStore = useUserStore()

// 📅 FILTER DATE
const selectedDate = ref("")

const filteredTransactions = computed(() => {
  if (!selectedDate.value) return userStore.transactions

  return userStore.transactions.filter(trx =>
    trx.date === formatDate(selectedDate.value)
  )
})

// 🧠 FORMAT DATE (biar cocok sama data kamu)
const formatDate = (date: string) => {
  const d = new Date(date)
  return `${d.getDate()}/${d.getMonth() + 1}/${d.getFullYear()}`
}

// 🔙 BACK
const goBack = () => {
  router.push("/user-dashboard") // ke beranda
}
</script>