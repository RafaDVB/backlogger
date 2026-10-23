<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useListsStore } from "../../stores/lists";
import { useShelfStore } from "../../stores/shelf";
import type { GameSummary } from "../../types/game";
import BaseButton from "../ui/BaseButton.vue";
import BaseChip from "../ui/BaseChip.vue";
import BaseDialog from "../ui/BaseDialog.vue";

const props = defineProps<{ game: GameSummary }>();

const shelf = useShelfStore();
const lists = useListsStore();
const step = ref<"ask" | "rate">("ask");

onMounted(() => lists.loadRatings());
</script>

<template>
  <BaseDialog
    :title="step === 'ask' ? `Take ${props.game.title} off the shelf` : `How would you rate ${props.game.title}?`"
    @close="shelf.cancelRemoval()">
    <template v-if="step === 'ask'">
      <p>Did you finish it?</p>
      <div class="buttons">
        <BaseButton variant="primary" @click="step = 'rate'">Yes, finished</BaseButton>
        <BaseButton @click="shelf.confirmRemoval({ kind: 'notFinished' })">Not finished</BaseButton>
        <BaseButton @click="shelf.confirmRemoval({ kind: 'dropped' })">I dropped it</BaseButton>
        <BaseButton variant="quiet" @click="shelf.cancelRemoval()">Cancel</BaseButton>
      </div>
    </template>

    <template v-else>
      <div class="buttons">
        <BaseChip v-for="rating in lists.ratings" :key="rating.id"
          :style="rating.color ? { borderLeft: '6px solid ' + rating.color } : undefined"
          @click="shelf.confirmRemoval({ kind: 'finished', ratingId: rating.id })">
          {{ rating.label }}
        </BaseChip>
      </div>
      <div class="buttons">
        <BaseButton @click="shelf.confirmRemoval({ kind: 'finished', ratingId: null })">
          Skip rating
        </BaseButton>
        <BaseButton variant="quiet" @click="step = 'ask'">Back</BaseButton>
      </div>
    </template>
  </BaseDialog>
</template>

<style scoped>
.buttons {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  margin-top: var(--space-3);
}
</style>