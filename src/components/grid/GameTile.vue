<script setup lang="ts">
import { computed } from "vue";
import { useHoverCycle } from "../../composables/useHoverCycle";
import { resolveImage } from "../../lib/images";
import type { GameSummary } from "../../types/game";

const props = defineProps<{ game: GameSummary }>();
const emit = defineEmits<{ select: [id: number] }>();

// Featured screenshot first, then the others; the cover only when there are no screenshots.
const images = computed(() => {
  const { featuredPath, screenshotPaths, coverPath } = props.game;
  const paths = featuredPath
    ? [featuredPath, ...screenshotPaths.filter((p) => p !== featuredPath)]
    : coverPath
      ? [coverPath]
      : [];
  return paths.map(resolveImage).filter((src): src is string => src !== null);
});

const { current, start, stop } = useHoverCycle(images);

const label = computed(() =>
  props.game.releaseYear ? `${props.game.title} (${props.game.releaseYear})` : props.game.title,
);
</script>

<template>
  <button
    class="tile"
    type="button"
    @mouseenter="start"
    @mouseleave="stop"
    @click="emit('select', game.id)"
  >
    <div class="frame">
      <img v-if="current" :src="current" :alt="game.title" draggable="false" />
      <div v-else class="no-image">No image</div>
      <span v-if="game.shelfSlot" class="badge">On shelf</span>
      <span v-else-if="game.outcome" class="badge">{{ game.outcome }}</span>
    </div>
    <div class="label">{{ label }}</div>
  </button>
</template>

<style scoped>
.tile {
  display: block;
  padding: 0;
  border: none;
  background: none;
  color: inherit;
  font: inherit;
  text-align: left;
  cursor: pointer;
}
.frame {
  position: relative;
  aspect-ratio: 16 / 9;
  overflow: hidden;
  border-radius: 8px;
  background: rgba(128, 128, 128, 0.2);
}
.frame img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}
.no-image {
  display: grid;
  place-items: center;
  height: 100%;
  opacity: 0.6;
}
.badge {
  position: absolute;
  top: 8px;
  left: 8px;
  padding: 2px 8px;
  border-radius: 999px;
  background: rgba(0, 0, 0, 0.7);
  color: white;
  font-size: 12px;
  text-transform: capitalize;
}
.label {
  margin-top: 6px;
  font-size: 14px;
}
</style>