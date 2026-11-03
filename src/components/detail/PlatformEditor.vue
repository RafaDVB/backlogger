<script setup lang="ts">
import { computed } from "vue";
import type { GamePlatform, Platform, PlatformState } from "../../types/game";
import BaseSelect from "../ui/BaseSelect.vue";
import PlatformTag from "../ui/PlatformTag.vue";

const props = defineProps<{
    platforms: GamePlatform[];
    available: Platform[];
    newState?: PlatformState;
}>();
const emit = defineEmits<{ set: [platformId: number, state: PlatformState | null] }>();

function hint(platform: GamePlatform): string {
    return platform.state === "owned"
        ? `Owned on ${platform.name}. Click to mark as wanted.`
        : `Wanted on ${platform.name}. Click to mark as owned.`;
}

const availableOptions = computed(() => props.available.map((p) => ({ value: p.id, label: p.name })));

function onAdd(id: number) {
    emit("set", id, props.newState ?? "wanted");
}
</script>

<template>
    <div class="editor">
        <span v-for="platform in platforms" :key="platform.id" class="item">
            <button type="button" class="toggle"
                @click="emit('set', platform.id, platform.state === 'owned' ? 'wanted' : 'owned')">
                <PlatformTag :name="platform.name" :state="platform.state" :hint="hint(platform)" />
            </button>
            <button type="button" class="remove" :title="`Remove ${platform.name}`"
                :aria-label="`Remove ${platform.name}`" @click="emit('set', platform.id, null)">
                ×
            </button>
        </span>

        <BaseSelect v-if="available.length" :model-value="null" :options="availableOptions"
            placeholder="+ Add platform…" @update:model-value="onAdd" />
        <span v-if="platforms.length === 0 && available.length === 0" class="hint">
            No platforms yet. Add some in Settings.
        </span>
    </div>
</template>

<style scoped>
.editor {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-2);
    margin-bottom: var(--space-2);
}

.item {
    display: inline-flex;
    align-items: stretch;
}

.toggle {
    display: flex;
    padding: 0;
    border: none;
    background: none;
    font: inherit;
    cursor: pointer;
}

.toggle:hover :deep(.tag),
.toggle:focus-visible :deep(.tag) {
    border-color: var(--edge-3);
}

.remove {
    position: relative;
    margin-left: 0px;
    padding: 0 var(--space-1);
    font: inherit;
    color: var(--color-text-muted);
    background: var(--fill-1);
    border: 1px solid var(--edge-1);
    border-radius: var(--radius-sm);
    cursor: pointer;
}

.remove:hover,
.remove:focus-visible {
    z-index: 1;
    color: var(--color-danger);
    border-color: var(--color-danger);
    background: color-mix(in srgb, var(--color-danger) 25%, transparent);
}
</style>
