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
    padding: 2px var(--space-3);
    font: inherit;
    color: var(--color-text);
    background: transparent;
    border: 1px solid var(--edge-2);
    border-radius: var(--radius-md);
    cursor: pointer;
}

.chip:hover:not(:disabled),
.chip:focus-visible {
    background: var(--gradient-focus);
    border-color: var(--edge-3);
}

.chip.active {
    background: var(--fill-3);
    border-color: var(--edge-3);
}

.chip.active:hover:not(:disabled),
.chip.active:focus-visible {
    background: var(--gradient-focus), var(--fill-3);
}

.chip:disabled {
    opacity: 0.45;
    cursor: default;
}
</style>