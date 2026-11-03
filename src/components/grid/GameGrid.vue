<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from "vue";
import type { GameSummary } from "../../types/game";
import GameTile from "./GameTile.vue";

const props = defineProps<{ games: GameSummary[]; tile: number }>();
defineEmits<{ select: [id: number]; markOwned: [id: number] }>();

const GAP = 12;
const grid = ref<HTMLElement | null>(null);
const columns = ref(`repeat(auto-fill, minmax(min(${props.tile}px, 100%), 1fr))`);

let observer: ResizeObserver | null = null;

function layout() {
  const element = grid.value;
  if (!element) return;

  const width = Math.floor(element.getBoundingClientRect().width);
  if (width <= 0) return;

  const preferred = Math.max(1, props.tile);
  const count = Math.max(1, Math.floor((width + GAP) / (preferred + GAP)));
  const columnWidth = Math.max(1, Math.floor((width - (count - 1) * GAP) / count));
  columns.value = `repeat(${count}, ${columnWidth}px)`;
}

onMounted(() => {
  layout();
  if (typeof ResizeObserver !== "undefined") {
    observer = new ResizeObserver(layout);
    if (grid.value) observer.observe(grid.value);
  }
  window.addEventListener("resize", layout);
});

onBeforeUnmount(() => {
  observer?.disconnect();
  observer = null;
  window.removeEventListener("resize", layout);
});

watch(() => props.tile, layout);
</script>

<template>
  <div ref="grid" class="grid" :style="{ '--tile': `${tile}px`, gridTemplateColumns: columns }">
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
