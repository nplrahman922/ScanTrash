<template>
  <div class="p-4 bg-gray-100 min-h-screen">

    <!-- 🔝 TITLE -->
    <h1 class="text-lg font-semibold mb-4">
      Tips Memilah Sampah
    </h1>

    <!-- 📋 LIST -->
    <div class="space-y-4">
      <TipsCard
        v-for="item in tipsStore.tips"
        :key="item.id"
        :title="item.title"
        :description="item.description"
        :icon="item.icon"
        :highlight="item.highlight"
      />
    </div>

  </div>
</template>

<script setup lang="ts">
import { useTipsStore } from "../../stores/tipsStore"
import TipsCard from "../components/TipsCard.vue"

const tipsStore = useTipsStore()
</script>

<style scoped>
div {
  border-radius: 20px;
}
</style>