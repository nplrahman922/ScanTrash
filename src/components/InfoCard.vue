<template>
  <div :class="['p-4 rounded-2xl shadow-md w-full', bgColor]">
    
    <img :src="icon" class="w-8 h-8 mb-2" />

    <p class="text-sm opacity-70">{{ title }}</p>
    <p class="font-bold text-lg">{{ value }}</p>

  </div>
</template>

<script setup lang="ts">
defineProps({
  icon: String,
  title: String,
  value: String,
  bgColor: String
})
</script>