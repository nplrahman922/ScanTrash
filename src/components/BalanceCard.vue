<template>
  <div :class="['rounded-2xl p-5 text-white shadow-md', bgColor]">
    
    <div class="flex justify-between items-center">
      <div>
        <p class="text-sm opacity-80">Total Saldo Anda</p>
        <h1 class="text-2xl font-bold">
          Rp{{ balance.toLocaleString("id-ID") }}
        </h1>
      </div>

      <img :src="icon" class="w-14 h-14" />
    </div>

    <!-- 🔘 ACTION (OPSIONAL) -->
    <div 
      v-if="showActions" 
      class="flex gap-3 mt-4"
    >
      <button 
        @click="emit('scan')"
        class="flex-1 bg-white/30 py-2 rounded-xl flex items-center justify-center gap-2"
      >
        <img :src="scanIcon" class="w-4 h-4" />
        Scan
      </button>

      <button 
        @click="emit('history')"
        class="flex-1 bg-white/30 py-2 rounded-xl flex items-center justify-center gap-2"
      >
        <img :src="historyIcon" class="w-4 h-4" />
        Riwayat
      </button>
    </div>

  </div>
</template>

<script setup lang="ts">
defineProps({
  balance: {
    type: Number,
    required: true
  },
  icon: String,
  scanIcon: String,
  historyIcon: String,
  bgColor: String,
  
showActions: {
    type: Boolean,
    default: true
  }
})

const emit = defineEmits(['scan', 'history'])
</script>