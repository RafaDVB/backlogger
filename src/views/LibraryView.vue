<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import GameFormDialog from "../components/dialogs/GameFormDialog.vue";
import GameGrid from "../components/grid/GameGrid.vue";
import { isLeaving, useDragDrop } from "../composables/useDragDrop";
import { useLibraryStore } from "../stores/library";
import { useShelfStore } from "../stores/shelf";
import { useNavStore } from "../stores/navigation";
import type { Status } from "../types/game";

const library = useLibraryStore();
const shelf = useShelfStore();
const nav = useNavStore();
const { dragging, endDrag } = useDragDrop();

const tab = ref<Status>("owned");
const shown = computed(() => (tab.value === "owned" ? library.owned : library.interested));

onMounted(() => library.load());

const adding = ref(false);
const openAdd = () => (adding.value = true);

const removeZoneActive = ref(false);
const acceptsRemoval = computed(() => dragging.value?.source === "shelf");

function onDragOver(event: DragEvent) {
  if (!acceptsRemoval.value) return;
  event.preventDefault();
  removeZoneActive.value = true;
}

function onDragLeave(event: DragEvent) {
  if (isLeaving(event)) removeZoneActive.value = false;
}

function onDrop(event: DragEvent) {
  if (!dragging.value || !acceptsRemoval.value) return;
  event.preventDefault();
  removeZoneActive.value = false;
  const gameId = dragging.value.gameId;
  endDrag();
  shelf.requestRemoval(gameId);
}
</script>

<template>
  <section :class="{ 'remove-zone': removeZoneActive }" @dragover="onDragOver" @dragleave="onDragLeave" @drop="onDrop">
    <div class="top">
      <nav class="tabs">
        <button :class="{ active: tab === 'owned' }" @click="tab = 'owned'">
          Owned ({{ library.owned.length }})
        </button>
        <button :class="{ active: tab === 'interested' }" @click="tab = 'interested'">
          Interested ({{ library.interested.length }})
        </button>
      </nav>
      <div class="actions">
        <button class="add" @click="openAdd">+ Add game</button>
        <button class="add" @click="nav.openSettings()">Settings</button>
      </div>
    </div>

    <p v-if="library.error" class="error">{{ library.error }}</p>
    <p v-if="removeZoneActive" class="zone-hint">Drop here to take this game off the shelf</p>

    <div v-if="!library.loading && library.games.length === 0" class="empty">
      <p>No games yet.</p>
      <button @click="openAdd">Add your first game</button>
    </div>
    <p v-else-if="shown.length === 0" class="empty">Nothing in this tab yet.</p>

    <GameGrid v-else :games="shown" @select="nav.open($event)" @mark-owned="library.markOwned($event)" />

    <GameFormDialog v-if="adding" :game-id="null" :default-status="tab" @close="adding = false" />
  </section>
</template>

<style scoped>
section {
  min-height: 100%;
  border: 2px dashed transparent;
  border-radius: 8px;
}

section.remove-zone {
  border-color: seagreen;
}

.zone-hint {
  color: seagreen;
}

.top {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.tabs {
  display: flex;
  gap: 8px;
}

.tabs button,
.add,
.empty button {
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

.actions {
  display: flex;
  gap: 8px;
}

.error {
  color: crimson;
}
</style>