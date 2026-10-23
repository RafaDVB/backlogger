<script setup lang="ts" generic="T extends string">
import BaseChip from "./BaseChip.vue";

defineProps<{ tabs: { id: T; label: string }[]; modelValue: T }>();
const emit = defineEmits<{ "update:modelValue": [id: T] }>();
</script>

<template>
    <nav class="tabs" role="tablist">
        <BaseChip v-for="tab in tabs" :key="tab.id" role="tab" :active="tab.id === modelValue"
            :aria-selected="tab.id === modelValue" @click="emit('update:modelValue', tab.id)">
            {{ tab.label }}
        </BaseChip>
    </nav>
</template>

<style scoped>
.tabs {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
}
</style>