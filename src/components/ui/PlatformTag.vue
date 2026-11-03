<script setup lang="ts">
import { computed } from "vue";
import type { PlatformState } from "../../types/game";

const props = defineProps<{ name: string; state: PlatformState; hint?: string }>();

const label = computed(() =>
    props.state === "owned" ? `Owned on ${props.name}` : `Wanted on ${props.name}`,
);
</script>

<template>
    <span class="tag" :class="state" :title="hint ?? label" :aria-label="label">{{ name }}</span>
</template>

<style scoped>
.tag {
    display: inline-block;
    padding: 0 var(--space-1);
    border: 1px solid var(--edge-1);
    border-radius: var(--radius-sm);
    background: var(--fill-1);
    color: var(--color-text-muted);
}

.owned {
    border-color: var(--edge-3);
    background: var(--fill-3);
    color: var(--color-text);
}
</style>