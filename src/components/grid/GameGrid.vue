<script setup lang="ts">
import type { GameSummary } from "../../types/game";
import GameTile from "./GameTile.vue";

defineProps<{ games: GameSummary[] }>();
defineEmits<{ select: [id: number] }>();
</script>

<template>
  <div class="grid">
    <GameTile
      v-for="game in games"
      :key="game.id"
      :game="game"
      @select="$emit('select', $event)"
    />
  </div>
</template>

<style scoped>
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 16px;
}
</style>