<script setup lang="ts">
import { computed } from "vue"
import { storeToRefs } from "pinia"
import { useAuthStore } from "../stores/authStore"

import SideBarAdmin from "./SideBarAdmin.vue"
import SideBarUser from "./SideBarUser.vue"
import ProfileInfo from "../components/ProfileInfo.vue"

const authStore = useAuthStore()
const { userProfile } = storeToRefs(authStore)

const isAdmin = computed(() => userProfile.value?.role === "admin")
</script>

<template>
  <header class="flex items-center justify-between px-4 py-2 bg-white shadow-md">
    
    <SideBarAdmin v-if="isAdmin" />
    <SideBarUser v-else />

    <img class="logo" src="../assets/Logo2.svg" alt="Logo" />

    <ProfileInfo />
  </header>
</template>