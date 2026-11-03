<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import GameFormDialog from "../components/dialogs/GameFormDialog.vue";
import MarkOwnedDialog from "../components/dialogs/MarkOwnedDialog.vue";
import GameGrid from "../components/grid/GameGrid.vue";
import FilterBar from "../components/library/FilterBar.vue";
import BaseButton from "../components/ui/BaseButton.vue";
import TabBar from "../components/ui/TabBar.vue";
import { isLeaving, useDragDrop } from "../composables/useDragDrop";
import { LIBRARY_SIZE } from "../lib/appearance";
import { applyFilters, hasActiveFilters, sortGames } from "../lib/gameFilters";
import type { Tab } from "../lib/viewState";
import { useAppearanceStore } from "../stores/appearance";
import { useLibraryStore } from "../stores/library";
import { useListsStore } from "../stores/lists";
import { useNavStore } from "../stores/navigation";
import { useShelfStore } from "../stores/shelf";
import { useViewStore } from "../stores/view";
import type { GameSummary } from "../types/game";

const appearance = useAppearanceStore();
const library = useLibraryStore();
const lists = useListsStore();
const nav = useNavStore();
const shelf = useShelfStore();
const view = useViewStore();
const { dragging, endDrag } = useDragDrop();

// ---- what is shown: filter first, then pick the tab, then sort ----

const filtered = computed(() => applyFilters(library.games, view.filters));

const counts = computed(() => ({
  all: filtered.value.length,
  owned: filtered.value.filter((g) => g.status === "owned").length,
  interested: filtered.value.filter((g) => g.status === "interested").length,
}));

const tabs = computed<{ id: Tab; label: string }[]>(() => [
  { id: "all", label: `All (${counts.value.all})` },
  { id: "owned", label: `Owned (${counts.value.owned})` },
  { id: "interested", label: `Interested (${counts.value.interested})` },
]);

const shown = computed(() => {
  const inTab =
    view.tab === "all" ? filtered.value : filtered.value.filter((g) => g.status === view.tab);
  return sortGames(inTab, view.sort);
});

// ---- loading, and keeping the saved filters honest ----

const listsReady = ref(false);

function pruneFilters() {
  view.prune({
    genres: lists.genres.map((g) => g.id),
    platforms: lists.platforms.map((p) => p.id),
    ratings: lists.ratings.map((r) => r.id),
  });
}

onMounted(async () => {
  await Promise.all([library.load(), lists.loadAll()]);
  pruneFilters();
  listsReady.value = true;
});

watch(
  () => [lists.genres, lists.platforms, lists.ratings],
  () => {
    if (listsReady.value) pruneFilters();
  },
);

// ---- adding a game, and marking one as owned ----

const adding = ref(false);
const openAdd = () => (adding.value = true);

const markOwnedTarget = ref<GameSummary | null>(null);

function askMarkOwned(id: number) {
  const game = library.games.find((g) => g.id === id);
  if (!game) return;
  library.error = null;
  markOwnedTarget.value = game;
}

async function confirmMarkOwned(platformId: number) {
  const game = markOwnedTarget.value;
  if (!game) return;
  if (await library.markOwned(game.id, platformId)) markOwnedTarget.value = null;
}

// ---- dropping a shelf game anywhere in the library asks to take it off the shelf ----

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
      <TabBar v-model="view.tab" :tabs="tabs" />
      <div class="actions">
        <label class="size">
          <span class="hint">Size</span>
          <input v-model.number="appearance.librarySize" type="range" :min="LIBRARY_SIZE.min" :max="LIBRARY_SIZE.max"
            step="10" aria-label="Tile size" />
        </label>
        <BaseButton variant="primary" @click="openAdd">+ Add game</BaseButton>
        <BaseButton @click="nav.openSettings()">Settings</BaseButton>
      </div>
    </div>

    <FilterBar />

    <p v-if="library.error && !markOwnedTarget" class="error">{{ library.error }}</p>
    <p v-if="removeZoneActive" class="zone-hint">Drop here to take this game off the shelf</p>
    <p v-if="hasActiveFilters(view.filters) && library.games.length > 0" class="status">
      Showing {{ filtered.length }} of {{ library.games.length }} games
    </p>

    <div v-if="!library.loading && library.games.length === 0" class="empty">
      <p>No games yet.</p>
      <BaseButton variant="primary" @click="openAdd">Add your first game</BaseButton>
    </div>
    <div v-else-if="shown.length === 0" class="empty">
      <p v-if="hasActiveFilters(view.filters)">No games match these filters.</p>
      <p v-else>Nothing in this tab yet.</p>
      <BaseButton v-if="hasActiveFilters(view.filters)" @click="view.clearFilters()">
        Clear filters
      </BaseButton>
    </div>

    <GameGrid v-else :games="shown" :tile="appearance.librarySize" @select="nav.open($event)"
      @mark-owned="askMarkOwned($event)" />

    <GameFormDialog v-if="adding" :game-id="null" :default-status="view.tab === 'owned' ? 'owned' : 'interested'"
      @close="adding = false" />

    <MarkOwnedDialog v-if="markOwnedTarget" :title="markOwnedTarget.title" :linked="markOwnedTarget.platforms"
      :error="library.error" @confirm="confirmMarkOwned" @cancel="markOwnedTarget = null" />
  </section>
</template>

<style scoped>
section {
  min-height: 100%;
  border: 1px solid transparent;
  border-radius: var(--radius-lg);
}

section.remove-zone {
  border-color: var(--edge-3);
  background: var(--gradient-focus);
}

.size {
  display: flex;
  flex: none;
  align-items: center;
  gap: var(--space-2);
}

.zone-hint {
  color: var(--color-text);
}

.status {
  display: inline-block;
  margin: 0 0 var(--space-3);
  padding: 0 var(--space-2);
  border: 1px solid var(--edge-1);
  background: var(--fill-1);
  color: var(--color-text-muted);
}

.top {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-4);
}

.actions {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}

.empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-5) var(--space-4);
  border: 1px solid var(--edge-1);
  background: var(--fill-1);
  color: var(--color-text-muted);
  text-align: center;
}

.empty p {
  margin: 0;
}
</style>