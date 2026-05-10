<template>
  <div
    :class="['relative overflow-hidden p-4 rounded-2xl shadow-md w-full', bgColor]"
    :style="bgStyle"
  >
    <div v-if="bgImage" class="absolute inset-0 bg-black/10"></div>
    <div class="relative">
      <img v-if="icon && !bgImage" :src="icon" class="w-8 h-8 mb-2" />

    <p class="text-sm opacity-80 text-black/80">{{ title }}</p>
    <p class="font-bold text-lg text-black">{{ value }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"

const props = defineProps({
  icon: String,
  title: String,
  value: String,
  bgColor: String,
  bgImage: String
})

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