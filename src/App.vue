<script setup lang="ts">
import ShelfSidebar from "./components/shelf/ShelfSidebar.vue";
import { useNavStore } from "./stores/navigation";
import GameDetailView from "./views/GameDetailView.vue";
import LibraryView from "./views/LibraryView.vue";

const nav = useNavStore();
</script>

<template>
  <div class="app">
    <ShelfSidebar />
    <main class="main">
      <LibraryView v-show="nav.openGameId === null" />
      <GameDetailView
        v-if="nav.openGameId !== null"
        :key="nav.openGameId"
        :game-id="nav.openGameId"
      />
    </main>
  </div>
</template>

<style>
:root {
  color-scheme: light dark;
  font-family: system-ui, sans-serif;
}
body {
  margin: 0;
}
.app {
  display: grid;
  grid-template-columns: 240px 1fr;
  height: 100vh;
}
.shelf {
  padding: 16px;
  border-right: 1px solid rgba(128, 128, 128, 0.3);
  overflow-y: auto;
}
.main {
  padding: 16px;
  overflow-y: auto;
}
.hint {
  opacity: 0.6;
  font-size: 14px;
}
</style>