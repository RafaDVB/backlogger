<script setup lang="ts">
defineProps<{ active?: boolean }>();
</script>

<template>
    <button type="button" class="chip" :class="{ active }">
        <slot />
    </button>
</template>

<style scoped>
.chip {
    padding: var(--space-1) var(--space-3);
    font: inherit;
    color: var(--color-text);
    background: transparent;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    cursor: pointer;
}

.chip:hover:not(:disabled) {
    background: var(--color-hover);
}

.chip.active {
    background: var(--color-selected);
    border-color: var(--color-accent);
    font-weight: 600;
}

.chip:disabled {
    opacity: 0.45;
    cursor: default;
}
</style>