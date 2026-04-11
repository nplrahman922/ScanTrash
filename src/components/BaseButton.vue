<script setup lang="ts">
import { computed } from "vue"

interface Props {
  label: string
  color?: string
  rounded?: string
  size?: "sm" | "md" | "lg"
}

const props = withDefaults(defineProps<Props>(), {
  color: "bg-blue-500",
  rounded: "rounded-lg",
  size: "md"
})

const sizeClass = computed(() => {
  switch (props.size) {
    case "sm":
      return "px-3 py-1 text-sm"
    case "lg":
      return "px-6 py-3 text-lg"
    default:
      return "px-4 py-2 text-base"
  }
})
</script>

<template>
  <button
    :class="[
      'text-white font-semibold transition duration-200',
      color,
      rounded,
      sizeClass,
      'hover:opacity-80'
    ]"
  >
    {{ label }}
  </button>
</template>