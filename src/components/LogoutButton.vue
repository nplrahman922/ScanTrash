<script setup lang="ts">
import { useRouter } from "vue-router"
import { useAuthStore } from "../stores/authStore"

const props = defineProps({
  label: { type: String, default: "Logout" },
  bgColor: { type: String, default: "var(--color-secondary)" },
  textColor: { type: String, default: "#ffffff" },
  hoverColor: { type: String, default: "rgba(250,161,17,0.9)" },
  borderColor: { type: String, default: "var(--color-secondary)" },
  iconSrc: { type: String, default: "" }
})
const emit = defineEmits<["click"]>()

const authStore = useAuthStore()
const router = useRouter()

const handleLogout = async () => {
  emit("click")
  await authStore.logout()
  router.push("/login")
}
</script>

<template>
  <button
    @click="handleLogout"
    :disabled="authStore.loading"
    :style="{
      '--bg-color': props.bgColor,
      '--hover-bg': props.hoverColor,
      '--border-color': props.borderColor,
      color: props.textColor
    }"
    class="logout-button flex items-center justify-center gap-2 w-full border px-4 py-2 rounded-lg transition duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
  >
    <img v-if="props.iconSrc" :src="props.iconSrc" alt="Logout icon" class="w-5 h-5" />
    <span>{{ authStore.loading ? "Logging out..." : props.label }}</span>
  </button>
</template>

<style scoped>
.logout-button {
  background-color: var(--bg-color);
  border-color: var(--border-color);
}
.logout-button:hover {
  background-color: var(--hover-bg);
}
</style>