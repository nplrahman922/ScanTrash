<template>
  <div>
    <!-- Menu Burger Button -->
    <button
      class="cursor-pointer w-12 h-12 p-2 rounded-lg bg-white shadow-md"
      @click="toggleSidebar"
      aria-label="Open menu"
    >
      <img :src="sidebarIcon" alt="Menu" class="w-full h-full object-contain" />
    </button>

    <!-- Sidebar + overlay -->
    <transition name="fade">
      <div v-if="isSidebarOpen" class="fixed inset-0 z-40">
        <div
          class="absolute inset-0 bg-black/30"
          @click="closeSidebar"
        />

        <aside class="absolute left-0 top-0 h-full w-72 bg-white shadow-2xl border-r"
          :style="{ borderColor: 'var(--color-tertiary)' }"
        >
          <div class="flex items-center justify-between px-5 py-4 border-b"
            :style="{ borderColor: 'var(--color-tertiary)' }"
          >
            <img :src="logoIcon" alt="ScanTrash" class="w-28 h-auto object-contain" />
            <button
              class="p-2 rounded-md hover:bg-gray-100"
              @click="closeSidebar"
              aria-label="Close menu"
            >
              <img :src="exitIcon" alt="Close" class="w-5 h-5 object-contain" />
            </button>
          </div>

          <nav class="flex-1 flex flex-col py-5">
            <router-link
              to="/user-dashboard"
              class="flex items-center gap-3 px-5 py-4 text-gray-800 text-base hover:bg-gray-100 transition-colors"
              @click="closeSidebar"
            >
              <img :src="dashboardIcon" alt="Dashboard" class="w-6 h-6 object-contain" />
              <span>Dashboard</span>
            </router-link>

            <router-link
              to="/jenis-sampah"
              class="flex items-center gap-3 px-5 py-4 text-gray-800 text-base hover:bg-gray-100 transition-colors"
              @click="closeSidebar"
            >
              <img :src="jenisSampahIcon" alt="Jenis Sampah" class="w-6 h-6 object-contain" />
              <span>Jenis Sampah</span>
            </router-link>

            <router-link
              to="/tips-memilah-sampah"
              class="flex items-center gap-3 px-5 py-4 text-gray-800 text-base hover:bg-gray-100 transition-colors"
              @click="closeSidebar"
            >
              <img :src="tipsIcon" alt="Tips Memilah" class="w-6 h-6 object-contain" />
              <span>Tips Memilah</span>
            </router-link>

            <div class="h-px bg-gray-200 my-4"></div>

            <div class="px-5">
              <LogoutButton
                label="Log Out"
                :iconSrc="logoutIcon"
                bgColor="transparent"
                textColor="var(--color-secondary)"
                borderColor="var(--color-secondary)"
                hoverColor="rgba(250,161,17,0.12)"
                class="w-full"
                @click="closeSidebar"
              />
            </div>
          </nav>
        </aside>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue"
import LogoutButton from "../components/LogoutButton.vue"
import sidebarIcon from "../assets/Sidebar.svg"
import logoIcon from "../assets/Logo2.svg"
import exitIcon from "../assets/Exit1.svg"
import dashboardIcon from "../assets/Dashboard.svg"
import jenisSampahIcon from "../assets/userAset/Jenis Sampah.svg"
import tipsIcon from "../assets/userAset/Tips Memilah.svg"
import logoutIcon from "../assets/userAset/LogOut.svg"

const isSidebarOpen = ref(false)

const toggleSidebar = () => {
  isSidebarOpen.value = !isSidebarOpen.value
}

const closeSidebar = () => {
  isSidebarOpen.value = false
}
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>