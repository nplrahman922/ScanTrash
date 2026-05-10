<script setup>
import Header from '../components/Header.vue'
import Footer from '../components/Footer.vue'
</script>

<template>
  <div class="min-h-screen flex flex-col">
    <Header />

    <main class="content-area bg-[#FFFFFF] grow">
      <router-view />
    </main>

    <Footer class="pb-[env(safe-area-inset-bottom)]"></Footer>
  </div>
</template>