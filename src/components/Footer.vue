<script setup lang="ts">
import { computed } from "vue"
import { storeToRefs } from "pinia"
import { useAuthStore } from "../stores/authStore"

import FooterAdmin from "./FooterAdmin.vue"
import BottomNav from "./BottomNav.vue"

const authStore = useAuthStore()
const { userProfile } = storeToRefs(authStore)

const isAdmin = computed(() => userProfile.value?.role === "admin")
</script>

<template>
  <FooterAdmin v-if="isAdmin" />
  <BottomNav v-else />
</template>