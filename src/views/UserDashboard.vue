<template>
  <div class="p-4 space-y-5 bg-gray-100 min-h-screen">

    <!-- 🟢 BALANCE -->
    <BalanceCard
      :balance="userStore.balance"
      :icon="walletIcon"
      :scanIcon="scanIcon"
      :historyIcon="historyIcon"
      bgColor="bg-green-500"
      @scan="goToScan"
      @history="goToWallet"
    />

    <!-- 🟡 INFO -->
    <div class="grid grid-cols-2 gap-4">
      <InfoCard
        :icon="trashIcon"
        title="Sampah Terkumpul"
        :value="userStore.totalTrash + ' Item'"
        bgColor="bg-blue-100"
      />

      <InfoCard
        :icon="calendarIcon"
        title="Jadwal Setor"
        :value="userStore.schedule"
        bgColor="bg-yellow-100"
      />
    </div>

    <!-- 🔵 TRANSAKSI -->
    <div>
      <div class="flex justify-between mb-3">
        <h2 class="font-semibold">Transaksi Terakhir</h2>
        <button 
          @click="goToWallet"
          class="text-green-500 text-sm">Lihat Semua</button>
      </div>

      <div class="space-y-3">
        <TransactionCard
          v-for="trx in userStore.transactions"
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
import { useUserStore } from "../stores/userStore"
import { useRouter } from "vue-router"

import BalanceCard from "../components/BalanceCard.vue"
import InfoCard from "../components/InfoCard.vue"
import TransactionCard from "../components/TransactionCard.vue"

// icon
import walletIcon from "../assets/user/Dompet1.svg"
import scanIcon from "../assets/user/Scan1.svg"
import historyIcon from "../assets/Riwayat.svg"
import trashIcon from "../assets/user/Jumlah Item.svg"
import calendarIcon from "../assets/user/Jadwal.svg"

const userStore = useUserStore()
const router = useRouter()

const goToScan = () => {
  router.push("/scan")
}

const goToWallet = () => {
  router.push("/wallet")
}
</script>