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
    padding: 1px 8px;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-pill);
    font-size: var(--font-size-sm);
}

.owned {
    background: var(--color-accent-soft);
    border-color: var(--color-accent);
}

.wanted {
    border-style: dashed;
    color: var(--color-text-muted);
}
</style>