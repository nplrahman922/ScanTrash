<template>
  <div :class="[
    'flex gap-4 p-4 rounded-2xl shadow-md',
    highlight ? 'bg-green-200' : 'bg-gray-200'
  ]">

    <!-- 🌿 ICON -->
    <div class="flex items-start">
      <img :src="icon" class="w-10 h-10" />
    </div>

    <!-- 📄 TEXT -->
    <div>
      <h3 class="font-semibold text-sm mb-1">
        {{ title }}
      </h3>
      <p class="text-xs text-gray-700">
        {{ description }}
      </p>
    </div>

  </div>
</template>

<script setup lang="ts">
defineProps({
  title: String,
  description: String,
  icon: String,
  highlight: Boolean
})
</script>