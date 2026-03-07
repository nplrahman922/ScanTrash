<template>
  <div class="p-4">
    <h1 class="text-xl font-bold mb-4">Test Fetch Pricelist</h1>
    
    <button 
      @click="fetchData" 
      class="bg-blue-500 text-white px-4 py-2 rounded"
    >
      Ambil Data Harga
    </button>

    <div v-if="loading" class="mt-4">Loading...</div>
    <div v-else-if="error" class="mt-4 text-red-500">Error: {{ error }}</div>
    
    <ul v-else-if="pricelist.length > 0" class="mt-4">
      <li v-for="item in pricelist" :key="item.id" class="border-b py-2">
        {{ item.labels }} - Rp {{ item.price }}
      </li>
    </ul>
    <div v-else class="mt-4 text-gray-500">Data kosong atau belum di-fetch.</div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const pricelist = ref([]);
const loading = ref(false);
const error = ref(null);

async function fetchData() {
  loading.value = true;
  error.value = null;
  
  try {
    // Memanggil Rust tanpa embel-embel token lagi!
    const result = await invoke('get_pricelist_command');
    
    pricelist.value = result;
    console.log("Data berhasil di-fetch:", result);
  } catch (err) {
    error.value = err;
    console.error("Gagal fetch data:", err);
  } finally {
    loading.value = false;
  }
}
</script>