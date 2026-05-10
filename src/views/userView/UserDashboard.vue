<template>
  <div class="p-4 space-y-5 min-h-screen">

    <!-- 🟢 BALANCE -->
    <BalanceCard
      :balance="userStore.balance"
      :bgImage="balanceBg"
      @scan="goToScan"
      @history="goToWallet"
    />

    <!-- 🟡 INFO -->
    <div class="grid grid-cols-2 gap-4">
      <InfoCard
        title="Sampah Terkumpul"
        :value="userStore.totalTrash + ' Item'"
        :bgImage="trashBg"
      />

      <InfoCard
        title="Jadwal Setor"
        :value="userStore.schedule"
        :bgImage="scheduleBg"
      />
    </div>

    <!-- 🔵 TRANSAKSI -->
    <div>
      <div class="flex justify-between mb-3">
        <h2 class="font-semibold text-gray-900">Transaksi Terakhir</h2>
        <button 
          @click="goToWallet"
          class="text-[var(--color-primary)] text-sm font-medium">Lihat Semua</button>
      </div>

      <div class="space-y-3">
        <TransactionCard
          v-for="(trx, index) in userStore.transactions"
          :key="trx.id"
          :name="trx.name"
          :date="trx.date"
          :amount="trx.amount"
          :type="trx.type"
          :bgImage="index === 0 ? transaction1Bg : index === 1 ? transaction2Bg : ''"
        />
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { useUserStore } from "../../stores/userStore"
import { useRouter } from "vue-router"

import BalanceCard from "../components/BalanceCard.vue"
import InfoCard from "../components/InfoCard.vue"
import TransactionCard from "../components/TransactionCard.vue"

import balanceBg from "../../assets/userAset/userCard/saldo.webp"
import trashBg from "../../assets/userAset/userCard/sampah terkumpul.webp"
import scheduleBg from "../../assets/userAset/userCard/jadwal setor.webp"
import transaction1Bg from "../../assets/userAset/userCard/transaksi terakhir 1.webp"
import transaction2Bg from "../../assets/userAset/userCard/transaksi terakhir 2.webp"

const userStore = useUserStore()
const router = useRouter()

const goToScan = () => {
  router.push("/scan")
}

const goToWallet = () => {
  router.push("/wallet")
}
</script>