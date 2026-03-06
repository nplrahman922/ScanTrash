<template>
  <header class="group flex items-center justify-between px-4 py-2 bg-white shadow-md">
    <ActionMenu />
    <img class="logo" src="../assets/Logo2.svg" alt="Logo" />
    <img class="logo" src="../assets/Profile.svg" alt="Profile" />
  </header>
</template>

<script lang="ts">
import ActionMenu from './ActionMenu.vue';
export default {
  name: 'Header',
  components: {
    ActionMenu
  },
};
</script>