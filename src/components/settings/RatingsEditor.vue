<script setup lang="ts">
import { ref } from "vue";
import type { Rating } from "../../types/game";
import ColorField from "./ColorField.vue";

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
            <button type="button" @click="add">Add</button>
        </div>

        <ul class="items">
            <li v-for="(rating, index) in ratings" :key="rating.id">
                <div v-if="editingId === rating.id" class="row">
                    <input v-model="editLabel" type="text" @keydown.enter="saveRename(rating)"
                        @keydown.esc="editingId = null" />
                    <button type="button" @click="saveRename(rating)">Save</button>
                    <button type="button" @click="editingId = null">Cancel</button>
                </div>

                <div v-else class="row">
                    <button type="button" :disabled="index === 0" @click="move(index, -1)">↑</button>
                    <button type="button" :disabled="index === ratings.length - 1" @click="move(index, 1)">
                        ↓
                    </button>
                    <span class="name">{{ rating.label }}</span>
                    <ColorField :model-value="rating.color" @update:model-value="setColor(rating, $event)" />
                    <button type="button" @click="startRename(rating)">Rename</button>
                    <button type="button" class="danger" @click="requestDelete(rating.id)">Delete…</button>
                </div>
            </li>
        </ul>
    </div>
</template>

<style scoped>
.row {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
}

.add {
    margin-bottom: 14px;
}

.items {
    list-style: none;
    margin: 0;
    padding: 0;
}

.items li {
    padding: 6px 0;
    border-bottom: 1px solid rgba(128, 128, 128, 0.25);
}

.name {
    flex: 1;
    min-width: 140px;
}

input[type="text"] {
    padding: 5px 8px;
    border: 1px solid rgba(128, 128, 128, 0.5);
    border-radius: 6px;
    background: none;
    color: inherit;
    font: inherit;
}

button {
    padding: 4px 12px;
    border: 1px solid rgba(128, 128, 128, 0.5);
    border-radius: 6px;
    background: none;
    color: inherit;
    font: inherit;
    cursor: pointer;
}

button:disabled {
    opacity: 0.4;
    cursor: default;
}

.danger {
    border-color: crimson;
    color: crimson;
}

.hint {
    opacity: 0.7;
    font-size: 14px;
}
</style>