<script setup lang="ts">
import type { GameSummary } from "../../types/game";
import GameTile from "./GameTile.vue";

defineProps<{ games: GameSummary[] }>();
defineEmits<{ select: [id: number]; markOwned: [id: number] }>();
</script>

<template>
  <div class="grid">
    <GameTile v-for="game in games" :key="game.id" :game="game" @select="$emit('select', $event)"
      @mark-owned="$emit('markOwned', $event)" />
  </div>
</template>

<style scoped>
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(min(var(--tile, 240px), 100%), 1fr));
  gap: var(--space-3);
}
</style>