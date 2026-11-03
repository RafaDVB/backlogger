<script setup lang="ts">
import { computed, ref } from "vue";
import BaseButton from "../ui/BaseButton.vue";
import BaseSelect from "../ui/BaseSelect.vue";

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
const mergeOptions = computed(() => mergeChoices.value.map((item) => ({ value: item.id, label: item.name })));

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
      <input v-model="newName" type="text" :placeholder="`New ${noun}…`" @keydown.enter="add" />
      <BaseButton variant="primary" @click="add">Add</BaseButton>
    </div>

    <ul class="items">
      <li v-for="item in items" :key="item.id">
        <div v-if="editingId === item.id" class="row">
          <input v-model="editName" type="text" @keydown.enter="saveRename" @keydown.esc="editingId = null" />
          <BaseButton size="sm" variant="primary" @click="saveRename">Save</BaseButton>
          <BaseButton size="sm" variant="quiet" @click="editingId = null">Cancel</BaseButton>
        </div>

        <div v-else-if="mergingId === item.id" class="row">
          <span>Move every game from "{{ item.name }}" to</span>
          <BaseSelect v-model="mergeTarget" :options="mergeOptions" placeholder="choose…" />
          <BaseButton size="sm" variant="primary" :disabled="mergeTarget === null" @click="saveMerge">
            Merge
          </BaseButton>
          <BaseButton size="sm" variant="quiet" @click="mergingId = null">Cancel</BaseButton>
        </div>

        <div v-else class="row">
          <span class="name">{{ item.name }}</span>
          <BaseButton size="sm" @click="startRename(item)">Rename</BaseButton>
          <BaseButton v-if="mergeItem" size="sm" @click="startMerge(item)">Merge…</BaseButton>
          <BaseButton size="sm" variant="danger" @click="requestDelete(item.id)">Delete…</BaseButton>
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