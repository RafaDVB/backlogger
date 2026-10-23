<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import GameFormDialog from "../components/dialogs/GameFormDialog.vue";
import GameGrid from "../components/grid/GameGrid.vue";
import BaseButton from "../components/ui/BaseButton.vue";
import TabBar from "../components/ui/TabBar.vue";
import { isLeaving, useDragDrop } from "../composables/useDragDrop";
import { useLibraryStore } from "../stores/library";
import { useNavStore } from "../stores/navigation";
import { useShelfStore } from "../stores/shelf";
import type { Status } from "../types/game";

const library = useLibraryStore();
const shelf = useShelfStore();
const nav = useNavStore();
const { dragging, endDrag } = useDragDrop();

const tab = ref<Status>("owned");
const shown = computed(() => (tab.value === "owned" ? library.owned : library.interested));
const tabs = computed<{ id: Status; label: string }[]>(() => [
  { id: "owned", label: `Owned (${library.owned.length})` },
  { id: "interested", label: `Interested (${library.interested.length})` },
]);

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
      <TabBar v-model="tab" :tabs="tabs" />
      <div class="actions">
        <BaseButton variant="primary" @click="openAdd">+ Add game</BaseButton>
        <BaseButton @click="nav.openSettings()">Settings</BaseButton>
      </div>
    </div>

    <p v-if="library.error" class="error">{{ library.error }}</p>
    <p v-if="removeZoneActive" class="zone-hint">Drop here to take this game off the shelf</p>

    <div v-if="!library.loading && library.games.length === 0" class="empty">
      <p>No games yet.</p>
      <BaseButton variant="primary" @click="openAdd">Add your first game</BaseButton>
    </div>
    <p v-else-if="shown.length === 0" class="hint">Nothing in this tab yet.</p>

    <GameGrid v-else :games="shown" @select="nav.open($event)" @mark-owned="library.markOwned($event)" />

    <GameFormDialog v-if="adding" :game-id="null" :default-status="tab" @close="adding = false" />
  </section>
</template>

<style scoped>
section {
  min-height: 100%;
  border: 2px dashed transparent;
  border-radius: var(--radius-lg);
}

section.remove-zone {
  border-color: var(--color-accent);
}

.zone-hint {
  color: var(--color-accent);
}

.top {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-4);
}

.actions {
  display: flex;
  gap: var(--space-2);
}

.empty {
  color: var(--color-text-muted);
}
</style>