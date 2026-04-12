<template>
  <div class="flex items-center justify-between p-4 bg-white rounded-xl shadow-sm">
    
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
const props = defineProps({
  name: String,
  date: String,
  amount: {
    type: Number,
    required: true
  },
  type: String
})

const isIncome = props.type === "income"
</script>