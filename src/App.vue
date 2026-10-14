<script setup lang="ts">
import { useDebugStore } from "./stores/debug";

const debug = useDebugStore();
</script>

<template>
  <main class="container">
    <h1>Backlogger</h1>
    <button @click="debug.ping('hello from Vue')">Ping Rust</button>
    <p>{{ debug.reply }}</p>

    <button @click="debug.seedAndLoad()">Seed and load games</button>
    <p v-if="debug.error">{{ debug.error }}</p>
    <ul>
      <li v-for="game in debug.games" :key="game.id">
        {{ game.title }} ({{ game.releaseYear }}) · {{ game.status }}
        <span v-if="game.shelfSlot"> · slot {{ game.shelfSlot }}</span>
      </li>
    </ul>
  </main>
</template>

<style scoped>
</style>
