<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { describeError, errorCode } from "../api/errors";
import * as gamesApi from "../api/games";
import GameFormDialog from "../components/dialogs/GameFormDialog.vue";
import { useAutosave } from "../composables/useAutosave";
import { resolveImage } from "../lib/images";
import { useLibraryStore } from "../stores/library";
import { useListsStore } from "../stores/lists";
import { useNavStore } from "../stores/navigation";
import type { GameDetail, Outcome, PlatformState } from "../types/game";
import * as imagesApi from "../api/images";
import { usePasteImages } from "../composables/usePasteImages";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseChip from "../components/ui/BaseChip.vue";
import MarkOwnedDialog from "../components/dialogs/MarkOwnedDialog.vue";
import PlatformEditor from "../components/detail/PlatformEditor.vue";

const props = defineProps<{ gameId: number }>();

const nav = useNavStore();
const library = useLibraryStore();
const lists = useListsStore();

const game = ref<GameDetail | null>(null);
const error = ref<string | null>(null);
const editing = ref(false);
const selectedIndex = ref(0);

const notes = ref("");
const autosave = useAutosave(notes, (value) => gamesApi.setNotes(props.gameId, value));

const outcomes: { value: Outcome | null; label: string }[] = [
    { value: null, label: "None" },
    { value: "completed", label: "Completed" },
    { value: "dropped", label: "Dropped" },
];

const shots = computed(() => game.value?.screenshots ?? []);
const current = computed(() => shots.value[selectedIndex.value] ?? null);
const featuredId = computed(() => game.value?.featuredScreenshotId ?? shots.value[0]?.id ?? null);
const bigImage = computed(() =>
    resolveImage(current.value?.path ?? game.value?.coverPath ?? null),
);
const isOwned = computed(() => game.value?.status === "owned");

const saveLabel = computed(
    () =>
        ({
            idle: "",
            pending: "Unsaved changes…",
            saving: "Saving…",
            saved: "Saved",
            error: "Couldn't save",
        })[autosave.state.value],
);

async function load() {
    try {
        const loaded = await gamesApi.getGame(props.gameId);
        game.value = loaded;
        autosave.reset(loaded.notes);
        if (selectedIndex.value >= loaded.screenshots.length) selectedIndex.value = 0;
    } catch (e) {
        if (errorCode(e) === "not_found") nav.close();
        else error.value = describeError(e);
    }
}

onMounted(() => Promise.all([lists.loadAll(), load()]));

async function change(action: () => Promise<GameDetail>): Promise<boolean> {
    error.value = null;
    try {
        game.value = await action();
        await library.load();
        return true;
    } catch (e) {
        error.value = describeError(e);
        return false;
    }
}

const markingOwned = ref(false);

async function confirmMarkOwned(platformId: number) {
    if (await change(() => gamesApi.markOwned(props.gameId, platformId))) markingOwned.value = false;
}

const setPlatform = (platformId: number, state: PlatformState | null) =>
    change(() => gamesApi.setPlatformState(props.gameId, platformId, state));

const availablePlatforms = computed(() =>
    lists.platforms.filter((p) => !game.value?.platforms.some((g) => g.id === p.id)),
);

const setRating = (id: number | null) => change(() => gamesApi.setRating(props.gameId, id));
const setOutcome = (outcome: Outcome | null) =>
    change(() => gamesApi.setOutcome(props.gameId, outcome));

function setFeatured() {
    const shot = current.value;
    if (shot) change(() => gamesApi.setFeaturedScreenshot(props.gameId, shot.id));
}

async function onFormClosed() {
    editing.value = false;
    await autosave.flush();
    await load();
}

usePasteImages(async (images) => {
    if (editing.value) return;
    error.value = null;
    try {
        for (const image of images) {
            await imagesApi.addScreenshotFromBytes(props.gameId, image.bytes, image.extension);
        }
        await autosave.flush();
        await load();
        await library.load();
        selectedIndex.value = Math.max(shots.value.length - 1, 0);
    } catch (e) {
        error.value = describeError(e);
    }
});
</script>

<template>
    <section v-if="game" class="detail">
        <header class="top">
            <BaseButton variant="quiet" @click="nav.close()">← Library</BaseButton>
            <h2>
                {{ game.title }}<span v-if="game.releaseYear"> ({{ game.releaseYear }})</span>
            </h2>
            <div class="spacer" />
            <BaseButton v-if="game.status === 'interested'" variant="primary" @click="markingOwned = true">
                Mark as owned
            </BaseButton>
            <BaseButton @click="editing = true">Edit</BaseButton>
        </header>

        <p v-if="error" class="error">{{ error }}</p>

        <div class="layout">
            <div class="gallery">
                <div class="big">
                    <img v-if="bigImage" :src="bigImage" :alt="game.title" />
                    <div v-else class="no-image">No images yet</div>
                    <span v-if="current && current.id === featuredId" class="badge">Featured</span>
                </div>
                <div class="strip">
                    <button v-for="(shot, index) in shots" :key="shot.id" type="button" class="thumb"
                        :class="{ selected: index === selectedIndex }" @click="selectedIndex = index">
                        <img :src="resolveImage(shot.path) ?? undefined" alt="" />
                    </button>
                </div>
                <BaseButton v-if="current && current.id !== featuredId" size="sm" @click="setFeatured">
                    Use as featured screenshot
                </BaseButton>
                <p class="hint">Tip: copy an image and press Ctrl+V on this screen to add it as a screenshot.</p>
            </div>

            <div class="info">
                <div class="block">
                    <p>
                        {{ isOwned ? "Owned" : "Interested" }}
                        <span v-if="game.shelfSlot"> · On the shelf (slot {{ game.shelfSlot }})</span>
                    </p>
                    <PlatformEditor :platforms="game.platforms" :available="availablePlatforms" @set="setPlatform" />
                    <div v-if="game.genres.length" class="chips genres">
                        <span v-for="genre in game.genres" :key="genre.id" class="tag genre">
                            {{ genre.name }}
                        </span>
                    </div>
                </div>

                <div class="block">
                    <h3>Rating</h3>
                    <div class="chips">
                        <BaseChip :active="game.rating === null" @click="setRating(null)">Unrated</BaseChip>
                        <BaseChip v-for="rating in lists.ratings" :key="rating.id"
                            :active="game.rating?.id === rating.id"
                            :style="rating.color ? { borderLeft: '6px solid ' + rating.color } : undefined"
                            @click="setRating(rating.id)">
                            {{ rating.label }}
                        </BaseChip>
                    </div>
                </div>

                <div class="block">
                    <h3>Outcome</h3>
                    <div class="chips">
                        <BaseChip v-for="option in outcomes" :key="option.label" :active="game.outcome === option.value"
                            :disabled="!isOwned" @click="setOutcome(option.value)">
                            {{ option.label }}
                        </BaseChip>
                    </div>
                    <p v-if="!isOwned" class="hint">Mark the game as owned to track whether you finished it.</p>
                </div>

                <div class="block">
                    <h3>
                        Notes <span class="save-state">{{ saveLabel }}</span>
                    </h3>
                    <textarea v-model="notes" rows="8" placeholder="Where you left off, passwords, thoughts…" />
                </div>
            </div>
        </div>

        <MarkOwnedDialog v-if="markingOwned" :title="game.title" :linked="game.platforms" :error="error"
            @confirm="confirmMarkOwned" @cancel="markingOwned = false" />
        <GameFormDialog v-if="editing" :game-id="gameId" :default-status="game.status" @close="onFormClosed" />
    </section>

    <p v-else-if="error" class="error">{{ error }}</p>
    <p v-else>Loading…</p>
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
    font-size: var(--font-size-lg);
}

.spacer {
    flex: 1;
}

.layout {
    display: grid;
    grid-template-columns: minmax(0, 720px) minmax(280px, 1fr);
    gap: var(--space-4);
    align-items: start;
}

@media (max-width: 1100px) {
    .layout {
        grid-template-columns: 1fr;
    }
}

.gallery {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
}

.big {
    position: relative;
    width: 100%;
    aspect-ratio: 16 / 9;
    overflow: hidden;
    border: 1px solid var(--edge-2);
    background: var(--fill-1);
}

.big img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: contain;
}

.no-image {
    display: grid;
    place-items: center;
    height: 100%;
    color: var(--color-text-muted);
}

.badge {
    position: absolute;
    top: var(--space-2);
    left: var(--space-2);
    padding: 0 var(--space-2);
    border: 1px solid var(--edge-2);
    background: var(--color-on-image-bg);
    color: var(--color-on-image);
}

.strip {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
}

.thumb {
    padding: 0;
    border: 1px solid var(--edge-1);
    border-radius: var(--radius-sm);
    overflow: hidden;
    line-height: 0;
    background: var(--fill-1);
    cursor: pointer;
    opacity: 0.7;
}

.thumb:hover,
.thumb:focus-visible {
    border-color: var(--edge-3);
    opacity: 1;
}

.thumb.selected {
    border-color: var(--edge-3);
    box-shadow: 0 0 0 1px var(--edge-3);
    opacity: 1;
}

.thumb img {
    width: 96px;
    height: 54px;
    object-fit: cover;
}

.info {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
}

.block {
    padding: var(--space-3);
    border: 1px solid var(--edge-1);
    background: var(--fill-1);
}

.block p {
    margin: 0 0 var(--space-2);
}

.block h3 {
    margin: 0 0 var(--space-2);
    padding-bottom: var(--space-1);
    border-bottom: 1px solid var(--edge-1);
    font-size: var(--font-size-md);
    color: var(--color-text-muted);
}

.chips {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
}

.genres {
    margin-top: var(--space-2);
}

.tag {
    padding: 0 var(--space-1);
    border: 1px solid var(--edge-1);
    background: var(--fill-1);
    color: var(--color-text-muted);
}

.tag.genre {
    border-color: var(--edge-2);
    background: var(--fill-2);
    color: var(--color-text);
}

textarea {
    width: 100%;
    resize: vertical;
}

.save-state {
    color: var(--color-text-muted);
}
</style>