<script setup lang="ts">
import { backgrounds, themes } from "../../lib/appearance";
import { useAppearanceStore } from "../../stores/appearance";
import BaseChip from "../ui/BaseChip.vue";

const appearance = useAppearanceStore();
</script>

<template>
    <div>
        <h3>Palette</h3>
        <div class="palettes" role="radiogroup" aria-label="Palette">
            <button v-for="palette in themes" :key="palette.id" type="button" class="palette"
                :class="{ active: appearance.theme === palette.id }" role="radio"
                :aria-checked="appearance.theme === palette.id" @click="appearance.theme = palette.id">
                <span class="preview"
                    :style="{ background: `linear-gradient(135deg, ${palette.wall[0]}, ${palette.wall[1]})` }">
                    <i class="bar"
                        :style="{ background: `linear-gradient(90deg, ${palette.accent[0]}, ${palette.accent[1]})` }"></i>
                </span>
                {{ palette.label }}
            </button>
        </div>

        <h3>Background</h3>
        <div class="options" role="radiogroup" aria-label="Background">
            <BaseChip v-for="option in backgrounds" :key="option.id" role="radio"
                :active="appearance.background === option.id" :aria-checked="appearance.background === option.id"
                @click="appearance.background = option.id">
                {{ option.label }}
            </BaseChip>
        </div>

    </div>
</template>

<style scoped>
.options {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
}

.palettes {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-3);
}

.palette {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-2);
    font: inherit;
    color: var(--color-text);
    text-align: left;
    background: transparent;
    border: 1px solid var(--edge-2);
    border-radius: var(--radius-md);
    cursor: pointer;
}

.palette:hover,
.palette:focus-visible {
    background: var(--gradient-focus);
    border-color: var(--edge-3);
}

.palette.active {
    background: var(--fill-3);
    border-color: var(--edge-3);
}

.palette.active:hover,
.palette.active:focus-visible {
    background: var(--gradient-focus), var(--fill-3);
}

.preview {
    position: relative;
    display: block;
    width: 120px;
    height: 64px;
    border: 1px solid var(--edge-2);
}

.bar {
    position: absolute;
    right: 8px;
    bottom: 8px;
    left: 8px;
    height: 8px;
}
</style>
