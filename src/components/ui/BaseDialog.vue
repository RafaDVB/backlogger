<script setup lang="ts">
import { onBeforeUnmount, onMounted } from "vue";

const props = withDefaults(
    defineProps<{
        title?: string;
        width?: string;
        dismissible?: boolean;
    }>(),
    { width: "460px", dismissible: true },
);
const emit = defineEmits<{ close: [] }>();

function onKeydown(event: KeyboardEvent) {
    if (event.key === "Escape" && props.dismissible) emit("close");
}

onMounted(() => window.addEventListener("keydown", onKeydown));
onBeforeUnmount(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
    <div class="overlay" @click.self="dismissible && emit('close')">
        <div class="panel" role="dialog" aria-modal="true" :aria-label="title"
            :style="{ width: `min(${width}, 92vw)` }">
            <h3 v-if="title" class="title">{{ title }}</h3>
            <slot />
            <div v-if="$slots.footer" class="footer">
                <slot name="footer" />
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
    background: var(--color-overlay);
    z-index: 10;
}

.panel {
    max-height: 90vh;
    overflow-y: auto;
    padding: var(--space-5);
    border: 1px solid var(--edge-3);
    border-radius: var(--radius-lg);
    background: linear-gradient(180deg, var(--fill-3), var(--fill-2)), var(--wall-2);
    color: var(--color-text);
}

.title {
    margin: calc(-1 * var(--space-5)) calc(-1 * var(--space-5)) var(--space-4);
    padding: var(--space-1) var(--space-3);
    border-bottom: 1px solid var(--edge-2);
    background: var(--gradient-title);
    font-size: var(--font-size-md);
}

.footer {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-2);
    margin-top: var(--space-4);
}
</style>