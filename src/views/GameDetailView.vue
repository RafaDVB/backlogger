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
            <button type="button" @click="nav.close()">← Library</button>
            <h2>
                {{ game.title }}<span v-if="game.releaseYear"> ({{ game.releaseYear }})</span>
            </h2>
            <div class="spacer" />
            <button v-if="game.status === 'interested'" type="button" @click="markOwned">
                Mark as owned
            </button>
            <button type="button" @click="editing = true">Edit</button>
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
            <p class="hint">Tip: copy an image and press Ctrl+V on this screen to add it as a screenshot.</p>
            <button v-if="current && current.id !== featuredId" type="button" @click="setFeatured">
                Use as featured screenshot
            </button>
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
                <button type="button" class="chip" :class="{ active: game.rating === null }" @click="setRating(null)">
                    Unrated
                </button>
                <button v-for="rating in lists.ratings" :key="rating.id"
                    :style="rating.color ? { borderLeft: '6px solid ' + rating.color } : undefined" type="button"
                    class="chip" :class="{ active: game.rating?.id === rating.id }" @click="setRating(rating.id)">
                    {{ rating.label }}
                </button>
            </div>
        </div>

        <div class="block">
            <h3>Outcome</h3>
            <div class="chips">
                <button v-for="option in outcomes" :key="option.label" type="button" class="chip"
                    :class="{ active: game.outcome === option.value }" :disabled="!isOwned"
                    @click="setOutcome(option.value)">
                    {{ option.label }}
                </button>
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
    gap: 12px;
    margin-bottom: 16px;
}

.top h2 {
    margin: 0;
}

.spacer {
    flex: 1;
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

button:hover:not(:disabled) {
    background: rgba(128, 128, 128, 0.2);
}

button:disabled {
    opacity: 0.4;
    cursor: default;
}

.gallery {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 10px;
    margin-bottom: 20px;
}

.big {
    position: relative;
    width: min(720px, 100%);
    aspect-ratio: 16 / 9;
    border-radius: 8px;
    overflow: hidden;
    background: rgba(128, 128, 128, 0.2);
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
    opacity: 0.6;
}

.badge {
    position: absolute;
    top: 8px;
    left: 8px;
    padding: 2px 8px;
    border-radius: 999px;
    background: rgba(0, 0, 0, 0.7);
    color: white;
    font-size: 12px;
}

.strip {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
}

.thumb {
    padding: 0;
    border-width: 2px;
    overflow: hidden;
    line-height: 0;
}

.thumb.selected {
    border-color: seagreen;
}

.thumb img {
    width: 96px;
    height: 54px;
    object-fit: cover;
}

.block {
    margin-bottom: 20px;
}

.block h3 {
    margin: 0 0 8px;
    font-size: 15px;
}

.chips {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
}

.chip.active {
    background: rgba(128, 128, 128, 0.3);
    font-weight: 600;
    border-color: seagreen;
}

.tag {
    padding: 2px 10px;
    border-radius: 999px;
    background: rgba(128, 128, 128, 0.2);
    font-size: 13px;
}

.tag.genre {
    background: rgba(46, 139, 87, 0.25);
}

textarea {
    width: 100%;
    box-sizing: border-box;
    padding: 8px;
    border: 1px solid rgba(128, 128, 128, 0.5);
    border-radius: 6px;
    background: none;
    color: inherit;
    font: inherit;
    resize: vertical;
}

.save-state,
.hint {
    font-size: 13px;
    font-weight: normal;
    opacity: 0.7;
}

.error {
    color: crimson;
}
</style>