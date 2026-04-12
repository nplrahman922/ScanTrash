<script setup>
import Header from '../components/Header.vue'
import Footer from '../components/Footer.vue'
</script>

<template>
  <div class="min-h-screen flex flex-col">
    <Header class="shrink-0 z-50 shadow-sm" />

    <main class="content-area bg-[#FFFFFF] grow">
      <router-view />
    </main>

    <Footer class="shrink-0 z-50 shadow-[0_-4px_6px_-1px_rgba(0,0,0,0.05)]" />
  </div>
</template>