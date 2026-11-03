<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from "vue";
import { createBackground, type Background } from "../../lib/backgroundAnimations";
import { useAppearanceStore } from "../../stores/appearance";

const appearance = useAppearanceStore();
const canvas = ref<HTMLCanvasElement | null>(null);

let background: Background | null = null;

function fit() {
    background?.resize(window.innerWidth, window.innerHeight);
}

function onVisibilityChange() {
    if (document.hidden) background?.pause();
    else background?.resume();
}

onMounted(() => {
    const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    background = createBackground(canvas.value!, { animate: !reduceMotion });
    fit();
    background.setMode(appearance.background);
    window.addEventListener("resize", fit);
    document.addEventListener("visibilitychange", onVisibilityChange);
});

watch(
    () => appearance.background,
    (mode) => background?.setMode(mode),
);

onBeforeUnmount(() => {
    window.removeEventListener("resize", fit);
    document.removeEventListener("visibilitychange", onVisibilityChange);
    background?.destroy();
    background = null;
});
</script>

<template>
    <canvas ref="canvas" class="background" aria-hidden="true"></canvas>
</template>

<style scoped>
.background {
    position: fixed;
    inset: 0;
    width: 100%;
    height: 100%;
    z-index: -1;
    pointer-events: none;
    image-rendering: pixelated;
    filter: blur(2px);
}
</style>