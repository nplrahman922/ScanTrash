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
        <a href="#" class="flex items-center py-4 px-5 text-gray-800 text-base hover:bg-gray-100 transition-colors">
          <img class="w-6 h-6 mr-3" src="../assets/Dashboard.svg" alt="Dashboard" />Dashboard
        </a>
        <a href="#" class="flex items-center py-4 px-5 text-gray-800 text-base hover:bg-gray-100 transition-colors">
          <img class="w-6 h-6 mr-3" src="../assets/Nasabah.svg" alt="Nasabah" />Nasabah
        </a>
        <a href="#" class="flex items-center py-4 px-5 text-gray-800 text-base hover:bg-gray-100 transition-colors">
          <img class="w-6 h-6 mr-3" src="../assets/Katalog Harga.svg" alt="Katalog Harga" />Katalog Harga
        </a>
        <a href="#" class="flex items-center py-4 px-5 text-gray-800 text-base hover:bg-gray-100 transition-colors">
          <img class="w-6 h-6 mr-3" src="../assets/Riwayat.svg" alt="Riwayat" />Riwayat
        </a>
        <a href="#" class="flex items-center py-4 px-5 text-gray-800 text-base hover:bg-gray-100 transition-colors">
          <img class="w-6 h-6 mr-3" src="../assets/Jadwal Setor.svg" alt="Jadwal Setor" />Jadwal Setor
        </a>

        <!-- Divider -->
        <div class="h-px bg-gray-300 my-5"></div>

        <!-- Logout Button -->
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