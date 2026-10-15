<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useListsStore } from "../../stores/lists";
import { useShelfStore } from "../../stores/shelf";
import type { GameSummary } from "../../types/game";

defineProps<{ game: GameSummary }>();

const shelf = useShelfStore();
const lists = useListsStore();
const step = ref<"ask" | "rate">("ask");

onMounted(() => lists.loadRatings());
</script>

<template>
  <div class="overlay" @click.self="shelf.cancelRemoval()">
    <div class="panel" role="dialog" aria-modal="true">
      <template v-if="step === 'ask'">
        <h3>Take {{ game.title }} off the shelf</h3>
        <p>Did you finish it?</p>
        <div class="buttons">
          <button @click="step = 'rate'">Yes, finished</button>
          <button @click="shelf.confirmRemoval({ kind: 'notFinished' })">Not finished</button>
          <button @click="shelf.confirmRemoval({ kind: 'dropped' })">I dropped it</button>
          <button class="quiet" @click="shelf.cancelRemoval()">Cancel</button>
        </div>
      </template>

      <template v-else>
        <h3>How would you rate {{ game.title }}?</h3>
        <div class="buttons ratings">
          <button
            v-for="rating in lists.ratings"
            :key="rating.id"
            @click="shelf.confirmRemoval({ kind: 'finished', ratingId: rating.id })"
          >
            {{ rating.label }}
          </button>
        </div>
        <div class="buttons">
          <button @click="shelf.confirmRemoval({ kind: 'finished', ratingId: null })">Skip rating</button>
          <button class="quiet" @click="step = 'ask'">Back</button>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  display: grid;
  place-items: center;
  background: rgba(0, 0, 0, 0.5);
  z-index: 10;
}
.panel {
  width: min(420px, 90vw);
  padding: 20px;
  border-radius: 10px;
  background: Canvas;
  color: CanvasText;
}
.buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 12px;
}
.buttons button {
  padding: 6px 14px;
  border: 1px solid rgba(128, 128, 128, 0.5);
  border-radius: 6px;
  background: none;
  color: inherit;
  font: inherit;
  cursor: pointer;
}
.buttons button:hover {
  background: rgba(128, 128, 128, 0.2);
}
.quiet {
  opacity: 0.7;
}
</style>