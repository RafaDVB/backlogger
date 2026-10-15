<script setup lang="ts">
import { computed } from "vue";
import { useDragDrop } from "../../composables/useDragDrop";
import { useHoverCycle } from "../../composables/useHoverCycle";
import { resolveImage } from "../../lib/images";
import type { GameSummary } from "../../types/game";

const props = defineProps<{ game: GameSummary }>();
const emit = defineEmits<{ select: [id: number]; markOwned: [id: number] }>();

const { startDrag, endDrag } = useDragDrop();
const canDrag = computed(() => props.game.shelfSlot === null);

function onDragStart(event: DragEvent) {
  startDrag(event, {
    source: "grid",
    gameId: props.game.id,
    status: props.game.status,
    fromSlot: null,
  });
}

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
  <article
    class="tile"
    :class="{ draggable: canDrag }"
    :draggable="canDrag"
    tabindex="0"
    role="button"
    @mouseenter="start"
    @mouseleave="stop"
    @click="emit('select', game.id)"
    @keydown.enter="emit('select', game.id)"
    @dragstart="onDragStart"
    @dragend="endDrag"
  >
    <div class="frame">
      <img v-if="current" :src="current" :alt="game.title" draggable="false" />
      <div v-else class="no-image">No image</div>
      <span v-if="game.shelfSlot" class="badge">On shelf</span>
      <span v-else-if="game.outcome" class="badge">{{ game.outcome }}</span>
      <button
        v-if="game.status === 'interested'"
        class="own"
        type="button"
        @click.stop="emit('markOwned', game.id)"
      >
        Mark as owned
      </button>
    </div>
    <div class="label">{{ label }}</div>
  </article>
</template>

<style scoped>
.tile {
  cursor: pointer;
}
.tile.draggable {
  cursor: grab;
}
.own {
  position: absolute;
  right: 8px;
  bottom: 8px;
  padding: 4px 10px;
  border: none;
  border-radius: 6px;
  background: rgba(0, 0, 0, 0.75);
  color: white;
  font-size: 12px;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.15s;
}
.tile:hover .own,
.own:focus-visible {
  opacity: 1;
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