<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import * as debugApi from "../api/debug.ts";
import GameGrid from "../components/grid/GameGrid.vue";
import { useLibraryStore } from "../stores/library.ts";
import type { Status } from "../types/game.ts";

const library = useLibraryStore();
const tab = ref<Status>("owned");
const shown = computed(() => (tab.value === "owned" ? library.owned : library.interested));

onMounted(() => library.load());

// Temporary: removed in slice 4 when you can add games for real.
async function loadSamples() {
  await debugApi.seedSampleData();
  await library.load();
}

function openGame(id: number) {
  console.log("open game", id); // The detail view arrives in slice 5.
}
</script>

<template>
  <section>
    <nav class="tabs">
      <button :class="{ active: tab === 'owned' }" @click="tab = 'owned'">
        Owned ({{ library.owned.length }})
      </button>
      <button :class="{ active: tab === 'interested' }" @click="tab = 'interested'">
        Interested ({{ library.interested.length }})
      </button>
    </nav>

    <p v-if="library.error" class="error">{{ library.error }}</p>

    <div v-if="!library.loading && library.games.length === 0" class="empty">
      <p>No games yet.</p>
      <button @click="loadSamples">Load sample data</button>
    </div>
    <p v-else-if="shown.length === 0" class="empty">Nothing in this tab yet.</p>

    <GameGrid v-else :games="shown" @select="openGame" />
  </section>
</template>

<style scoped>
.tabs {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
}
.tabs button {
  padding: 6px 14px;
  border: 1px solid rgba(128, 128, 128, 0.4);
  border-radius: 6px;
  background: none;
  color: inherit;
  font: inherit;
  cursor: pointer;
}
.tabs button.active {
  background: rgba(128, 128, 128, 0.25);
  font-weight: 600;
}
.empty {
  opacity: 0.7;
}
.error {
  color: crimson;
}
</style>