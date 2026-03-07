<script setup lang="ts">
import Header from './components/Header.vue'
import BottomNav from './components/BottomNav.vue'
</script>

<template>
  <div class="min-h-screen flex flex-col">
    <Header />

    <main class="content-area bg-[#FFFFFF] grow">
      <router-view />
    </main>

    <BottomNav />
  </div>
</template>