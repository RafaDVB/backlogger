<script setup lang="ts">
import { computed } from "vue";
import type { DeletePreview, PreviewGame } from "../../types/game";

const props = defineProps<{
  kind: "genre" | "rating" | "platform";
  name: string;
  preview: DeletePreview;
  error: string | null;
}>();
const emit = defineEmits<{ confirm: []; cancel: [] }>();

const consequence = computed(
  () =>
    ({
      genre: "Deleting it removes the genre from these games.",
      rating: "Deleting it sets these games back to Unrated.",
      platform:
        "Deleting it removes the platform from these games. A game whose only owned platform this is goes back to Interested.",
    })[props.kind],
);

const blocked = computed(() => props.preview.blocked.length > 0);
const remaining = computed(() => props.preview.total - props.preview.first.length);

function describe(game: PreviewGame): string {
  const year = game.releaseYear ? ` (${game.releaseYear})` : "";
  const rated = props.kind === "rating" ? `, rated ${props.name}` : "";
  return `${game.title}${year}${rated}`;
}
</script>

<template>
  <div class="overlay" @click.self="emit('cancel')">
    <div class="panel" role="dialog" aria-modal="true">
      <h3>Delete {{ kind }} "{{ name }}"?</h3>

      <p v-if="preview.total === 0">No games use it.</p>
      <template v-else>
        <p>
          {{ preview.total }} {{ preview.total === 1 ? "game uses" : "games use" }} it:
        </p>
        <ul>
          <li v-for="game in preview.first" :key="game.id">{{ describe(game) }}</li>
          <li v-if="remaining > 0">…and {{ remaining }} more</li>
        </ul>
        <p>{{ consequence }}</p>
      </template>

      <p v-if="blocked" class="warning">
        It can't be deleted yet: {{ preview.blocked.join(", ") }}
        {{ preview.blocked.length === 1 ? "is" : "are" }} on the shelf or
        {{ preview.blocked.length === 1 ? "has" : "have" }} an outcome, and this is the only
        platform {{ preview.blocked.length === 1 ? "it is" : "they are" }} owned on. Take
        {{ preview.blocked.length === 1 ? "it" : "them" }} off the shelf and clear the outcome
        first, or own {{ preview.blocked.length === 1 ? "it" : "them" }} on another platform.
      </p>
      <p v-if="error" class="warning">{{ error }}</p>

      <div class="buttons">
        <button type="button" class="danger" :disabled="blocked" @click="emit('confirm')">
          Delete
        </button>
        <button type="button" @click="emit('cancel')">Cancel</button>
      </div>
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
  width: min(460px, 92vw);
  max-height: 90vh;
  overflow-y: auto;
  padding: 20px;
  border-radius: 10px;
  background: Canvas;
  color: CanvasText;
}
ul {
  margin: 0 0 12px;
  padding-left: 20px;
}
.warning {
  color: crimson;
}
.buttons {
  display: flex;
  gap: 8px;
  margin-top: 16px;
}
button {
  padding: 6px 14px;
  border: 1px solid rgba(128, 128, 128, 0.5);
  border-radius: 6px;
  background: none;
  color: inherit;
  font: inherit;
  cursor: pointer;
}
button:disabled {
  opacity: 0.4;
  cursor: default;
}
.danger {
  border-color: crimson;
  color: crimson;
}
</style>