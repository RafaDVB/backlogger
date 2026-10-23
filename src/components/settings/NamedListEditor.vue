<script setup lang="ts">
import { computed, ref } from "vue";

interface Item {
  id: number;
  name: string;
}

const props = defineProps<{
  items: Item[];
  noun: string;
  createItem: (name: string) => Promise<boolean>;
  renameItem: (id: number, name: string) => Promise<boolean>;
  requestDelete: (id: number) => void;
  mergeItem?: (fromId: number, toId: number) => Promise<boolean>;
}>();

const newName = ref("");
const editingId = ref<number | null>(null);
const editName = ref("");
const mergingId = ref<number | null>(null);
const mergeTarget = ref<number | null>(null);

const mergeChoices = computed(() => props.items.filter((item) => item.id !== mergingId.value));

async function add() {
  if (!newName.value.trim()) return;
  if (await props.createItem(newName.value)) newName.value = "";
}

function startRename(item: Item) {
  editingId.value = item.id;
  editName.value = item.name;
  mergingId.value = null;
}

async function saveRename() {
  if (editingId.value === null) return;
  if (await props.renameItem(editingId.value, editName.value)) editingId.value = null;
}

function startMerge(item: Item) {
  mergingId.value = item.id;
  mergeTarget.value = null;
  editingId.value = null;
}

async function saveMerge() {
  if (mergingId.value === null || mergeTarget.value === null || !props.mergeItem) return;
  if (await props.mergeItem(mergingId.value, mergeTarget.value)) mergingId.value = null;
}
</script>

<template>
  <div>
    <div class="row add">
      <input
        v-model="newName"
        type="text"
        :placeholder="`New ${noun}…`"
        @keydown.enter="add"
      />
      <button type="button" @click="add">Add</button>
    </div>

    <ul class="items">
      <li v-for="item in items" :key="item.id">
        <div v-if="editingId === item.id" class="row">
          <input
            v-model="editName"
            type="text"
            @keydown.enter="saveRename"
            @keydown.esc="editingId = null"
          />
          <button type="button" @click="saveRename">Save</button>
          <button type="button" @click="editingId = null">Cancel</button>
        </div>

        <div v-else-if="mergingId === item.id" class="row">
          <span>Move every game from "{{ item.name }}" to</span>
          <select v-model="mergeTarget">
            <option :value="null" disabled>choose…</option>
            <option v-for="choice in mergeChoices" :key="choice.id" :value="choice.id">
              {{ choice.name }}
            </option>
          </select>
          <button type="button" :disabled="mergeTarget === null" @click="saveMerge">
            Merge
          </button>
          <button type="button" @click="mergingId = null">Cancel</button>
        </div>

        <div v-else class="row">
          <span class="name">{{ item.name }}</span>
          <button type="button" @click="startRename(item)">Rename</button>
          <button v-if="mergeItem" type="button" @click="startMerge(item)">Merge…</button>
          <button type="button" class="danger" @click="requestDelete(item.id)">Delete…</button>
        </div>
      </li>
    </ul>
    <p v-if="items.length === 0" class="hint">No {{ noun }}s yet.</p>
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
input[type="text"],
select {
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
}
</style>