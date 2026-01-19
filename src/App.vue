<template>
  <div class="flex h-screen">
    <Sidebar />
    <main class="flex-1 overflow-auto p-6">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import Sidebar from './components/Sidebar.vue'
</script>
