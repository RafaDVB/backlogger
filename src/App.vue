<script setup lang="ts">
import { useDebugStore } from "./stores/debug";

const debug = useDebugStore();
</script>

<template>
  <main class="container">
    <h1>Backlogger</h1>
    <button @click="debug.ping('hello from Vue')">Ping Rust</button>
    <p>{{ debug.reply }}</p>
  </main>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
}
</style>
