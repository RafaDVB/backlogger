<script setup lang="ts">
import LibraryView from "./views/LibraryView.vue";
</script>

<template>
  <div class="app">
    <aside class="shelf">
      <h2>Now playing</h2>
      <p class="hint">The six shelf slots arrive in slice 3.</p>
    </aside>
    <main class="main">
      <LibraryView />
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