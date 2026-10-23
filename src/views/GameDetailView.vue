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
import type { GameDetail, Outcome } from "../types/game";
import * as imagesApi from "../api/images";
import { usePasteImages } from "../composables/usePasteImages";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseChip from "../components/ui/BaseChip.vue";

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

onMounted(() => Promise.all([lists.loadRatings(), load()]));

async function change(action: () => Promise<GameDetail>) {
    error.value = null;
    try {
        game.value = await action();
        await library.load();
    } catch (e) {
        error.value = describeError(e);
    }
}

const markOwned = () => change(() => gamesApi.markOwned(props.gameId));
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
            <BaseButton v-if="game.status === 'interested'" variant="primary" @click="markOwned">
                Mark as owned
            </BaseButton>
            <BaseButton @click="editing = true">Edit</BaseButton>
        </header>

        <p v-if="error" class="error">{{ error }}</p>

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

        <div class="block">
            <p>
                {{ isOwned ? "Owned" : "Interested" }}
                <span v-if="game.shelfSlot"> · On the shelf (slot {{ game.shelfSlot }})</span>
            </p>
            <div class="chips">
                <span v-for="platform in game.platforms" :key="platform.id" class="tag">
                    {{ platform.name }}
                </span>
                <span v-for="genre in game.genres" :key="genre.id" class="tag genre">
                    {{ genre.name }}
                </span>
            </div>
        </div>

        <div class="block">
            <h3>Rating</h3>
            <div class="chips">
                <BaseChip :active="game.rating === null" @click="setRating(null)">Unrated</BaseChip>
                <BaseChip v-for="rating in lists.ratings" :key="rating.id" :active="game.rating?.id === rating.id"
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
}

.spacer {
    flex: 1;
}

.gallery {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
    margin-bottom: var(--space-5);
}

.big {
    position: relative;
    width: min(720px, 100%);
    aspect-ratio: 16 / 9;
    border-radius: var(--radius-lg);
    overflow: hidden;
    background: var(--color-hover);
}

.big img {
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
    padding: 2px var(--space-2);
    border-radius: var(--radius-pill);
    background: var(--color-on-image-bg);
    color: var(--color-on-image);
    font-size: var(--font-size-sm);
}

.strip {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
}

.thumb {
    padding: 0;
    border: 2px solid transparent;
    border-radius: var(--radius-md);
    overflow: hidden;
    line-height: 0;
    background: none;
    cursor: pointer;
}

.thumb.selected {
    border-color: var(--color-accent);
}

.thumb img {
    width: 96px;
    height: 54px;
    object-fit: cover;
}

.block {
    margin-bottom: var(--space-5);
}

.block h3 {
    margin: 0 0 var(--space-2);
    font-size: var(--font-size-md);
}

.chips {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
}

.tag {
    padding: 2px 10px;
    border-radius: var(--radius-pill);
    background: var(--color-hover);
    font-size: var(--font-size-sm);
}

.tag.genre {
    background: var(--color-accent-soft);
}

textarea {
    width: 100%;
    resize: vertical;
}

.save-state {
    font-size: var(--font-size-sm);
    font-weight: normal;
    color: var(--color-text-muted);
}
</style>