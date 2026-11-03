<script setup lang="ts">
import { onMounted, ref } from "vue";
import ConfirmDeleteDialog from "../components/settings/ConfirmDeleteDialog.vue";
import NamedListEditor from "../components/settings/NamedListEditor.vue";
import RatingsEditor from "../components/settings/RatingsEditor.vue";
import AppearanceEditor from "../components/settings/AppearanceEditor.vue";
import { useListsStore } from "../stores/lists";
import { useNavStore } from "../stores/navigation";
import { useSettingsStore } from "../stores/settings";
import type { DeletePreview } from "../types/game";
import BaseButton from "../components/ui/BaseButton.vue";
import PixelIcon from "../components/ui/PixelIcon.vue";
import TabBar from "../components/ui/TabBar.vue";

type Tab = "genres" | "platforms" | "ratings" | "appearance" | "rawg";
type ListKind = "genre" | "platform" | "rating";

const nav = useNavStore();
const lists = useListsStore();
const settings = useSettingsStore();

const tabs: { id: Tab; label: string }[] = [
    { id: "genres", label: "Genres" },
    { id: "platforms", label: "Platforms" },
    { id: "ratings", label: "Ratings" },
    { id: "appearance", label: "Appearance" },
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

// ---- deleting a genre, platform or rating: preview first, then confirm ----

const pending = ref<{
    kind: ListKind;
    id: number;
    name: string;
    preview: DeletePreview;
} | null>(null);

function nameOf(kind: ListKind, id: number): string {
    if (kind === "rating") return lists.ratings.find((r) => r.id === id)?.label ?? "";
    const items = kind === "genre" ? lists.genres : lists.platforms;
    return items.find((item) => item.id === id)?.name ?? "";
}

async function askDelete(kind: ListKind, id: number) {
    const preview = await lists.previewDelete(kind, id);
    if (preview) pending.value = { kind, id, name: nameOf(kind, id), preview };
}

async function confirmDelete() {
    const current = pending.value;
    if (!current) return;
    let done: boolean;
    if (current.kind === "genre") done = await lists.deleteGenre(current.id);
    else if (current.kind === "platform") done = await lists.deletePlatform(current.id);
    else done = await lists.deleteRating(current.id);
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
            <BaseButton variant="quiet" @click="nav.closeSettings()">
                <PixelIcon name="arr-left" />
                <span>Library</span>
            </BaseButton>
            <h2>Settings</h2>
        </header>

        <TabBar v-model="tab" :tabs="tabs" class="tab-bar" />

        <p v-if="lists.error && !pending" class="error">{{ lists.error }}</p>

        <NamedListEditor v-if="tab === 'genres'" noun="genre" :items="lists.genres" :create-item="lists.createGenre"
            :rename-item="lists.renameGenre" :request-delete="(id) => askDelete('genre', id)" />

        <NamedListEditor v-else-if="tab === 'platforms'" noun="platform" :items="lists.platforms"
            :create-item="lists.createPlatform" :rename-item="lists.renamePlatform" :merge-item="lists.mergePlatform"
            :request-delete="(id) => askDelete('platform', id)" />

        <RatingsEditor v-else-if="tab === 'ratings'" :ratings="lists.ratings" :create-rating="lists.createRating"
            :update-rating="lists.updateRating" :reorder="lists.reorderRatings"
            :request-delete="(id) => askDelete('rating', id)" />

        <AppearanceEditor v-else-if="tab === 'appearance'" />

        <div v-else-if="tab === 'rawg'">
            <h3>RAWG API key</h3>
            <p class="hint">
                A later version will use this to search for games online. RAWG gives out free keys on its
                website. The key is saved unencrypted in this app's local database, and the app only uses
                it to talk to RAWG.
            </p>
            <div class="row">
                <input v-model="keyDraft" :type="showKey ? 'text' : 'password'" placeholder="Paste your key" />
                <label><input v-model="showKey" type="checkbox" /> Show</label>
                <BaseButton variant="primary" @click="saveKey">Save</BaseButton>
                <span class="hint">{{ keyStatus }}</span>
            </div>
            <p class="hint">To remove the key, empty the box and save.</p>
        </div>

        <ConfirmDeleteDialog v-if="pending" :kind="pending.kind" :name="pending.name" :preview="pending.preview"
            :error="lists.error" @confirm="confirmDelete" @cancel="cancelDelete" />
    </section>
</template>

<style scoped>
.top {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    margin-bottom: var(--space-4);
}

.top h2 {
    margin: 0;
}

.tab-bar {
    margin-bottom: var(--space-4);
}

.row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex-wrap: wrap;
}

input[type="text"],
input[type="password"] {
    min-width: 280px;
}

.row label {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    cursor: pointer;
}
</style>