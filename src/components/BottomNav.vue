<script setup lang="ts">
import { useRouter } from "vue-router"

import IconHome from "../assets/Beranda.svg"
import IconWallet from "../assets/user/Dompet2.svg"
import IconScan from "../assets/user/Scan2.svg"

const router = useRouter()

const goTo = (path: string) => {
  router.push(path)
}
</script>

<template>
  <div class="fixed bottom-0 left-0 w-full bg-white border-t shadow-md z-50">
    
    <div class="relative flex justify-around items-center h-16">

      <!-- Left -->
      <button @click="goTo('/')" class="flex flex-col items-center text-sm text-blue-500">
        <img :src="IconHome" class="w-6 h-6" />
        <span>Beranda</span>
      </button>

      <div class="w-16"></div>

      <!-- Right -->
      <button @click="goTo('/wallet')" class="flex flex-col items-center text-sm text-blue-500">
        <img :src="IconWallet" class="w-6 h-6" />
        <span>Dompet</span>
      </button>

      <!-- Scan -->
      <button
        @click="goTo('/scan')"
        class="absolute -top-6 bg-green-500 w-14 h-14 rounded-full flex items-center justify-center shadow-lg border-4 border-white"
      >
        <img :src="IconScan" class="w-7 h-7" />
      </button>

    </div>
  </div>
</template>