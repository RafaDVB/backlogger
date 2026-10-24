<script setup lang="ts">
import type { GamePlatform, Platform, PlatformState } from "../../types/game";
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

function onPick(event: Event) {
    const select = event.target as HTMLSelectElement;
    const id = Number(select.value);
    select.value = "";
    if (id) emit("set", id, props.newState ?? "wanted");
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

        <select v-if="available.length" value="" @change="onPick">
            <option value="" disabled>+ Add platform…</option>
            <option v-for="platform in available" :key="platform.id" :value="String(platform.id)">
                {{ platform.name }}
            </option>
        </select>
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
    align-items: center;
}

.toggle {
    padding: 0;
    border: none;
    background: none;
    font: inherit;
    cursor: pointer;
}

.remove {
    padding: 0 var(--space-1);
    border: none;
    background: none;
    color: var(--color-text-muted);
    cursor: pointer;
}

.remove:hover {
    color: var(--color-danger);
}
</style>