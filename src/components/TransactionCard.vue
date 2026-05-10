<template>
  <div class="relative overflow-hidden rounded-xl shadow-sm" :style="bgStyle">
    <div v-if="bgImage" class="absolute inset-0 bg-black/10"></div>

    <div class="relative flex items-center justify-between p-4 bg-white/75 backdrop-blur-sm">
      <div class="flex items-center gap-3">
      <div :class="[
        'w-10 h-10 rounded-lg flex items-center justify-center',
        isIncome ? 'bg-green-100' : 'bg-red-100'
      ]">
        <span>{{ isIncome ? '↩' : '🚀' }}</span>
      </div>

      <div>
        <p class="font-semibold text-sm">{{ name }}</p>
        <p class="text-xs text-gray-500">{{ date }}</p>
      </div>
    </div>

    <p :class="[
      'font-semibold',
      isIncome ? 'text-green-500' : 'text-red-500'
    ]">
      {{ isIncome ? '+' : '-' }}Rp{{ amount.toLocaleString("id-ID") }}
    </p>

  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"

const props = defineProps({
  name: String,
  date: String,
  amount: {
    type: Number,
    required: true
  },
  type: String,
  bgImage: String
})

const isIncome = props.type === "income"
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