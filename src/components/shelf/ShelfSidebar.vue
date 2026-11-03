<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useShelfStore } from "../../stores/shelf";
import RemoveFromShelfDialog from "../dialogs/RemoveFromShelfDialog.vue";
import ShelfSlot from "./ShelfSlot.vue";

const shelf = useShelfStore();
onMounted(() => shelf.load());

const filled = computed(() => shelf.slots.filter((entry) => entry.game).length);
</script>

<template>
  <aside class="shelf">
    <h2>
      <span>My Shelf</span>
      <span class="count">{{ filled }}/{{ shelf.slots.length }}</span>
    </h2>
    <p v-if="shelf.error" class="error">{{ shelf.error }}</p>
    <div class="slots">
      <ShelfSlot v-for="entry in shelf.slots" :key="entry.slot" :entry="entry" />
    </div>
    <RemoveFromShelfDialog v-if="shelf.pendingRemoval" :game="shelf.pendingRemoval" />
  </aside>
</template>

<style scoped>
h2 {
  display: flex;
  justify-content: space-between;
  margin: 0;
  padding: var(--space-2) var(--space-3);
  border-bottom: 1px solid var(--edge-1);
  font-size: var(--font-size-md);
}

.count {
  color: var(--color-text-muted);
}

.error {
  margin: 0;
  padding: var(--space-2) var(--space-3);
}

.slots {
  margin-top: var(--space-2);
  padding-bottom: 1px;
}
</style>