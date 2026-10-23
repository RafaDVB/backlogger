<script setup lang="ts">
import BaseButton from "../ui/BaseButton.vue";

defineProps<{ modelValue: string | null }>();
const emit = defineEmits<{ "update:modelValue": [value: string | null] }>();

function onChange(event: Event) {
  emit("update:modelValue", (event.target as HTMLInputElement).value);
}
</script>

<template>
  <span class="color">
    <template v-if="modelValue">
      <input type="color" :value="modelValue" @change="onChange" />
      <BaseButton size="sm" variant="quiet" title="No color" @click="emit('update:modelValue', null)">
        ×
      </BaseButton>
    </template>
    <BaseButton v-else size="sm" @click="emit('update:modelValue', '#4a90d9')">Add color</BaseButton>
  </span>
</template>

<style scoped>
.color {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
}

input[type="color"] {
  width: 34px;
  height: 26px;
  padding: 0;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  background: none;
  cursor: pointer;
}
</style>