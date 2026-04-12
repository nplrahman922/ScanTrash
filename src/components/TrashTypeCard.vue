<template>
  <div class="flex items-center justify-between p-4 bg-white rounded-2xl shadow-md border">

    <!-- 🖼️ IMAGE (OPSIONAL) -->
    <div class="w-16 h-16 flex items-center justify-center">
      <img 
        v-if="image" 
        :src="image" 
        class="w-12 h-12 object-contain"
      />
      <div v-else class="w-12 h-12 bg-gray-200 rounded-lg"></div>
    </div>

    <!-- 📄 TEXT -->
    <div class="flex-1 ml-4">
      <h3 class="font-semibold">{{ name }}</h3>
      <p class="text-sm text-gray-500">Harga Estimasi</p>
    </div>

    <!-- 💰 PRICE -->
    <div>
      <p class="text-green-500 font-semibold">
        Rp{{ price.toLocaleString("id-ID") }}/kg
      </p>
    </div>

  </div>
</template>

<script setup lang="ts">
defineProps({
  name: String,
  price: { type: Number, required: true },
  image: String // 🔥 optional
})
</script>