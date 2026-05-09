<template>
  <div class="p-4 bg-gray-100 min-h-screen">

    <!-- 🔝 TITLE -->
    <h1 class="text-lg font-semibold mb-4">Jenis Sampah</h1>

    <!-- ⏳ LOADING SKELETON -->
    <div v-if="trashStore.loading" class="space-y-4">
      <div
        v-for="n in 4"
        :key="n"
        class="animate-pulse bg-gray-300 rounded-2xl h-20 w-full"
      />
    </div>

    <!-- ❌ ERROR STATE -->
    <div
      v-else-if="trashStore.error"
      class="bg-red-100 text-red-600 text-sm px-4 py-3 rounded-xl"
    >
      ⚠️ Gagal memuat data: {{ trashStore.error }}
    </div>

    <!-- 📭 EMPTY STATE -->
    <div
      v-else-if="trashStore.trashTypes.length === 0"
      class="text-center text-gray-400 text-sm py-12"
    >
      Belum ada data jenis sampah.
    </div>

    <!-- 📋 LIST -->
    <div v-else class="space-y-4">
      <TrashTypeCard
        v-for="item in trashStore.trashTypes"
        :key="item.id"
        :name="item.name"
        :price="item.price"
        :image="item.image"
      />
    </div>

  </div>
</template>

<script setup lang="ts">
import { onMounted } from "vue"
import { useTrashStore } from "../stores/trashStore"
import TrashTypeCard from "../components/TrashTypeCard.vue"

const trashStore = useTrashStore()

// 🚀 Fetch data saat halaman dibuka
onMounted(() => {
  trashStore.fetchTrashTypes()
})
</script>

<style scoped>
div {
  border-radius: 20px;
}
</style>