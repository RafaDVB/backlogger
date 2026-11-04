<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { openUrl } from "@tauri-apps/plugin-opener";
import packageJson from "../../package.json";
import ConfirmDeleteDialog from "../components/settings/ConfirmDeleteDialog.vue";
import NamedListEditor from "../components/settings/NamedListEditor.vue";
import RatingsEditor from "../components/settings/RatingsEditor.vue";
import AppearanceEditor from "../components/settings/AppearanceEditor.vue";
import { useListsStore } from "../stores/lists";
import { useNavStore } from "../stores/navigation";
import { useSettingsStore } from "../stores/settings";
import type { DeletePreview } from "../types/game";
import { describeError } from "../api/errors";
import { getLegalInfo, type LegalInfo } from "../api/system";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseDialog from "../components/ui/BaseDialog.vue";
import PixelIcon from "../components/ui/PixelIcon.vue";
import TabBar from "../components/ui/TabBar.vue";

type Tab = "genres" | "platforms" | "ratings" | "appearance" | "rawg" | "about";
type ListKind = "genre" | "platform" | "rating";
type LegalView = "license" | "third-party";

const nav = useNavStore();
const lists = useListsStore();
const settings = useSettingsStore();

const tabs: { id: Tab; label: string }[] = [
    { id: "genres", label: "Genres" },
    { id: "platforms", label: "Platforms" },
    { id: "ratings", label: "Ratings" },
    { id: "appearance", label: "Appearance" },
    { id: "rawg", label: "RAWG" },
    { id: "about", label: "About" },
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

// ---- legal notices ----

const legalView = ref<LegalView | null>(null);
const legal = ref<LegalInfo | null>(null);
const legalLoading = ref(false);
const legalError = ref("");

const legalTitle = computed(() => (legalView.value === "third-party" ? "Third-party notices" : "License"));

const legalText = computed(() => {
    if (legalError.value) return legalError.value;
    if (legalLoading.value || !legal.value) return "Loading...";
    return legalView.value === "third-party" ? legal.value.thirdParty : legal.value.license;
});

async function openLegal(view: LegalView) {
    legalView.value = view;
    if (legal.value || legalLoading.value) return;
    legalLoading.value = true;
    legalError.value = "";
    try {
        legal.value = await getLegalInfo();
    } catch (e) {
        legalError.value = describeError(e);
    } finally {
        legalLoading.value = false;
    }
}

function openRepository() {
    void openUrl("https://github.com/RafaDVB/backlogger").catch((e) => {
        console.error("Couldn't open the repository", e);
    });
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

        <div v-else-if="tab === 'about'" class="about">
            <h3>Backlogger {{ packageJson.version }}</h3>
            <p class="hint">A local-first video game backlog and shelf tracker.</p>
            <p>
                Copyright (C) 2026 RafaDVB. Backlogger is free software: you can redistribute it and/or
                modify it under the terms of the GNU General Public License as published by the Free
                Software Foundation, either version 3 of the License, or (at your option) any later
                version.
            </p>
            <p class="hint">
                This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
                without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
                See the GNU General Public License for more details.
            </p>
            <div class="row">
                <BaseButton @click="openLegal('license')">License</BaseButton>
                <BaseButton @click="openLegal('third-party')">Third-party notices</BaseButton>
                <BaseButton variant="quiet" @click="openRepository">Source code</BaseButton>
            </div>
        </div>

        <ConfirmDeleteDialog v-if="pending" :kind="pending.kind" :name="pending.name" :preview="pending.preview"
            :error="lists.error" @confirm="confirmDelete" @cancel="cancelDelete" />

        <BaseDialog v-if="legalView" :title="legalTitle" width="820px" @close="legalView = null">
            <pre class="legal-text">{{ legalText }}</pre>
            <template #footer>
                <BaseButton variant="primary" @click="legalView = null">Close</BaseButton>
            </template>
        </BaseDialog>
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

.about {
    max-width: 720px;
}

.about p {
    line-height: 1.5;
}

.legal-text {
    max-height: 55vh;
    margin: 0;
    padding: var(--space-3);
    overflow: auto;
    border: 1px solid var(--edge-1);
    background: var(--fill-1);
    font-family: ui-monospace, "Courier New", monospace;
    font-size: 12px;
    line-height: 1.4;
    white-space: pre-wrap;
    overflow-wrap: anywhere;
}
</style>
