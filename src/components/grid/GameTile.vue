<script setup lang="ts">
import { computed } from "vue";
import { useDragDrop } from "../../composables/useDragDrop";
import { useHoverCycle } from "../../composables/useHoverCycle";
import { resolveImage } from "../../lib/images";
import type { GameSummary } from "../../types/game";
import PlatformTag from "../ui/PlatformTag.vue";

const props = defineProps<{ game: GameSummary }>();
const emit = defineEmits<{ select: [id: number]; markOwned: [id: number] }>();

const { startDrag, endDrag } = useDragDrop();
const canDrag = computed(() => props.game.shelfSlot === null);

const MAX_PLATFORMS = 3;
const shownPlatforms = computed(() => props.game.platforms.slice(0, MAX_PLATFORMS));
const hiddenCount = computed(() => props.game.platforms.length - MAX_PLATFORMS);

function onDragStart(event: DragEvent) {
  startDrag(event, {
    source: "grid",
    gameId: props.game.id,
    status: props.game.status,
    fromSlot: null,
  });
}

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
  <article class="tile" :class="{ draggable: canDrag }" :draggable="canDrag" tabindex="0" role="button"
    @mouseenter="start" @mouseleave="stop" @click="emit('select', game.id)" @keydown.enter="emit('select', game.id)"
    @dragstart="onDragStart" @dragend="endDrag">
    <div class="frame">
      <img v-if="current" :src="current" :alt="game.title" draggable="false" />
      <div v-else class="no-image">No image</div>
      <span v-if="game.shelfSlot" class="badge">On shelf</span>
      <span v-else-if="game.outcome" class="badge">{{ game.outcome }}</span>
      <button v-if="game.status === 'interested'" class="own" type="button" @click.stop="emit('markOwned', game.id)">
        Mark as owned
      </button>
    </div>
    <div class="label">{{ label }}</div>
    <div v-if="game.platforms.length" class="platforms">
      <PlatformTag v-for="platform in shownPlatforms" :key="platform.id" :name="platform.name"
        :state="platform.state" />
      <span v-if="hiddenCount > 0" class="more">+{{ hiddenCount }}</span>
    </div>
  </article>
</template>

<style scoped>
.tile {
  border: 1px solid var(--edge-1);
  border-radius: var(--radius-md);
  background: var(--fill-1);
  cursor: pointer;
}

.tile.draggable {
  cursor: grab;
}

.tile:hover,
.tile:focus-visible {
  border-color: var(--edge-3);
  background: var(--fill-2);
}

.tile:focus-visible {
  outline: 1px solid var(--edge-3);
  outline-offset: 2px;
}

.frame {
  position: relative;
  aspect-ratio: 16 / 9;
  overflow: hidden;
  border-bottom: 1px solid var(--edge-1);
  background: var(--fill-1);
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
  color: var(--color-text-muted);
}

.badge {
  position: absolute;
  top: var(--space-2);
  left: var(--space-2);
  padding: 0 var(--space-2);
  border: 1px solid var(--edge-2);
  background: var(--color-on-image-bg);
  color: var(--color-on-image);
  text-transform: capitalize;
}

.own {
  position: absolute;
  right: var(--space-2);
  bottom: var(--space-2);
  padding: 2px var(--space-2);
  font: inherit;
  border: 1px solid var(--edge-2);
  border-radius: var(--radius-sm);
  background: var(--color-on-image-bg);
  color: var(--color-on-image);
  cursor: pointer;
  opacity: 0;
  transition: opacity var(--transition);
}

.own:hover,
.own:focus-visible {
  border-color: var(--edge-3);
  background: var(--gradient-focus), var(--color-on-image-bg);
}

.tile:hover .own,
.own:focus-visible {
  opacity: 1;
}

.label {
  padding: var(--space-1) var(--space-2) 0;
}

.platforms {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--space-1);
  padding: var(--space-1) var(--space-2) var(--space-2);
}

.label:last-child {
  padding-bottom: var(--space-2);
}

.more {
  color: var(--color-text-muted);
}
</style>