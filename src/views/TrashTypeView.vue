<template>
  <div class="p-4 bg-gray-100 min-h-screen">

    <!-- 🔝 TITLE -->
    <h1 class="text-lg font-semibold mb-4">Jenis Sampah</h1>

    <!-- 📋 LIST -->
    <div class="space-y-4">
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
import { useTrashStore } from "../stores/trashStore"
import TrashTypeCard from "../components/TrashTypeCard.vue"

const trashStore = useTrashStore()

// 🔥 nanti kalau pakai backend
// onMounted(() => {
//   trashStore.fetchTrashTypes()
// })
</script>

<style scoped>
div {
  border-radius: 20px;
}
</style>