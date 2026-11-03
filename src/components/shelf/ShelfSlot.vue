<script setup lang="ts">
import { computed, ref } from "vue";
import { isLeaving, useDragDrop } from "../../composables/useDragDrop";
import { resolveImage } from "../../lib/images";
import { useShelfStore } from "../../stores/shelf";
import { useNavStore } from "../../stores/navigation";
import type { ShelfEntry } from "../../types/shelf";

const props = defineProps<{ entry: ShelfEntry }>();
const shelf = useShelfStore();
const nav = useNavStore();
const { dragging, startDrag, endDrag } = useDragDrop();

const over = ref(false);
const cover = computed(() => resolveImage(props.entry.game?.coverPath ?? null));

const refusal = computed<string | null>(() => {
  const drag = dragging.value;
  if (!drag) return null;
  if (drag.source === "grid") {
    if (drag.status !== "owned") return "Mark as owned first";
    if (props.entry.game) return "Slot taken";
    return null;
  }
  return drag.fromSlot === props.entry.slot ? "Already here" : null;
});
const canDrop = computed(() => dragging.value !== null && refusal.value === null);

function onDragStart(event: DragEvent) {
  if (!props.entry.game) return;
  startDrag(event, {
    source: "shelf",
    gameId: props.entry.game.id,
    status: "owned",
    fromSlot: props.entry.slot,
  });
}

function onDragOver(event: DragEvent) {
  if (!dragging.value) return;
  over.value = true;
  if (canDrop.value) {
    event.preventDefault();
    if (event.dataTransfer) event.dataTransfer.dropEffect = "move";
  }
}

function onDragLeave(event: DragEvent) {
  if (isLeaving(event)) over.value = false;
}

async function onDrop(event: DragEvent) {
  event.preventDefault();
  over.value = false;
  const drag = dragging.value;
  if (!drag || !canDrop.value) return;
  endDrag();
  if (drag.source === "grid") {
    await shelf.place(drag.gameId, props.entry.slot);
  } else if (drag.fromSlot !== null) {
    await shelf.move(drag.fromSlot, props.entry.slot);
  }
}
</script>

<template>
  <div class="slot" :class="{
    full: entry.game,
    available: canDrop && !over,
    accept: over && canDrop,
    refuse: over && refusal,
  }" @dragover="onDragOver" @dragleave="onDragLeave" @drop="onDrop">
    <span class="n">{{ entry.slot }}</span>
    <div v-if="entry.game" class="game" draggable="true" @click="nav.open(entry.game.id)" @dragstart="onDragStart"
      @dragend="endDrag">
      <img v-if="cover" :src="cover" :alt="entry.game.title" draggable="false" />
      <div v-else class="no-cover" />
      <span class="title">{{ entry.game.title }}</span>
      <button class="remove" type="button" title="Take off the shelf" aria-label="Take off the shelf"
        @click.stop="shelf.requestRemoval(entry.game.id)">
        ×
      </button>
    </div>
    <div v-else class="empty">Empty</div>
    <div v-if="over && refusal" class="hint">{{ refusal }}</div>
  </div>
</template>

<style scoped>
.slot {
  position: relative;
  margin-bottom: -1px;
  border: 1px solid var(--edge-1);
  background: var(--fill-1);
}

.slot.full {
  border-color: var(--edge-2);
  background: var(--fill-2);
}

.slot.full:hover:not(.accept):not(.refuse) {
  z-index: 1;
  border-color: var(--edge-3);
  background: var(--fill-3);
}

.slot.available {
  z-index: 1;
  border-color: var(--edge-2);
}

.slot.accept {
  z-index: 1;
  border-color: var(--edge-3);
  background: var(--gradient-focus);
}

.slot.refuse {
  z-index: 1;
  border-color: var(--color-danger);
  background: color-mix(in srgb, var(--color-danger) 25%, transparent);
}

.n {
  position: absolute;
  top: 8px;
  left: 8px;
  z-index: 1;
  display: grid;
  align-items: center;
  justify-items: start;
  width: 22px;
  height: 22px;
  padding-left: round(down, calc((20px - 1ch) / 2), 1px);
  border: 1px solid var(--edge-3);
  background: var(--fill-3);
  pointer-events: none;
}

.slot.full .n {
  background: var(--color-on-image-bg);
  color: var(--color-on-image);
}

.game {
  display: flex;
  align-items: stretch;
  gap: var(--space-3);
  cursor: grab;
}

.game img,
.no-cover {
  display: block;
  flex: none;
  width: 96px;
  aspect-ratio: 3 / 4;
  object-fit: cover;
  border-right: 1px solid var(--edge-2);
  background: var(--fill-1);
}

.no-cover {
  background: repeating-linear-gradient(45deg, transparent 0 6px, var(--fill-1) 6px 12px);
}

.title {
  flex: 1;
  min-width: 0;
  padding: var(--space-2) var(--space-5) var(--space-2) 0;
  overflow-wrap: anywhere;
}

.remove {
  position: absolute;
  top: 8px;
  right: 8px;
  display: grid;
  place-items: center;
  width: 23px;
  height: 22px;
  padding: 0;
  font: inherit;
  color: var(--color-text);
  background: var(--fill-1);
  border: 1px solid var(--edge-1);
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.remove:hover,
.remove:focus-visible {
  background: var(--gradient-focus);
  border-color: var(--edge-3);
}

.empty {
  display: flex;
  align-items: center;
  height: 64px;
  padding-left: 34px;
  color: var(--color-text-muted);
}

.hint {
  position: absolute;
  inset: 0;
  display: grid;
  place-items: center;
  padding-bottom: 1px;
  background: var(--color-on-image-bg);
  color: var(--color-on-image);
  pointer-events: none;
}
</style>