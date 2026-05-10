<template>
  <div
    :class="['relative overflow-hidden rounded-2xl p-5 text-white shadow-md', bgColor]"
    :style="bgStyle"
  >
    <div v-if="bgImage" class="absolute inset-0 bg-black/20"></div>

    <div class="relative">
      <div class="flex justify-between items-start">
      <div>
        <p class="text-sm opacity-80">Total Saldo Anda</p>
        <h1 class="text-2xl font-bold">
          Rp{{ balance.toLocaleString("id-ID") }}
        </h1>
      </div>

      <img v-if="icon" :src="icon" class="w-14 h-14" />
    </div>

    <!-- 🔘 ACTION (OPSIONAL) -->
    <div 
      v-if="showActions" 
      class="flex gap-3 mt-6"
    >
      <button 
        @click="emit('scan')"
        class="flex-1 bg-white/30 py-3 rounded-xl flex items-center justify-center gap-2 text-sm"
      >
        <img v-if="scanIcon" :src="scanIcon" class="w-4 h-4" />
        Scan
      </button>

      <button 
        @click="emit('history')"
        class="flex-1 bg-white/30 py-3 rounded-xl flex items-center justify-center gap-2 text-sm"
      >
        <img v-if="historyIcon" :src="historyIcon" class="w-4 h-4" />
        Riwayat
      </button>
    </div>

  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"

const props = defineProps({
  balance: {
    type: Number,
    required: true
  },
  icon: String,
  scanIcon: String,
  historyIcon: String,
  bgColor: String,
  bgImage: String,
  showActions: {
    type: Boolean,
    default: true
  }
})

const emit = defineEmits(['scan', 'history'])

const bgStyle = computed(() => {
  return props.bgImage
    ? {
        backgroundImage: `url(${props.bgImage})`,
        backgroundSize: 'cover',
        backgroundPosition: 'center',
        backgroundRepeat: 'no-repeat'
      }
    : {}
})
</script>