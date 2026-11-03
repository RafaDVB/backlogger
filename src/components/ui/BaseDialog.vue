<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref } from "vue";

const props = withDefaults(
    defineProps<{
        title?: string;
        width?: string;
        dismissible?: boolean;
    }>(),
    { width: "460px", dismissible: true },
);
const emit = defineEmits<{ close: [] }>();

const overlay = ref<HTMLElement | null>(null);
const panel = ref<HTMLElement | null>(null);
const panelLeft = ref("0px");
const panelTop = ref("0px");
const ready = ref(false);
let observer: ResizeObserver | null = null;

function center() {
    const host = overlay.value;
    const dialog = panel.value;
    if (!host || !dialog) return;

    const rect = dialog.getBoundingClientRect();
    panelLeft.value = `${Math.round((host.clientWidth - rect.width) / 2)}px`;
    panelTop.value = `${Math.round((host.clientHeight - rect.height) / 2)}px`;
    ready.value = true;
}

function onResize() {
    center();
}

function onKeydown(event: KeyboardEvent) {
    if (event.key === "Escape" && props.dismissible) emit("close");
}

onMounted(() => {
    window.addEventListener("keydown", onKeydown);
    window.addEventListener("resize", onResize);
    if (typeof ResizeObserver !== "undefined") {
        observer = new ResizeObserver(center);
        if (panel.value) observer.observe(panel.value);
    }
    void nextTick(center);
    if (document.fonts) void document.fonts.ready.then(center);
});

onBeforeUnmount(() => {
    window.removeEventListener("keydown", onKeydown);
    window.removeEventListener("resize", onResize);
    observer?.disconnect();
    observer = null;
});
</script>

<template>
    <div ref="overlay" class="overlay" @click.self="dismissible && emit('close')">
        <div ref="panel" class="panel" :class="{ ready }" role="dialog" aria-modal="true" :aria-label="title"
            :style="{ width: `min(${width}, 92vw)`, left: panelLeft, top: panelTop }">
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
    background: var(--color-overlay);
    z-index: 10;
}

.panel {
    position: absolute;
    max-height: 90vh;
    overflow-y: auto;
    padding: var(--space-5);
    border: 1px solid var(--edge-3);
    border-radius: var(--radius-lg);
    background: linear-gradient(180deg, var(--fill-3), var(--fill-2)), var(--wall-2);
    color: var(--color-text);
    visibility: hidden;
}

.panel.ready {
    visibility: visible;
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
