<script setup lang="ts">
import { ref } from "vue";
import type { Rating } from "../../types/game";
import ColorField from "./ColorField.vue";
import BaseButton from "../ui/BaseButton.vue";
import PixelIcon from "../ui/PixelIcon.vue";

const props = defineProps<{
    ratings: Rating[];
    createRating: (label: string, color: string | null) => Promise<boolean>;
    updateRating: (id: number, label: string, color: string | null) => Promise<boolean>;
    reorder: (ids: number[]) => Promise<boolean>;
    requestDelete: (id: number) => void;
}>();

const newLabel = ref("");
const newColor = ref<string | null>(null);
const editingId = ref<number | null>(null);
const editLabel = ref("");

async function add() {
    if (!newLabel.value.trim()) return;
    if (await props.createRating(newLabel.value, newColor.value)) {
        newLabel.value = "";
        newColor.value = null;
    }
}

function startRename(rating: Rating) {
    editingId.value = rating.id;
    editLabel.value = rating.label;
}

async function saveRename(rating: Rating) {
    if (await props.updateRating(rating.id, editLabel.value, rating.color)) editingId.value = null;
}

function setColor(rating: Rating, color: string | null) {
    props.updateRating(rating.id, rating.label, color);
}

function move(index: number, offset: -1 | 1) {
    const ids = props.ratings.map((rating) => rating.id);
    const target = index + offset;
    [ids[index], ids[target]] = [ids[target], ids[index]]; // swap the two neighbours
    props.reorder(ids);
}
</script>

<template>
    <div>
        <p class="hint">Ratings appear in this order everywhere. Use the arrows to change it.</p>

        <div class="row add">
            <input v-model="newLabel" type="text" placeholder="New rating…" @keydown.enter="add" />
            <ColorField v-model="newColor" />
            <BaseButton variant="primary" @click="add">Add</BaseButton>
        </div>

        <ul class="items">
            <li v-for="(rating, index) in ratings" :key="rating.id">
                <div v-if="editingId === rating.id" class="row">
                    <input v-model="editLabel" type="text" @keydown.enter="saveRename(rating)"
                        @keydown.esc="editingId = null" />
                    <BaseButton size="sm" variant="primary" @click="saveRename(rating)">Save</BaseButton>
                    <BaseButton size="sm" variant="quiet" @click="editingId = null">Cancel</BaseButton>
                </div>

                <div v-else class="row">
                    <BaseButton size="sm" :disabled="index === 0" title="Move up" aria-label="Move up"
                        @click="move(index, -1)">
                        <PixelIcon name="arr-up" />
                    </BaseButton>
                    <BaseButton size="sm" :disabled="index === ratings.length - 1" title="Move down"
                        aria-label="Move down" @click="move(index, 1)">
                        <PixelIcon name="arr-down" />
                    </BaseButton>
                    <span class="name">{{ rating.label }}</span>
                    <ColorField :model-value="rating.color" @update:model-value="setColor(rating, $event)" />
                    <BaseButton size="sm" @click="startRename(rating)">Rename</BaseButton>
                    <BaseButton size="sm" variant="danger" @click="requestDelete(rating.id)">
                        Delete…
                    </BaseButton>
                </div>
            </li>
        </ul>
    </div>
</template>

<style scoped>
.row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex-wrap: wrap;
}

.add {
    margin-bottom: var(--space-4);
}

.items {
    list-style: none;
    margin: 0;
    padding: 0;
    border: 1px solid var(--edge-1);
    background: var(--fill-1);
}

.items li {
    padding: var(--space-2) var(--space-3);
    border-bottom: 1px solid var(--edge-1);
}

.items li:last-child {
    border-bottom: none;
}

.items li:hover {
    background: var(--fill-2);
}

.row input[type="text"] {
    flex: 1;
    min-width: 160px;
    max-width: 420px;
}

.name {
    flex: 1;
    min-width: 140px;
}
</style>