<template>
  <div class="border-t border-blue-500 my-1"></div>
  <footer class="flex items-center justify-center px-4 py-2 bg-white shadow-md fixed bottom-0 left-0 right-0 z-10">
    <div class="flex flex-col items-center text-center py-2 px-4">
      <img class="w-10 h-10 mb-1" :src="currentMenu.icon" />
    </div>
  </footer>
</template>

<script lang="ts">
import BerandaIcon from '../assets/Beranda.svg';
import NasabahIcon from '../assets/admin/Nasabah.svg';
import KatalogIcon from '../assets/admin/Katalog Harga.svg';
import RiwayatIcon from '../assets/Riwayat.svg';
import JadwalIcon from '../assets/admin/Jadwal Setor.svg';
export default {
  name: 'Footer',
  computed: {
    currentMenu() {
      const path = this.$route.path;
      if (path === '/dashboard' || path === '/') {
        return { icon: BerandaIcon };
      } else if (path === '/nasabah') {
        return { icon: NasabahIcon };
      } else if (path === '/katalog-harga') {
        return { icon: KatalogIcon };
      } else if (path === '/riwayat') {
        return { icon: RiwayatIcon };
      } else if (path === '/jadwal-setor') {
        return { icon: JadwalIcon };
      } else {
        return { icon: BerandaIcon };
      }
    },
  },
};
</script>

<style scoped>
/* Tambahkan padding bottom ke body atau container utama untuk menghindari overlap dengan bottom nav fixed */
</style>
