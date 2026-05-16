<template>
  <div>
    <!-- Menu Burger Button -->
    <img 
      class="cursor-pointer w-12 h-12" 
      src="../assets/Sidebar.svg" 
      alt="Menu" 
      @click="toggleSidebar"
    />

    <!-- Sidebar -->
    <div class="sidebar fixed top-0 -left-72 w-72 h-screen bg-white shadow-lg transition-all duration-300 ease-in-out z-50 flex flex-col" :class="{ 'left-0': isSidebarOpen }">
      <!-- Header dengan Logo dan Exit Button -->
      <div class="flex justify-between items-center px-4 border-b border-gray-300">
        <img class="w-36 h-36" src="../assets/Logo2.svg" alt="Logo" />
        <button class="bg-none border-none cursor-pointer p-1" @click="closeSidebar">
          <img class="w-6 h-6" src="../assets/Exit1.svg" alt="Exit" />
        </button>
      </div>

      <!-- Menu Items -->
      <nav class="flex-1 flex flex-col py-5">

        <router-link 
          to="/admin-dashboard"
          class="flex items-center py-4 px-5 text-gray-800 text-base hover:bg-gray-100 transition-colors"
          @click="closeSidebar"
        >
          <img class="w-6 h-6 mr-3" src="../assets/Dashboard.svg" />
          Dashboard
        </router-link>

        <router-link 
          to="/admin-nasabah"
          class="flex items-center py-4 px-5 text-gray-800 text-base hover:bg-gray-100"
          @click="closeSidebar"
        >
          <img class="w-6 h-6 mr-3" src="../assets/admin/Nasabah.svg" />
          Nasabah
        </router-link>

        <router-link 
          to="/admin-katalog"
          class="flex items-center py-4 px-5 hover:bg-gray-100"
          @click="closeSidebar"
        >
          <img class="w-6 h-6 mr-3" src="../assets/admin/Katalog Harga.svg" />
          Katalog Harga
        </router-link>

        <router-link 
          to="/riwayat"
          class="flex items-center py-4 px-5 hover:bg-gray-100"
          @click="closeSidebar"
        >
          <img class="w-6 h-6 mr-3" src="../assets/Riwayat.svg" />
          Riwayat
        </router-link>

        <router-link 
          to="/admin-jadwal"
          class="flex items-center py-4 px-5 hover:bg-gray-100"
          @click="closeSidebar"
        >
          <img class="w-6 h-6 mr-3" src="../assets/admin/Jadwal Setor.svg" />
          Jadwal Setor
        </router-link>

        <div class="h-px bg-gray-300 my-5"></div>

        <LogoutButton />

      </nav>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue"
import LogoutButton from "../components/LogoutButton.vue"

const isSidebarOpen = ref(false)

const toggleSidebar = () => {
  isSidebarOpen.value = !isSidebarOpen.value
}

const closeSidebar = () => {
  isSidebarOpen.value = false
}
</script>

<style scoped>
/* Hapus semua style scoped karena sudah menggunakan Tailwind */
</style>