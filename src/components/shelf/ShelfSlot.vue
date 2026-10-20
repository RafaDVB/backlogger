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
  <div class="slot" :class="{ accept: over && canDrop, refuse: over && refusal }" @dragover="onDragOver"
    @dragleave="onDragLeave" @drop="onDrop">
    <div v-if="entry.game" class="game" draggable="true" @click="nav.open(entry.game.id)" @dragstart="onDragStart"
      @dragend="endDrag">
      <img v-if="cover" :src="cover" :alt="entry.game.title" draggable="false" />
      <div v-else class="no-cover" />
      <span class="title">{{ entry.game.title }}</span>
      <button class="remove" type="button" title="Take off the shelf" @click.stop="shelf.requestRemoval(entry.game.id)">
        ×
      </button>
    </div>
    <div v-else class="empty">Slot {{ entry.slot }}</div>
    <div v-if="over && refusal" class="hint">{{ refusal }}</div>
  </div>
</template>

<style scoped>
.slot {
  position: relative;
  margin-bottom: 10px;
  min-height: 72px;
  border: 2px dashed rgba(128, 128, 128, 0.35);
  border-radius: 8px;
}

.slot.accept {
  border-color: seagreen;
}

.slot.refuse {
  border-color: crimson;
}

.game {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 6px;
  cursor: grab;
}

.game img,
.no-cover {
  width: 54px;
  height: 72px;
  object-fit: cover;
  border-radius: 4px;
  background: rgba(128, 128, 128, 0.25);
  flex-shrink: 0;
}

.title {
  flex: 1;
  font-size: 14px;
}

.remove {
  border: none;
  background: none;
  color: inherit;
  font-size: 20px;
  line-height: 1;
  cursor: pointer;
  opacity: 0.6;
}

.remove:hover {
  opacity: 1;
}

.empty {
  display: grid;
  place-items: center;
  height: 72px;
  opacity: 0.5;
  font-size: 14px;
}

.hint {
  position: absolute;
  inset: 0;
  display: grid;
  place-items: center;
  border-radius: 6px;
  background: rgba(0, 0, 0, 0.75);
  color: white;
  font-size: 14px;
  pointer-events: none;
}
</style>