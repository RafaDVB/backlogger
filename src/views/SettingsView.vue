<script setup lang="ts">
import { onMounted, ref } from "vue";
import ConfirmDeleteDialog from "../components/settings/ConfirmDeleteDialog.vue";
import NamedListEditor from "../components/settings/NamedListEditor.vue";
import { useListsStore } from "../stores/lists";
import { useNavStore } from "../stores/navigation";
import { useSettingsStore } from "../stores/settings";
import type { DeletePreview } from "../types/game";

type Tab = "genres" | "platforms" | "rawg";

const nav = useNavStore();
const lists = useListsStore();
const settings = useSettingsStore();

const tabs: { id: Tab; label: string }[] = [
  { id: "genres", label: "Genres" },
  { id: "platforms", label: "Platforms" },
  { id: "rawg", label: "RAWG" },
];
const tab = ref<Tab>("genres");

const keyDraft = ref("");
const showKey = ref(false);
const keyStatus = ref("");

onMounted(async () => {
  await Promise.all([lists.loadAll(), settings.load()]);
  keyDraft.value = settings.rawgApiKey ?? "";
});

// ---- deleting a genre or platform: preview first, then confirm ----

const pending = ref<{
  kind: "genre" | "platform";
  id: number;
  name: string;
  preview: DeletePreview;
} | null>(null);

async function askDelete(kind: "genre" | "platform", id: number) {
  const items = kind === "genre" ? lists.genres : lists.platforms;
  const name = items.find((item) => item.id === id)?.name ?? "";
  const preview = await lists.previewDelete(kind, id);
  if (preview) pending.value = { kind, id, name, preview };
}

async function confirmDelete() {
  const current = pending.value;
  if (!current) return;
  const done =
    current.kind === "genre"
      ? await lists.deleteGenre(current.id)
      : await lists.deletePlatform(current.id);
  if (done) pending.value = null;
}

function cancelDelete() {
  pending.value = null;
  lists.error = null;
}

// ---- the RAWG key ----

async function saveKey() {
  const saved = await settings.saveRawgApiKey(keyDraft.value);
  keyStatus.value = saved ? "Saved" : "Couldn't save";
  keyDraft.value = settings.rawgApiKey ?? "";
}
</script>

<template>
  <section>
    <header class="top">
      <button type="button" @click="nav.closeSettings()">← Library</button>
      <h2>Settings</h2>
    </header>

    <nav class="tabs">
      <button
        v-for="item in tabs"
        :key="item.id"
        type="button"
        :class="{ active: tab === item.id }"
        @click="tab = item.id"
      >
        {{ item.label }}
      </button>
    </nav>

    <p v-if="lists.error && !pending" class="error">{{ lists.error }}</p>

    <NamedListEditor
      v-if="tab === 'genres'"
      noun="genre"
      :items="lists.genres"
      :create-item="lists.createGenre"
      :rename-item="lists.renameGenre"
      :request-delete="(id) => askDelete('genre', id)"
    />

    <NamedListEditor
      v-else-if="tab === 'platforms'"
      noun="platform"
      :items="lists.platforms"
      :create-item="lists.createPlatform"
      :rename-item="lists.renamePlatform"
      :merge-item="lists.mergePlatform"
      :request-delete="(id) => askDelete('platform', id)"
    />

    <div v-else>
      <h3>RAWG API key</h3>
      <p class="hint">
        A later version will use this to search for games online. RAWG gives out free keys on its
        website. The key is saved unencrypted in this app's local database, and the app only uses
        it to talk to RAWG.
      </p>
      <div class="row">
        <input
          v-model="keyDraft"
          :type="showKey ? 'text' : 'password'"
          placeholder="Paste your key"
        />
        <label><input v-model="showKey" type="checkbox" /> Show</label>
        <button type="button" @click="saveKey">Save</button>
        <span class="hint">{{ keyStatus }}</span>
      </div>
      <p class="hint">To remove the key, empty the box and save.</p>
    </div>

    <ConfirmDeleteDialog
      v-if="pending"
      :kind="pending.kind"
      :name="pending.name"
      :preview="pending.preview"
      :error="lists.error"
      @confirm="confirmDelete"
      @cancel="cancelDelete"
    />
  </section>
</template>

<style scoped>
.top {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}
.top h2 {
  margin: 0;
}
.tabs {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
}
.row {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}
button {
  padding: 6px 14px;
  border: 1px solid rgba(128, 128, 128, 0.5);
  border-radius: 6px;
  background: none;
  color: inherit;
  font: inherit;
  cursor: pointer;
}
.tabs button.active {
  background: rgba(128, 128, 128, 0.25);
  font-weight: 600;
}
input[type="text"],
input[type="password"] {
  min-width: 280px;
  padding: 5px 8px;
  border: 1px solid rgba(128, 128, 128, 0.5);
  border-radius: 6px;
  background: none;
  color: inherit;
  font: inherit;
}
.hint {
  opacity: 0.7;
  font-size: 14px;
}
.error {
  color: crimson;
}
</style>