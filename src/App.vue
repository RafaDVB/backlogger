<script setup lang="ts">
import ShelfSidebar from "./components/shelf/ShelfSidebar.vue";
import BackgroundCanvas from "./components/ui/BackgroundCanvas.vue";
import { useNavStore } from "./stores/navigation";
import GameDetailView from "./views/GameDetailView.vue";
import LibraryView from "./views/LibraryView.vue";
import SettingsView from "./views/SettingsView.vue";

const nav = useNavStore();
</script>

<template>
  <div class="app">
    <BackgroundCanvas />
    <ShelfSidebar />
    <main class="main">
      <LibraryView v-show="nav.openGameId === null && !nav.settingsOpen" />
      <GameDetailView v-if="nav.openGameId !== null" :key="nav.openGameId" :game-id="nav.openGameId" />
      <SettingsView v-if="nav.settingsOpen" />
    </main>
  </div>
</template>

<style>
.app {
  display: grid;
  grid-template-columns: 340px 1fr;
  height: 100vh;
}

.shelf {
  padding: var(--space-4);
  border-right: 1px solid var(--color-border);
  background: var(--color-surface);
  overflow-y: auto;
}

.main {
  padding: var(--space-4);
  overflow-y: auto;
}
</style>