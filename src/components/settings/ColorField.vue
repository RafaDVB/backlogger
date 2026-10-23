<script setup lang="ts">
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
      <button type="button" title="No color" @click="emit('update:modelValue', null)">×</button>
    </template>
    <button v-else type="button" @click="emit('update:modelValue', '#4a90d9')">Add color</button>
  </span>
</template>

<style scoped>
.color {
  display: inline-flex;
  align-items: center;
  gap: 4px;
}
input[type="color"] {
  width: 34px;
  height: 26px;
  padding: 0;
  border: 1px solid rgba(128, 128, 128, 0.5);
  border-radius: 4px;
  background: none;
  cursor: pointer;
}
button {
  padding: 3px 10px;
  border: 1px solid rgba(128, 128, 128, 0.5);
  border-radius: 6px;
  background: none;
  color: inherit;
  font: inherit;
  cursor: pointer;
}
</style>