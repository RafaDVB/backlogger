<script setup lang="ts">
import { onMounted } from "vue";
import { useShelfStore } from "../../stores/shelf";
import RemoveFromShelfDialog from "../dialogs/RemoveFromShelfDialog.vue";
import ShelfSlot from "./ShelfSlot.vue";

const shelf = useShelfStore();
onMounted(() => shelf.load());
</script>

<template>
  <aside class="shelf">
    <h2>Now playing</h2>
    <p v-if="shelf.error" class="error">{{ shelf.error }}</p>
    <ShelfSlot v-for="entry in shelf.slots" :key="entry.slot" :entry="entry" />
    <RemoveFromShelfDialog v-if="shelf.pendingRemoval" :game="shelf.pendingRemoval" />
  </aside>
</template>

<style scoped>
.error {
  color: crimson;
  font-size: 14px;
}
</style>