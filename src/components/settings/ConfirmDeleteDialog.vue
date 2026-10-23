<script setup lang="ts">
import { computed } from "vue";
import type { DeletePreview, PreviewGame } from "../../types/game";
import BaseButton from "../ui/BaseButton.vue";
import BaseDialog from "../ui/BaseDialog.vue";

const props = defineProps<{
  kind: "genre" | "rating" | "platform";
  name: string;
  preview: DeletePreview;
  error: string | null;
}>();
const emit = defineEmits<{ confirm: []; cancel: [] }>();

const title = computed(() => `Delete ${props.kind} "${props.name}"?`);

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
  <BaseDialog :title="title" @close="emit('cancel')">
    <p v-if="preview.total === 0">No games use it.</p>
    <template v-else>
      <p>{{ preview.total }} {{ preview.total === 1 ? "game uses" : "games use" }} it:</p>
      <ul>
        <li v-for="game in preview.first" :key="game.id">{{ describe(game) }}</li>
        <li v-if="remaining > 0">…and {{ remaining }} more</li>
      </ul>
      <p>{{ consequence }}</p>
    </template>

    <p v-if="blocked" class="error">
      It can't be deleted yet: {{ preview.blocked.join(", ") }}
      {{ preview.blocked.length === 1 ? "is" : "are" }} on the shelf or
      {{ preview.blocked.length === 1 ? "has" : "have" }} an outcome, and this is the only
      platform {{ preview.blocked.length === 1 ? "it is" : "they are" }} owned on. Take
      {{ preview.blocked.length === 1 ? "it" : "them" }} off the shelf and clear the outcome
      first, or own {{ preview.blocked.length === 1 ? "it" : "them" }} on another platform.
    </p>
    <p v-if="error" class="error">{{ error }}</p>

    <template #footer>
      <BaseButton variant="danger" :disabled="blocked" @click="emit('confirm')">Delete</BaseButton>
      <BaseButton variant="quiet" @click="emit('cancel')">Cancel</BaseButton>
    </template>
  </BaseDialog>
</template>

<style scoped>
ul {
  margin: 0 0 var(--space-3);
  padding-left: var(--space-5);
}
</style>